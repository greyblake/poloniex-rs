#[derive(Debug, Clone, Deserialize)]
pub struct ErrorMessage {
    pub error: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            { "error":"Not enough BTC." }
        "#;
        let error: ErrorMessage = serde_json::from_str(json).unwrap();
        assert_eq!(error.error, "Not enough BTC.");
    }
}
