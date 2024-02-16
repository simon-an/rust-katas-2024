use chrono::NaiveDate;
use darkbird::document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Booking {
    pub client_id: String,
    pub room_name: String,
    pub arrival_date: NaiveDate,
    pub departure_date: NaiveDate,
}

impl document::Document for Booking {}

impl document::Indexer for Booking {
    fn extract(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Tags for Booking {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Range for Booking {
    fn get_fields(&self) -> Vec<document::RangeField> {
        vec![]
    }
}

impl document::MaterializedView for Booking {
    fn filter(&self) -> Option<String> {
        None
    }
}

impl document::FullText for Booking {
    fn get_content(&self) -> Option<String> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub room_name: String,
}

impl document::Document for Room {}

impl document::Indexer for Room {
    fn extract(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Tags for Room {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Range for Room {
    fn get_fields(&self) -> Vec<document::RangeField> {
        vec![]
    }
}

impl document::MaterializedView for Room {
    fn filter(&self) -> Option<String> {
        None
    }
}

impl document::FullText for Room {
    fn get_content(&self) -> Option<String> {
        None
    }
}
