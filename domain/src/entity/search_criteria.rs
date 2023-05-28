use uuid::Uuid;

pub struct SearchCriteria {
    pub departure: Uuid,
    pub arrival: Uuid,
}
