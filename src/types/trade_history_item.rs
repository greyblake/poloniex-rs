use chrono::prelude::*;

use super::deserialize::{string_to_f64, string_to_utc_datetime};
use super::TradeType;

#[derive(Debug, Clone, Deserialize)]
pub struct TradeHistoryItem {
    #[serde(rename="globalTradeID")]
    pub global_trade_id: u64,

    #[serde(rename="tradeID")]
    pub trade_id: u64,

    #[serde(deserialize_with = "string_to_utc_datetime")]
    pub date: DateTime<Utc>,

    #[serde(rename="type")]
    pub trade_type: TradeType,

    #[serde(deserialize_with = "string_to_f64")]
    pub rate: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub amount: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub total: f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "globalTradeID":2036467,
                "tradeID":21387,
                "date":"2014-09-12 05:21:26",
                "type":"buy",
                "rate":"0.00008943",
                "amount":"1.27241180",
                "total":"0.00011379"
            }
        "#;

        let item: TradeHistoryItem= serde_json::from_str(json).unwrap();

        assert_eq!(item.global_trade_id, 2036467);
        assert_eq!(item.trade_id, 21387);
        assert_eq!(item.date, Utc.ymd(2014, 9, 12).and_hms(5, 21, 26));
        assert_eq!(item.trade_type, TradeType::Buy);
        assert_eq!(item.rate, 0.00008943);
        assert_eq!(item.amount, 1.27241180);
        assert_eq!(item.total, 0.00011379);
    }
}
