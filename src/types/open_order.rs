use chrono::prelude::*;

use super::TradeType;
use super::deserialize::{string_to_f64, string_to_utc_datetime, number_to_bool};

/// Returned by returnOpenOrders API command.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    pub order_number: String,

    #[serde(rename = "type")]
    pub trade_type: TradeType,

    #[serde(deserialize_with = "string_to_f64")]
    pub rate: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub starting_amount: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub amount: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub total: f64,

    #[serde(deserialize_with = "string_to_utc_datetime")]
    pub date: DateTime<Utc>,

    #[serde(rename = "margin")]
    #[serde(deserialize_with = "number_to_bool")]
    pub is_margin: bool
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
           {
              "orderNumber":"332017537261",
              "type":"buy",
              "rate":"0.01000000",
              "startingAmount":"0.02000000",
              "amount":"0.02000000",
              "total":"0.00020000",
              "date":"2017-08-19 19:43:10",
              "margin":0
           }
        "#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_number, "332017537261");
        assert_eq!(order.trade_type, TradeType::Buy);
        assert_eq!(order.rate, 0.01);
        assert_eq!(order.starting_amount, 0.02);
        assert_eq!(order.amount, 0.02);
        assert_eq!(order.total, 0.0002);
        assert_eq!(order.is_margin, false);
    }
}
