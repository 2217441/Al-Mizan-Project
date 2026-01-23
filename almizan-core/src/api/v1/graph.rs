use crate::repository::db::Database;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
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
    id: Cow<'a, str>,
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
    source: Cow<'a, str>,
    target: Cow<'a, str>,
    label: Cow<'a, str>,
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
    #[allow(dead_code)]
    display_text: Option<String>,
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

    // Helper to get string representation of Thing without SurrealQL escaping (brackets)
    // Optimization: Avoids to_string() overhead (checking escaping) and sanitize_id() overhead (replacing)
    let get_id = |thing: &surrealdb::sql::Thing| -> String {
        match &thing.id {
            surrealdb::sql::Id::String(s) => format!("{}:{}", thing.tb, s),
            surrealdb::sql::Id::Number(n) => format!("{}:{}", thing.tb, n),
            _ => {
                let s = thing.to_string();
                if s.contains('⟨') || s.contains('⟩') {
                    s.replace(&['⟨', '⟩'][..], "")
                } else {
                    s
                }
            }
        }
    };

    // 1. Add Allah (the root)
    nodes.push(CytoscapeNode {
        data: NodeData {
            id: Cow::Borrowed("allah:tawhid"),
            label: "الله".to_string(),
            node_type: Cow::Borrowed("allah"),
        },
    });

    // 2. Prepare Queries
    let prophets_sql = "SELECT id, name_ar FROM prophet LIMIT 25";
    let verses_sql = "SELECT id, surah_number, ayah_number FROM quran_verse LIMIT 20";
    let hadith_sql = "SELECT id, ref_no, collection, display_text FROM semantic_hadith LIMIT 50";
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

    // 5. Process Prophets
    for prophet in &prophets {
        let prophet_id = get_id(&prophet.id);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: Cow::Owned(prophet_id.clone()),
                label: prophet.name_ar.clone(),
                node_type: Cow::Borrowed("prophet"),
            },
        });

        // Edge: Allah -> Prophet (chosen_by)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("chosen_{prophet_id}"),
                source: Cow::Borrowed("allah:tawhid"),
                target: Cow::Owned(prophet_id), // Move instead of clone
                label: Cow::Borrowed("chose"),
            },
        });
    }

    // 3. Get sample verses narrated by Prophet Muhammad (using available Juz 30 data)
    // Already fetched in parallel above as `verses`

    for verse in &verses {
        let verse_id = get_id(&verse.id);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: Cow::Owned(verse_id.clone()),
                label: format!("{}:{}", verse.surah_number, verse.ayah_number),
                node_type: Cow::Borrowed("verse"),
            },
        });

        // Edge: Prophet Muhammad -> Verse (narrated)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("narrated_{verse_id}"),
                source: Cow::Borrowed("prophet:muhammad"),
                target: Cow::Owned(verse_id), // Move instead of clone
                label: Cow::Borrowed("narrated"),
            },
        });
    }

    // 4. Get Top Narrators (from semantic hadith narrator chains)
    // Already fetched in parallel above as `narrators_list`
    // Processed BEFORE Hadiths to build narrator_ids list for edge distribution

    // Optimization: Pre-allocate ID vectors
    let mut narrator_ids: Vec<String> = Vec::with_capacity(narrators_list.len());

    for narrator in &narrators_list {
        let narrator_id = get_id(&narrator.id);

        // Collect ID for edge creation later (distribution to hadiths)
        narrator_ids.push(narrator_id.clone());

        let label_str = narrator.name_ar.as_deref().unwrap_or("راوي");
        // Optimization: Find byte index for 15th char to slice, avoiding intermediate String allocation
        let end = label_str.char_indices().map(|(i, _)| i).nth(15).unwrap_or(label_str.len());
        let gen = narrator.generation.unwrap_or(0);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: Cow::Owned(narrator_id.clone()),
                label: format!("{} (ط{})", &label_str[..end], gen),
                node_type: Cow::Borrowed("narrator"),
            },
        });

        // Edge: Prophet -> Narrator (Taught)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("taught_{narrator_id}"),
                source: Cow::Borrowed("prophet:muhammad"),
                target: Cow::Owned(narrator_id), // Move
                label: Cow::Borrowed("taught"),
            },
        });
    }

    // 5. Get Hadiths (SemanticHadith V2)
    // Already fetched in parallel above as `hadiths`

    for (i, hadith) in hadiths.iter().enumerate() {
        let hadith_id = get_id(&hadith.id);

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
                id: Cow::Owned(hadith_id.clone()),
                label: format!("{} {}", collection_label, hadith.ref_no),
                node_type: Cow::Borrowed("hadith"),
            },
        });

        // Link Narrators -> Hadiths (Round Robin Distribution)
        if !narrator_ids.is_empty() {
            // Assign to a narrator based on index
            let narrator = &narrator_ids[i % narrator_ids.len()];

            edges_vec.push(CytoscapeEdge {
                data: EdgeData {
                    id: format!("narrated_{narrator}_{hadith_id}"),
                    source: Cow::Owned(narrator.clone()),
                    target: Cow::Owned(hadith_id), // Move
                    label: Cow::Borrowed("narrated"),
                },
            });
        }
    }

    info!(
        "[TAWHIDIC GRAPH] Allah: 1, Prophets: {}, Verses: {}, Hadiths: {}, Narrators: {}, Edges: {}",
        prophets.len(),
        verses.len(),
        hadiths.len(),
        narrators_list.len(),
        edges_vec.len()
    );

    Json(GraphData {
        nodes,
        edges: edges_vec,
    })
}
