use std::{collections::HashSet, sync::Arc};

use crate::model::*;
use chrono::NaiveDate;
use darkbird::Database;

pub struct QueryService {
    db: Arc<Database>,
}
impl QueryService {
    pub fn new(db: Arc<Database>) -> QueryService {
        QueryService { db }
    }
    // Room[] freeRooms(arrival: Date, departure: Date)
    pub fn free_rooms(&self, arrival: NaiveDate, departure: NaiveDate) -> Vec<Room> {
        let results: Vec<Booking> = self
            .db
            .iter::<String, Booking>()
            .unwrap()
            .map(|f| f.value().clone())
            .filter(|old| {
                // |o|o|o|a|
                // |d|o|o|o|
                !(departure <= old.arrival_date || arrival >= old.departure_date)
            })
            .collect();

        let blocked_room_names: HashSet<String> =
            results.iter().map(|r| r.room_name.clone()).collect();

        let rooms = self
            .db
            .iter::<String, Room>()
            .unwrap()
            .map(|f| f.value().clone())
            .filter(|r| !blocked_room_names.contains(&r.room_name))
            .collect();
        rooms
    }
}
