use errors::*;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) key: String,
    pub(crate) secret: String
}

impl Credentials {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }

    pub fn builder() -> CredentialsBuilder {
        CredentialsBuilder::new()
    }
}


pub struct CredentialsBuilder {
    key: Option<String>,
    secret: Option<String>
}

impl CredentialsBuilder {
    pub fn new() -> Self {
        Self { key: None, secret: None }
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    pub fn build(self) -> Result<Credentials> {
        let key = self.key.ok_or(ErrorKind::KeyMissing)?;
        let secret = self.secret.ok_or(ErrorKind::SecretMissing)?;
        Ok(Credentials::new(key, secret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_crendetials_with_builder() {
        let credentials = Credentials::builder()
            .key("KEY".to_owned())
            .secret("SECRET".to_owned())
            .build()
            .unwrap();
        assert_eq!(credentials.key, "KEY");
        assert_eq!(credentials.secret, "SECRET");
    }

    #[test]
    fn test_when_some_params_are_missing() {
        let result = Credentials::builder()
            .key("KEY".to_owned())
            .build();
        assert_eq!(result.unwrap_err().description(), "secret is missing");

        let result = Credentials::builder()
            .secret("SECRET".to_owned())
            .build();
        assert_eq!(result.unwrap_err().description(), "key is missing");
    }
}
