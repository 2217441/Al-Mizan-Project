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
/// Shows: Allah → Prophets → Quran Verses (via chosen_by and narrated_quran edges)
pub async fn get_graph(State(db): State<Database>) -> impl IntoResponse {
    let mut nodes: Vec<CytoscapeNode> = Vec::new();
    let mut edges_vec: Vec<CytoscapeEdge> = Vec::new();

    // Helper to sanitize Surreal IDs
    // Optimization: Avoid unnecessary allocations if no sanitization is needed
    let sanitize_id = |id: String| -> String {
        if id.contains('⟨') || id.contains('⟩') {
            id.replace(&['⟨', '⟩'][..], "")
        } else {
            id
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
        let prophet_id = sanitize_id(prophet.id.to_string());

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
                id: format!("chosen_{}", prophet_id),
                source: Cow::Borrowed("allah:tawhid"),
                target: Cow::Owned(prophet_id.clone()),
                label: Cow::Borrowed("chose"),
            },
        });
    }

    // 3. Get sample verses narrated by Prophet Muhammad (using available Juz 30 data)
    // Already fetched in parallel above as `verses`

    for verse in &verses {
        let verse_id = sanitize_id(verse.id.to_string());

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
                id: format!("narrated_{}", verse_id),
                source: Cow::Borrowed("prophet:muhammad"),
                target: Cow::Owned(verse_id),
                label: Cow::Borrowed("narrated"),
            },
        });
    }

    // 4. Get Hadiths (SemanticHadith V2)
    // Already fetched in parallel above as `hadiths`

    // Optimization: Pre-allocate ID vectors to avoid redundant iterations and sanitizations later
    let mut hadith_ids: Vec<String> = Vec::with_capacity(hadiths.len());
    let mut narrator_ids: Vec<String> = Vec::with_capacity(narrators_list.len());

    for hadith in &hadiths {
        let hadith_id = sanitize_id(hadith.id.to_string());

        // Collect ID for edge creation later
        hadith_ids.push(hadith_id.clone());

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
    }

    // 5. Get Top Narrators (from semantic hadith narrator chains)
    // Already fetched in parallel above as `narrators_list`

    for narrator in &narrators_list {
        let narrator_id = sanitize_id(narrator.id.to_string());

        // Collect ID for edge creation later
        narrator_ids.push(narrator_id.clone());

        let label = narrator
            .name_ar
            .clone()
            .unwrap_or_else(|| "راوي".to_string());
        let gen = narrator.generation.unwrap_or(0);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: Cow::Owned(narrator_id.clone()),
                label: format!("{} (ط{})", label.chars().take(15).collect::<String>(), gen),
                node_type: Cow::Borrowed("narrator"),
            },
        });
    }

    // 9. Connectivity Logic
    // Optimization: Removed redundant O(N) node_ids set construction and ID re-sanitization loops.
    // We already collected valid IDs in the loops above.

    // 9a. Link Prophet -> Narrators (Taught)
    for narrator_id in &narrator_ids {
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("taught_{narrator_id}"),
                source: Cow::Borrowed("prophet:muhammad"),
                target: Cow::Owned(narrator_id.clone()),
                label: Cow::Borrowed("taught"),
            },
        });
    }

    // 9b. Link Narrators -> Hadiths (Round Robin Distribution for Visualization)
    if !narrator_ids.is_empty() {
        for (i, hadith_id) in hadith_ids.iter().enumerate() {
            // Assign to a narrator based on index
            let narrator = &narrator_ids[i % narrator_ids.len()];

            edges_vec.push(CytoscapeEdge {
                data: EdgeData {
                    id: format!("narrated_{narrator}_{hadith_id}"),
                    source: Cow::Owned(narrator.clone()),
                    target: Cow::Owned(hadith_id.clone()),
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
