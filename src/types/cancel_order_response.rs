use super::deserialize::{number_to_bool, string_to_f64};

#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "success")]
    #[serde(deserialize_with = "number_to_bool")]
    pub is_success: bool,

    #[serde(deserialize_with = "string_to_f64")]
    pub amount: f64,

    pub message: String
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "success":1,
                "amount":"0.01000000",
                "message":"Order #332042333440 canceled."
            }
        "#;

        let resp: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.is_success, true);
        assert_eq!(resp.amount, 0.01);
        assert_eq!(resp.message, "Order #332042333440 canceled.");
    }
}
