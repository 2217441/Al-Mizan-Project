use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub title: String,
    pub status: String, // Pending, Active, Fulfilled
    pub linked_hadith: String,
    pub location: String,
    pub description: String,
}

impl Event {
    pub fn new(
        title: String,
        status: String,
        linked_hadith: String,
        location: String,
        description: String,
    ) -> Self {
        Self {
            title,
            status,
            linked_hadith,
            location,
            description,
        }
    }
}
