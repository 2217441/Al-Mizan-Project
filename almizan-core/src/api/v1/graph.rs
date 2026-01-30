use crate::api::v1::utils::{format_surreal_id, serialize_thing_id};
use crate::repository::db::Database;
use axum::{
    extract::State,
    http::header,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;
use tracing::info;

#[derive(Serialize)]
pub struct GraphData<'a> {
    nodes: Vec<CytoscapeNode<'a>>,
    edges: Vec<CytoscapeEdge<'a>>,
}

#[derive(Serialize)]
struct CytoscapeNode<'a> {
    data: NodeData<'a>,
}

#[derive(Serialize)]
struct NodeData<'a> {
    id: GraphId<'a>,
    label: Cow<'a, str>,
    #[serde(rename = "type")]
    node_type: Cow<'a, str>,
}

#[derive(Serialize)]
struct CytoscapeEdge<'a> {
    data: EdgeData<'a>,
}

#[derive(Serialize)]
struct EdgeData<'a> {
    id: GraphId<'a>,
    source: GraphId<'a>,
    target: GraphId<'a>,
    label: Cow<'a, str>,
}

enum GraphId<'a> {
    Thing(&'a surrealdb::sql::Thing),
    Str(Cow<'a, str>),
    Owned(String),
}

impl<'a> Serialize for GraphId<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GraphId::Thing(t) => serialize_thing_id(t, serializer),
            GraphId::Str(s) => serializer.serialize_str(s),
            GraphId::Owned(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Deserialize, Debug)]
struct DbVerse {
    id: surrealdb::sql::Thing,
    surah_number: i32,
    ayah_number: i32,
}

#[derive(Deserialize, Debug)]
struct DbProphet {
    id: surrealdb::sql::Thing,
    name_ar: String,
}

#[derive(Deserialize, Debug)]
struct DbSemanticHadith {
    id: surrealdb::sql::Thing,
    ref_no: i32,
    collection: String,
}

#[derive(Deserialize, Debug)]
struct DbNarrator {
    id: surrealdb::sql::Thing,
    name_ar: Option<String>,
    generation: Option<i32>,
}

