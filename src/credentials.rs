#[derive(Debug, Clone)]
pub struct Credentials {
    key: String,
    secret: String
}

impl Credentials {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }
}
