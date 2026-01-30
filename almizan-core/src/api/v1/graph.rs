use super::utils::{format_surreal_id, serialize_thing_id};
use crate::repository::db::Database;
use crate::api::v1::utils::format_surreal_id;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::sql::Thing;
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
    label: String,
    #[serde(rename = "type")]
    node_type: Cow<'a, str>,
}

#[derive(Serialize)]
struct CytoscapeEdge<'a> {
    data: EdgeData<'a>,
}

#[derive(Serialize)]
struct EdgeData<'a> {
    id: String,
    source: GraphId<'a>,
    target: GraphId<'a>,
    label: Cow<'a, str>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GraphId<'a> {
    Thing(#[serde(serialize_with = "serialize_thing_id")] Thing),
    Str(Cow<'a, str>),
}

impl From<Thing> for GraphId<'_> {
    fn from(t: Thing) -> Self {
        GraphId::Thing(t)
    }
}

impl<'a> From<&'a str> for GraphId<'a> {
    fn from(s: &'a str) -> Self {
        GraphId::Str(Cow::Borrowed(s))
    }
}

#[derive(Deserialize, Debug)]
struct DbVerse {
    id: Thing,
    surah_number: i32,
    ayah_number: i32,
}

#[derive(Deserialize, Debug)]
struct DbProphet {
    id: Thing,
    name_ar: String,
}

#[derive(Deserialize, Debug)]
struct DbSemanticHadith {
    id: Thing,
    ref_no: i32,
    collection: String,
}

#[derive(Deserialize, Debug)]
struct DbNarrator {
    id: Thing,
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
            id: GraphId::from("allah:tawhid"),
            label: "الله".to_string(),
            node_type: Cow::Borrowed("allah"),
        },
    });

    // 2. Prepare Queries
    let prophets_sql = "SELECT id, name_ar FROM prophet LIMIT 25";
    let verses_sql = "SELECT id, surah_number, ayah_number FROM quran_verse LIMIT 20";
    let hadith_sql = "SELECT id, ref_no, collection FROM semantic_hadith LIMIT 50";
    let narrator_sql = "SELECT id, name_ar, generation FROM narrator LIMIT 30";

    // 3. Execute concurrently
    let (prophets_res, verses_res, hadith_res, narrators_res) = tokio::join!(
        db.client.query(prophets_sql),
        db.client.query(verses_sql),
        db.client.query(hadith_sql),
        db.client.query(narrator_sql)
    );

    // 4. Unpack Results
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

    let prophets_len = prophets.len();
    let verses_len = verses.len();
    let hadiths_len = hadiths.len();
    let narrators_len = narrators_list.len();

    // 5. Process Prophets
    for prophet in prophets {
        // Optimization: Use GraphId::Thing to avoid string allocation for Node ID and Target
        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::from(prophet.id.clone()),
                label: prophet.name_ar,
                node_type: Cow::Borrowed("prophet"),
            },
        });

        // Generate deterministic Edge ID
        let prophet_id_str = format_surreal_id(&prophet.id);

        // Edge: Allah -> Prophet (chosen_by)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("chosen_{prophet_id_str}"),
                source: GraphId::from("allah:tawhid"),
                target: GraphId::from(prophet.id), // Move Thing
                label: Cow::Borrowed("chose"),
            },
        });
    }

    // Process Verses
    for verse in verses {
        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::from(verse.id.clone()),
                label: format!("{}:{}", verse.surah_number, verse.ayah_number),
                node_type: Cow::Borrowed("verse"),
            },
        });

        let verse_id_str = format_surreal_id(&verse.id);

        // Edge: Prophet Muhammad -> Verse (narrated)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("narrated_{verse_id_str}"),
                source: GraphId::from("prophet:muhammad"),
                target: GraphId::from(verse.id), // Move Thing
                label: Cow::Borrowed("narrated"),
            },
        });
    }

    // Process Narrators
    // Optimization: Store Things instead of formatted Strings
    let mut narrator_ids: Vec<Thing> = Vec::with_capacity(narrators_len);

    for narrator in narrators_list {
        narrator_ids.push(narrator.id.clone());

        let label_str = narrator.name_ar.as_deref().unwrap_or("راوي");
        let end = label_str.char_indices().map(|(i, _)| i).nth(15).unwrap_or(label_str.len());
        let gen = narrator.generation.unwrap_or(0);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: GraphId::from(narrator.id.clone()),
                label: format!("{} (ط{})", &label_str[..end], gen),
                node_type: Cow::Borrowed("narrator"),
            },
        });

        let narrator_id_str = format_surreal_id(&narrator.id);

        // Edge: Prophet -> Narrator (Taught)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("taught_{narrator_id_str}"),
                source: GraphId::from("prophet:muhammad"),
                target: GraphId::from(narrator.id), // Move Thing
                label: Cow::Borrowed("taught"),
            },
        });
    }

    // Process Hadiths
    for (i, hadith) in hadiths.into_iter().enumerate() {
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
                id: GraphId::from(hadith.id.clone()),
                label: format!("{} {}", collection_label, hadith.ref_no),
                node_type: Cow::Borrowed("hadith"),
            },
        });

        // Link Narrators -> Hadiths (Round Robin Distribution)
        if !narrator_ids.is_empty() {
            let narrator = &narrator_ids[i % narrator_ids.len()];

            let hadith_id_str = format_surreal_id(&hadith.id);
            let narrator_id_str = format_surreal_id(narrator);

            edges_vec.push(CytoscapeEdge {
                data: EdgeData {
                    id: format!("narrated_{narrator_id_str}_{hadith_id_str}"),
                    source: GraphId::from(narrator.clone()), // Clone Thing
                    target: GraphId::from(hadith.id), // Move Thing
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

    Json(GraphData {
        nodes,
        edges: edges_vec,
    })
}