/// GET /api/v1/graph
/// Returns the tawhidic knowledge graph showing epistemological chains.
/// Shows: Allah → Prophets → Quran Verses (via `chosen_by` and `narrated_quran` edges)
#[allow(clippy::too_many_lines)]
pub async fn get_graph(State(db): State<Database>) -> impl IntoResponse {
    // Optimization: Pre-allocate vectors to avoid reallocations.
    // Estimated count: 1 (Allah) + 25 (Prophets) + 20 (Verses) + 30 (Narrators) + 50 (Hadiths) = ~126
    let mut nodes: Vec<CytoscapeNode> = Vec::with_capacity(130);
    // Estimated edges: 25 (Prophets) + 20 (Verses) + 30 (Narrators) + 50 (Hadiths) = ~125
    let mut edges_vec: Vec<CytoscapeEdge> = Vec::with_capacity(130);

    // 1. Add Allah (the root)
    nodes.push(CytoscapeNode {
        data: NodeData {
            id: GraphId::Str(Cow::Borrowed("allah:tawhid")),
            label: Cow::Borrowed("الله"),
            node_type: Cow::Borrowed("allah"),
        },
    });

    // 2. Prepare Queries
    let prophets_sql = "SELECT id, name_ar FROM prophet LIMIT 25";
    let verses_sql = "SELECT id, surah_number, ayah_number FROM quran_verse LIMIT 20";
    // Optimization: Removed display_text from query as it is unused
    let hadith_sql = "SELECT id, ref_no, collection FROM semantic_hadith LIMIT 50";
    let narrator_sql = "SELECT id, name_ar, generation FROM narrator LIMIT 30";

    // 3. Execute concurrently
    // Parallelize 4 independent DB queries to reduce total latency
    let (prophets_res, verses_res, hadith_res, narrators_res) = tokio::join!(
        db.client.query(prophets_sql),
        db.client.query(verses_sql),
        db.client.query(hadith_sql),
        db.client.query(narrator_sql)
    );

    // 4. Unpack Results with Error Logging
    let prophets: Vec<DbProphet> = match prophets_res {
        Ok(mut response) => match response.take(0) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to deserialize prophets: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query prophets: {}", e);
            Vec::new()
        }
    };

    let verses: Vec<DbVerse> = match verses_res {
        Ok(mut response) => match response.take(0) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to deserialize verses: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query verses: {}", e);
            Vec::new()
        }
    };

    let hadiths: Vec<DbSemanticHadith> = match hadith_res {
        Ok(mut response) => match response.take(0) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to deserialize hadiths: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query hadiths: {}", e);
            Vec::new()
        }
    };

    let narrators_list: Vec<DbNarrator> = match narrators_res {
        Ok(mut response) => match response.take(0) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to deserialize narrators: {}", e);
                Vec::new()
            }
        },
        Err(e) => {
            tracing::error!("Failed to query narrators: {}", e);
            Vec::new()
        }
    };

    // Capture lengths for logging before consuming vectors
    let prophets_len = prophets.len();
    let verses_len = verses.len();
    let hadiths_len = hadiths.len();
    let narrators_len = narrators_list.len();

    // 5. Process Prophets
    // Optimization: Borrow data instead of cloning strings.
    for prophet in &prophets {
        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::Thing(&prophet.id),
                label: Cow::Borrowed(&prophet.name_ar),
                node_type: Cow::Borrowed("prophet"),
            },
        });

        // Edge: Allah -> Prophet (chosen_by)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                // We still format edge ID as string because it's a composite ID
                id: GraphId::Owned(format!("chosen_{}", format_surreal_id(&prophet.id))),
                source: GraphId::Str(Cow::Borrowed("allah:tawhid")),
                target: GraphId::Thing(&prophet.id),
                label: Cow::Borrowed("chose"),
            },
        });
    }

    // 3. Get sample verses narrated by Prophet Muhammad (using available Juz 30 data)
    for verse in &verses {
        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::Thing(&verse.id),
                label: Cow::Owned(format!("{}:{}", verse.surah_number, verse.ayah_number)),
                node_type: Cow::Borrowed("verse"),
            },
        });

        // Edge: Prophet Muhammad -> Verse (narrated)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: GraphId::Owned(format!("narrated_{}", format_surreal_id(&verse.id))),
                source: GraphId::Str(Cow::Borrowed("prophet:muhammad")),
                target: GraphId::Thing(&verse.id),
                label: Cow::Borrowed("narrated"),
            },
        });
    }

    // 4. Get Top Narrators
    // Collect narrator IDs for round-robin distribution without full formatting
    let narrator_ids: Vec<&surrealdb::sql::Thing> = narrators_list.iter().map(|n| &n.id).collect();

    for narrator in &narrators_list {
        let label_str = narrator.name_ar.as_deref().unwrap_or("راوي");
        // Optimization: Find byte index for 15th char to slice, avoiding intermediate String allocation
        let end = label_str.char_indices().map(|(i, _)| i).nth(15).unwrap_or(label_str.len());
        let gen = narrator.generation.unwrap_or(0);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::Thing(&narrator.id),
                label: Cow::Owned(format!("{} (ط{})", &label_str[..end], gen)),
                node_type: Cow::Borrowed("narrator"),
            },
        });

        // Edge: Prophet -> Narrator (Taught)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: GraphId::Owned(format!("taught_{}", format_surreal_id(&narrator.id))),
                source: GraphId::Str(Cow::Borrowed("prophet:muhammad")),
                target: GraphId::Thing(&narrator.id),
                label: Cow::Borrowed("taught"),
            },
        });
    }

    // 5. Get Hadiths
    for (i, hadith) in hadiths.iter().enumerate() {
        // Use Arabic collection names for labels
        let collection_label = match hadith.collection.as_str() {
            "bukhari" => "بخاري",
            "muslim" => "مسلم",
            "nasa'i" => "نسائي",
            "ibn_majah" => "ابن ماجه",
            "tirmidhi" => "ترمذي",
            "abu_dawud" => "أبو داود",
            _ => "حديث",
        };

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::Thing(&hadith.id),
                label: Cow::Owned(format!("{} {}", collection_label, hadith.ref_no)),
                node_type: Cow::Borrowed("hadith"),
            },
        });

        // Link Narrators -> Hadiths (Round Robin Distribution)
        if !narrator_ids.is_empty() {
            // Assign to a narrator based on index
            let narrator_id = narrator_ids[i % narrator_ids.len()];

            edges_vec.push(CytoscapeEdge {
                data: EdgeData {
                    id: GraphId::Owned(format!("narrated_{}_{}", format_surreal_id(narrator_id), format_surreal_id(&hadith.id))),
                    source: GraphId::Thing(narrator_id),
                    target: GraphId::Thing(&hadith.id),
                    label: Cow::Borrowed("narrated"),
                },
            });
        }
    }

    info!(
        "[TAWHIDIC GRAPH] Allah: 1, Prophets: {}, Verses: {}, Hadiths: {}, Narrators: {}, Edges: {}",
        prophets_len,
        verses_len,
        hadiths_len,
        narrators_len,
        edges_vec.len()
    );

    // Manually serialize to string to handle lifetimes and references
    match serde_json::to_string(&GraphData {
        nodes,
        edges: edges_vec,
    }) {
        Ok(json) => ([(header::CONTENT_TYPE, "application/json")], json).into_response(),
        Err(e) => {
            tracing::error!("Failed to serialize graph data: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to serialize graph data"})),
            )
                .into_response()
        }
    }
}
