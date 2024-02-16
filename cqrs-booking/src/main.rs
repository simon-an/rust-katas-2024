use std::sync::Arc;

use chrono::NaiveDate;
use darkbird::Database;
use model::*;
use query_service::QueryService;

mod booking_service;
mod db;
mod model;
mod query_service;

#[tokio::main]
async fn main() {
    let db: Arc<Database> = Arc::new(db::database_creation().await);

    db.iter::<String, Booking>().unwrap().for_each(|r| {
        println!("{}: {:?}", r.key(), r.value());
    });
    db.iter::<String, Room>().unwrap().for_each(|r| {
        println!("{}: {:?}", r.key(), r.value());
    });

    let query_service = QueryService::new(db.clone());
    let booking_service = booking_service::BookingService::new(db.clone());

    let free_rooms = query_service.free_rooms(
        NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap(),
    );
    println!("Expect no free rooms -> Free rooms: {:?}", free_rooms);

    let free_rooms = query_service.free_rooms(
        NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2021-01-03", "%Y-%m-%d").unwrap(),
    );
    println!("Free rooms: {:?}", free_rooms);

    booking_service.book_room(Booking{
        client_id: "123".to_string(),
        room_name: free_rooms[0].room_name.clone(),
        arrival_date: NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap(),
        departure_date: NaiveDate::parse_from_str("2021-01-03", "%Y-%m-%d").unwrap(),
    }).await;

    let free_rooms = query_service.free_rooms(
        NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2021-01-03", "%Y-%m-%d").unwrap(),
    );
    println!("Expect no free rooms -> Free rooms: {:?}", free_rooms);

    let free_rooms = query_service.free_rooms(
        NaiveDate::parse_from_str("2020-12-31", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2020-12-10", "%Y-%m-%d").unwrap(),
    );
    println!("Free rooms: {:?}", free_rooms);


    let free_rooms = query_service.free_rooms(
        NaiveDate::parse_from_str("2020-12-31", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2021-01-04", "%Y-%m-%d").unwrap(),
    );
    println!("Expect no free rooms -> Free rooms: {:?}", free_rooms);
}
