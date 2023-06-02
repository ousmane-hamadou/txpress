pub struct Owner {
    pub full_name: String,
    pub password: String,
}

impl Owner {
    pub fn new(name: &str, password: &str) -> Self {
        Owner {
            full_name: name.into(),
            password: password.into(),
        }
    }
}
