use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct ArchivedRuling {
    title: String,
    verdict: String,
    authority: String,
    date: String,
    tags: Vec<String>,
}

fn main() {
    println!("📜 Starting Digital Geniza Archive Process...");

    // 1. Mock Data Fetch (In real life, query SurrealDB)
    let rulings = vec![
        ArchivedRuling {
            title: "Bitcoin".to_string(),
            verdict: "Permissible (Minority)".to_string(),
            authority: "Sheikh Joe Crypto".to_string(),
            date: "2025-12-03".to_string(),
            tags: vec!["Finance".to_string(), "Crypto".to_string()],
        },
        ArchivedRuling {
            title: "Riba".to_string(),
            verdict: "Haram (Consensus)".to_string(),
            authority: "The Four Imams".to_string(),
            date: "0650-01-01".to_string(),
            tags: vec!["Finance".to_string(), "Prohibition".to_string()],
        },
    ];

    // 2. Ensure Archive Directory Exists
    let archive_dir = Path::new("archive");
    if !archive_dir.exists() {
        fs::create_dir(archive_dir).expect("Failed to create archive directory");
    }

    // 3. Serialize to Markdown + YAML Frontmatter
    for ruling in rulings {
        let yaml = serde_yaml::to_string(&ruling).expect("Failed to serialize YAML");
        let content = format!(
            "---\n{}\n---\n\n# {}\n\n**Verdict**: {}\n\n**Authority**: {}\n\n*Archived on {}*",
            yaml, ruling.title, ruling.verdict, ruling.authority, ruling.date
        );

        let filename = format!("archive/{}.md", ruling.title.to_lowercase());
        fs::write(&filename, content).expect("Failed to write archive file");
        println!("✅ Archived: {}", filename);
    }

    // 4. Simulate IPFS Upload
    println!("\n🚀 Uploading to IPFS (Simulated)...");
    println!("QmHash: QmXyZ1234567890abcdef... (Pinned to Arweave)");
    println!("The Knowledge is Safe.");
}
