use uuid::Uuid;

pub struct Taxi {
    pub id: Uuid,
    pub number: String,
    pub brand: String,
    pub number_of_seats: i32,
}

impl Taxi {
    pub fn new(num: &str, brand: &str, num_of_seats: i32) -> Self {
        Taxi {
            id: Uuid::new_v4(),
            number: num.into(),
            brand: brand.into(),
            number_of_seats: num_of_seats,
        }
    }
}
