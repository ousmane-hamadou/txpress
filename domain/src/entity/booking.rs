use uuid::Uuid;

pub struct Booking {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub reserved_seats: u8,
}
