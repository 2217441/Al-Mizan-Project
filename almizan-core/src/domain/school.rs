use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(dead_code)]
pub enum School {
    Shafi,
    Hanafi,
    Maliki,
    Hanbali,
    Jafari,
    Zaydi,
    Ibadi,
    Zahiri,
    Unknown,
}

impl std::fmt::Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            School::Shafi => "Shafi'i",
            School::Hanafi => "Hanafi",
            School::Maliki => "Maliki",
            School::Hanbali => "Hanbali",
            School::Jafari => "Ja'fari",
            School::Zaydi => "Zaydi",
            School::Ibadi => "Ibadi",
            School::Zahiri => "Zahiri",
            School::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}
