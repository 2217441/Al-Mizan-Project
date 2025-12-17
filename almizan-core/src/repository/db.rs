use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct Database {
    pub client: Arc<Surreal<Client>>,
}

impl Database {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = std::env::var("DB_URL").unwrap_or("127.0.0.1:8000".to_string());
        let client = Surreal::new::<Ws>(db_url).await?;

        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("idc").use_db("main").await?;

        let db = Self {
            client: Arc::new(client),
        };

        // db.seed().await?;

        // Seed Legal Ontology
        // Legacy seeding disabled for Phase 1
        // let _riba = db
        //     .client
        //     .query("CREATE concept:riba SET name = 'Riba', description = 'Usury/Interest'")
        //     .await?;
        // ... (rest commented out) ...
        // let _edge_inv2 = db
        //     .client
        //     .query("RELATE rule:prohibition_of_riba->invalidates_parameter->parameter:late_fee_calculation")
        //     .await?;

        Ok(db)
    }

    pub async fn seed(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Always re-seed for now to enforce the new structure
        tracing::info!("Re-seeding database with Tawhidic structure...");

        self.client
            .query(
                "
            BEGIN TRANSACTION;
            DELETE node;
            DELETE edge;

            CREATE node:root SET label = \"Allah (SWT)\", type = \"divine\";
            
            -- Verse 1 (Mansukh)
            CREATE node:v1 SET 
                text = \"And those who are taken in death among you and leave wives behind - for their wives is a bequest: maintenance for one year without turning [them] out.\",
                revelation_order = 87,
                uthmani_script = \"وَالَّذِينَ يُتَوَفَّوْنَ مِنكُمْ وَيَذَرُونَ أَزْوَاجًا وَصِيَّةً لِّأَزْوَاجِهِم مَّتَاعًا إِلَى الْحَوْلِ غَيْرَ إِخْرَاجٍ\",
                indopak_script = \"والذين يتوفون منكم ويذرون أزواجا وصية لأزواجهم متاعا إلى الحول غير إخراج\",
                simple_clean = \"والذين يتوفون منكم ويذرون ازواجا وصية لازواجهم متاعا الى الحول غير اخراج\",
                buckwalter = \"waAl~a*iyna yutawaf~awna minkum waya*aruwna >azwaAjFA waSiy~apF l~i>azwaAjihim m~ataAEFA <ilaY AlHawli gayra <ixraAjK\",
                digitized_by = \"System Seeder\",
                signature = \"sha256:mock_signature_v1\";

            -- Verse 2 (Nasikh)
            CREATE node:v2 SET 
                text = \"And those who are taken in death among you and leave wives behind - they, [the wives, shall] wait four months and ten [days].\",
                revelation_order = 113,
                uthmani_script = \"فَإِذَا انسَلَخَ الْأَشْهُرُ الْحُرُمُ\",
                indopak_script = \"فإذا انسلخ الأشهر الحرم\",
                simple_clean = \"فاذا انسلخ الاشهر الحرم\",
                buckwalter = \"fa<i*A Ansalaxa Al>a$huru AlHurumu\",
                digitized_by = \"System Seeder\",
                signature = \"sha256:mock_signature_v2\";

            -- Verse Mappings (Coordinate System)
            CREATE verse_mapping SET verse_id = node:v1, system = \"kufic\", surah = 2, ayah = 106, digitized_by = \"System Seeder\", signature = \"sha256:mock_sig_vm1\";
            CREATE verse_mapping SET verse_id = node:v2, system = \"kufic\", surah = 9, ayah = 5, digitized_by = \"System Seeder\", signature = \"sha256:mock_sig_vm2\";

            -- Schools
            CREATE node:shafi SET label = \"Shafi'i Madhab\", type = \"school\", digitized_by = \"System Seeder\", signature = \"sha256:mock_sig_school\";

            -- Scholars
            CREATE node:s1 SET 
                label = \"Imam Shafi'i\", 
                type = \"scholar\", 
                death_date = \"204 AH\",
                school = \"Shafi'i\",
                digitized_by = \"System Seeder\",
                signature = \"sha256:mock_sig_scholar\";
            
            -- Edges: Scholar -> School
            RELATE node:s1->follows->node:shafi SET rank = 1, digitized_by = \"System Seeder\", signature = \"sha256:mock_sig_edge1\";

            -- Hadith
            CREATE node:h1 SET label = \"Hadith Bukhari 123\", type = \"hadith\", digitized_by = \"System Seeder\", signature = \"sha256:mock_sig_hadith\";
            
            -- Relationships
            RELATE node:root->edge->node:v1 SET label = \"revealed\", authority = \"Divine Revelation\";
            RELATE node:root->edge->node:v2 SET label = \"revealed\", authority = \"Divine Revelation\";
            
            -- Opinion (The Triple-Node)
            CREATE node:op1 SET 
                label = \"Opinion: 9:5 Abrogates 2:106\", 
                type = \"opinion\", 
                verdict = \"Abrogated\",
                methodology = \"Naskh (General vs Specific)\",
                school = \"Shafi'i\",
                weight = 1.0,
                digitized_by = \"System Seeder\",
                signature = \"sha256:mock_sig_opinion\";

            -- Scholar holds Opinion
            RELATE node:s1->holds->node:op1;

            -- Opinion cites Evidence
            RELATE node:op1->cites->node:v1; -- Mansukh
            RELATE node:op1->cites->node:v2; -- Nasikh

            -- Legacy Edge for simple visualization (attributed to Scholar)
            RELATE node:v2->edge->node:v1 SET label = \"abrogates\", authority = \"Imam Shafi'i\";
            
            RELATE node:s1->edge->node:v2 SET label = \"confirms\", authority = \"Al-Shafi'i\";
            RELATE node:h1->edge->node:v2 SET label = \"supports\", authority = \"Al-Bukhari\";
            COMMIT TRANSACTION;
        ",
            )
            .await?;

        Ok(())
    }
}
