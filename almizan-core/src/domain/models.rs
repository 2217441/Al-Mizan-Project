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
    pub text_en: String,
    pub surah_number: i32,
    pub ayah_number: i32,
    pub mutability: Mutability, // Must be CONSTANT
}

// 2. HADITH (Thabit)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hadith {
    pub id: Option<Thing>,
    pub collection: String,
    pub hadith_number: i32,
    pub matn_ar: String,
    pub matn_en: String,
    pub grading: Grading,
    pub mutability: Mutability, // Must be CONSTANT
}

// 3. FIQH RULING (Zanni)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FiqhRuling {
    pub id: Option<Thing>,
    pub text: String,
    pub hukm: String, // e.g. "Wajib"
    pub madhab: String,
    pub mutability: Mutability, // Must be VARIABLE
}

// Edge: DERIVED_FROM
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DerivedFrom {
    pub r#in: Thing,  // The Ruling
    pub r#out: Thing, // The Source (Verse/Hadith)
    pub strength: f32,
    pub method: String,
}
