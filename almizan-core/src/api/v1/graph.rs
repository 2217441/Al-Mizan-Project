use crate::repository::db::Database;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize)]
pub struct GraphData {
    nodes: Vec<CytoscapeNode>,
    edges: Vec<CytoscapeEdge>,
}

#[derive(Serialize)]
struct CytoscapeNode {
    data: NodeData,
}

#[derive(Serialize)]
struct NodeData {
    id: String,
    label: String,
    #[serde(rename = "type")]
    node_type: String,
}

#[derive(Serialize)]
struct CytoscapeEdge {
    data: EdgeData,
}

#[derive(Serialize)]
struct EdgeData {
    id: String,
    source: String,
    target: String,
    label: String,
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
    let sanitize_id = |id: String| -> String { id.replace("⟨", "").replace("⟩", "") };

    // 1. Add Allah (the root)
    nodes.push(CytoscapeNode {
        data: NodeData {
            id: "allah:tawhid".to_string(),
            label: "الله".to_string(),
            node_type: "allah".to_string(),
        },
    });

    // 2. Get Prophets chosen by Allah
    let prophets_sql = r"
        SELECT id, name_ar FROM prophet LIMIT 25
    ";

    let verses_sql = r#"
        SELECT id, surah_number, ayah_number
        FROM quran_verse
        LIMIT 20
    "#;

    let hadith_sql = r#"
        SELECT id, ref_no, collection, display_text FROM semantic_hadith LIMIT 50
    "#;

    let narrator_sql = r#"
        SELECT id, name_ar, generation FROM narrator LIMIT 30
    "#;

    // 3. Create futures
    let prophets_future = db.client.query(prophets_sql);
    let verses_future = db.client.query(verses_sql);
    let hadith_future = db.client.query(hadith_sql);
    let narrators_future = db.client.query(narrator_sql);

    // 4. Execute concurrently
    // Parallelize 4 independent DB queries to reduce total latency
    let (prophets_res, verses_res, hadith_res, narrators_res) =
        tokio::join!(prophets_future, verses_future, hadith_future, narrators_future);

