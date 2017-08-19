#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) key: String,
    pub(crate) secret: String
}

impl Credentials {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }
}
