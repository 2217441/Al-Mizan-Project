#[cfg(test)]
mod tests {
    use crate::domain::models::*;
    use surrealdb::sql::Thing;

    #[test]
    fn test_quran_verse_serialization() {
        let verse = QuranVerse {
            id: None,
            text_uthmani: "بِسْمِ ٱللَّهِ".to_string(),
            surah_number: 1,
            ayah_number: 1,
            juz_number: 1,
            revelation_place: "Makkah".to_string(),
            mutability: Mutability::CONSTANT,
        };

        let json = serde_json::to_value(&verse).unwrap();
        assert_eq!(json["surah_number"], 1);
        assert_eq!(json["mutability"], "CONSTANT");
    }

    #[test]
    fn test_ruling_mutability() {
        let ruling = FiqhRuling {
            id: None,
            text: "Wudu is required for Salah".to_string(),
            hukm: "Wajib".to_string(),
            madhab: "Shafi".to_string(),
            issued_by: Thing::from(("scholar", "shafi")),
            mutability: Mutability::VARIABLE,
        };

        let json = serde_json::to_value(&ruling).unwrap();
        assert_eq!(json["mutability"], "VARIABLE");
    }
}