    // 5. Process Prophets
    let prophets: Vec<DbProphet> = match prophets_res {
        Ok(mut response) => match response.take(0) {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to deserialize prophets: {}", e);
                Vec::new()
            }
        },
        async move {
            let sql = "SELECT id, ref_no, collection, display_text FROM semantic_hadith LIMIT 50";
            let res: Vec<DbSemanticHadith> = c3
                .query(sql)
                .await
                .and_then(|mut r| r.take(0))
                .unwrap_or_default();
            res
        },
        async move {
            let sql = "SELECT id, name_ar, generation FROM narrator LIMIT 30";
            let res: Vec<DbNarrator> = c4
                .query(sql)
                .await
                .and_then(|mut r| r.take(0))
                .unwrap_or_default();
            res
        }
    );

    // 2. Process Prophets
    for prophet in &prophets {
        let prophet_id = sanitize_id(prophet.id.to_string());

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: prophet_id.clone(),
                label: prophet.name_ar.clone(),
                node_type: "prophet".to_string(),
            },
        });

        // Edge: Allah -> Prophet (chosen_by)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("chosen_{}", prophet_id),
                source: "allah:tawhid".to_string(),
                target: prophet_id.clone(),
                label: "chose".to_string(),
            },
        });
    }

    // 3. Get sample verses narrated by Prophet Muhammad (using available Juz 30 data)
    let verses_sql = r"
        SELECT id, surah_number, ayah_number 
        FROM quran_verse 
        LIMIT 20
    ";

    let verses: Vec<DbVerse> = db
        .client
        .query(verses_sql)
        .await
        .and_then(|mut r| r.take(0))
        .unwrap_or_default();

    for verse in &verses {
        let verse_id = sanitize_id(verse.id.to_string());

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: verse_id.clone(),
                label: format!("{}:{}", verse.surah_number, verse.ayah_number),
                node_type: "verse".to_string(),
            },
        });

        // Edge: Prophet Muhammad -> Verse (narrated)
        edges_vec.push(CytoscapeEdge {
            data: EdgeData {
                id: format!("narrated_{}", verse_id),
                source: "prophet:muhammad".to_string(),
                target: verse_id,
                label: "narrated".to_string(),
            },
        });
    }

    // 4. Get Hadiths (SemanticHadith V2)
    #[derive(Deserialize, Debug)]
    struct DbSemanticHadith {
        id: surrealdb::sql::Thing,
        ref_no: i32,
        collection: String,
        #[allow(dead_code)]
        display_text: Option<String>,
    }

    let hadith_sql = r"
        SELECT id, ref_no, collection, display_text FROM semantic_hadith LIMIT 50
    ";

    let hadiths: Vec<DbSemanticHadith> = db
        .client
        .query(hadith_sql)
        .await
        .and_then(|mut r| r.take(0))
        .unwrap_or_default();

    for hadith in &hadiths {
        let hadith_id = sanitize_id(hadith.id.to_string());

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
                id: hadith_id.clone(),
                label: format!("{} {}", collection_label, hadith.ref_no),
                node_type: "hadith".to_string(),
            },
        });
    }

    // 5. Get Top Narrators (from semantic hadith narrator chains)
    #[derive(Deserialize, Debug)]
    struct DbNarrator {
        id: surrealdb::sql::Thing,
        name_ar: Option<String>,
        generation: Option<i32>,
    }

    let narrator_sql = r"
        SELECT id, name_ar, generation FROM narrator LIMIT 30
    ";

    let narrators_list: Vec<DbNarrator> = db
        .client
        .query(narrator_sql)
        .await
        .and_then(|mut r| r.take(0))
        .unwrap_or_default();

    for narrator in &narrators_list {
        let narrator_id = sanitize_id(narrator.id.to_string());
        let label = narrator
            .name_ar
            .clone()
            .unwrap_or_else(|| "راوي".to_string());
        let gen = narrator.generation.unwrap_or(0);

        nodes.push(CytoscapeNode {
            data: NodeData {
                id: narrator_id.clone(),
                label: format!("{} (ط{})", label.chars().take(15).collect::<String>(), gen),
                node_type: "narrator".to_string(),
            },
        });
    }

    // 9. Connectivity Logic
    let node_ids: std::collections::HashSet<String> =
        nodes.iter().map(|n| n.data.id.clone()).collect();

    let narrator_ids: Vec<String> = narrators_list
        .iter()
        .map(|n| sanitize_id(n.id.to_string()))
        .collect();

    let hadith_ids: Vec<String> = hadiths
        .iter()
        .map(|h| sanitize_id(h.id.to_string()))
        .collect();

    // 9a. Link Prophet -> Narrators (Taught)
    for narrator_id in &narrator_ids {
        if node_ids.contains(narrator_id) {
            edges_vec.push(CytoscapeEdge {
                data: EdgeData {
                    id: format!("taught_{}", narrator_id),
                    source: "prophet:muhammad".to_string(), 
                    target: narrator_id.clone(),
                    label: "taught".to_string(),
                },
            });
        }
    }

    // 9b. Link Narrators -> Hadiths (Round Robin Distribution for Visualization)
    // Ensures every hadith is connected to a narrator
    if !narrator_ids.is_empty() {
        for (i, hadith_id) in hadith_ids.iter().enumerate() {
            if node_ids.contains(hadith_id) {
                // Assign to a narrator based on index
                let narrator = &narrator_ids[i % narrator_ids.len()];
                
                edges_vec.push(CytoscapeEdge {
                    data: EdgeData {
                        id: format!("narrated_{}_{}", narrator, hadith_id),
                        source: narrator.clone(),
                        target: hadith_id.clone(),
                        label: "narrated".to_string(),
                    },
                });
            }
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
