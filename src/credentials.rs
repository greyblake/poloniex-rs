use errors::*;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) key: ApiKey,
    pub(crate) secret: ApiSecret
}

impl Credentials {
    pub fn new<K: Into<ApiKey>, S: Into<ApiSecret>>(key: K, secret: S) -> Self {
        Self {
            key: key.into(),
            secret: secret.into()
        }
    }

    pub fn builder() -> CredentialsBuilder {
        CredentialsBuilder::new()
    }
}


#[derive(Debug)]
pub struct CredentialsBuilder {
    key: Option<ApiKey>,
    secret: Option<ApiSecret>
}

impl CredentialsBuilder {
    pub fn new() -> Self {
        Self { key: None, secret: None }
    }

    pub fn key<T: Into<ApiKey>>(mut self, key: T) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn secret<T: Into<ApiSecret>>(mut self, secret: T) -> Self {
        self.secret = Some(secret.into());
        self
    }

    pub fn build(self) -> Result<Credentials> {
        let key = self.key.ok_or(ErrorKind::KeyMissing)?;
        let secret = self.secret.ok_or(ErrorKind::SecretMissing)?;
        Ok(Credentials::new(key, secret))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiKey(pub String);

impl From<String> for ApiKey {
    fn from(s: String) -> Self {
        ApiKey(s)
    }
}

impl<'a> From<&'a str> for ApiKey {
    fn from(s: &'a str) -> Self {
        ApiKey(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiSecret(pub String);

impl From<String> for ApiSecret {
    fn from(s: String) -> Self {
        ApiSecret(s)
    }
}

impl<'a> From<&'a str> for ApiSecret {
    fn from(s: &'a str) -> Self {
        ApiSecret(s.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_crendetials_with_builder() {
        let credentials = Credentials::builder()
            .key("KEY")
            .secret("SECRET")
            .build()
            .unwrap();
        assert_eq!(credentials.key, ApiKey("KEY".to_owned()));
        assert_eq!(credentials.secret, ApiSecret("SECRET".to_owned()));
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
