use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Mutability {
    CONSTANT,
    VARIABLE,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Grading {
    Sahih,
    Hasan,
    Daif,
    Mawdu,
}

// 1. QURAN VERSE (Thabit)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuranVerse {
    pub id: Option<Thing>,
    pub text_uthmani: String,
    // text_en moved to Translation node
    pub surah_number: i32,
    pub ayah_number: i32,
    pub juz_number: i32,
    pub revelation_place: String, // "Makkah" or "Madinah"
    pub mutability: Mutability,   // Must be CONSTANT
}

// 1a. ROOT WORD (Morphology)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootWord {
    pub id: Option<Thing>,
    pub root_ar: String,
    pub definition_en: String,
}

// 1b. CONCEPT (Ontology)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Concept {
    pub id: Option<Thing>,
    pub name_en: String,
    pub name_ar: String,
}

// 1c. DIVINE NAME (Asma ul Husna)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DivineName {
    pub id: Option<Thing>,
    pub transliteration: String,
    pub arabic: String,
    pub meaning_en: String,
}

// 2. HADITH (Thabit)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hadith {
    pub id: Option<Thing>,
    pub collection: String,
    pub hadith_number: i32,
    pub matn_ar: String,
    // matn_en moved to Translation node
    // grading moved to Edge
    pub mutability: Mutability, // Must be CONSTANT
}

// 3. FIQH RULING (Zanni)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FiqhRuling {
    pub id: Option<Thing>,
    pub text: String,
    pub hukm: String, // e.g. "Wajib"
    pub madhab: String,
    pub issued_by: Thing,       // Strict Attribution
    pub mutability: Mutability, // Must be VARIABLE
}

// 4. SCHOLAR (Context)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ScholarStatus {
    Active,
    Slashed,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scholar {
    pub id: Option<Thing>,
    pub name_ar: String,
    pub name_en: String,
    pub death_year_ah: i32,
    pub status: ScholarStatus,
    pub reputation: f64,
}

// 5. TRANSLATION (Tafsir/Zanni)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Translation {
    pub id: Option<Thing>,
    pub text_en: String,
    pub translator: String,
    pub language: String,
}

// Edge: DERIVED_FROM
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DerivedFrom {
    pub r#in: Thing,  // The Ruling
    pub r#out: Thing, // The Source (Verse/Hadith)
    pub strength: f32,
    pub method: String,
}
