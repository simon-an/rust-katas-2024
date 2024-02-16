use crate::model::*;
use chrono::NaiveDate;
use darkbird::{Database, Options, Schema, StorageType};

fn factory_option(name: &str) -> Options {
    Options::new(".", name, 1000, StorageType::RamCopies, true)
}

pub async fn database_creation() -> Database {
    let db = Schema::new()
        .with_datastore::<String, Booking>(factory_option("Booking"))
        .await
        .unwrap()
        .with_datastore::<String, Room>(factory_option("Room"))
        .await
        .unwrap()
        .build();

    let room = Room {
        room_name: String::from("room1"),
    };

    let booking = Booking {
        client_id: String::from("123"),
        room_name: String::from("room1"),
        arrival_date: NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
        departure_date: NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap(),
    };

    db.insert(
        format!("{}{}", booking.room_name, booking.arrival_date),
        booking,
    )
    .await
    .unwrap();

    db.insert(room.room_name.clone(), room).await.unwrap();

    db
}
