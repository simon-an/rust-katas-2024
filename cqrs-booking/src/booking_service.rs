use std::sync::Arc;

use darkbird::Database;

use crate::model::*;

pub struct BookingService {
    db: Arc<Database>,
    // read_registry: ReadRegistry,
    // write_registry: WriteRegistry,
}
impl BookingService {
    pub fn new(db: Arc<Database>) -> BookingService {
        BookingService { db }
    }
    pub async fn book_room(&self, booking: Booking) {
        self.db
            .insert(
                format!("{}{}", booking.room_name, booking.arrival_date),
                booking,
            )
            .await
            .unwrap();
    }
}
