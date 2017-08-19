use chrono::prelude::*;

use super::TradeType;
use super::deserialize::{string_to_f64, string_to_utc_datetime};

/// Structure returned from `buy` and `sell` API calls.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenedOrder {
    pub order_number: String,
    pub resulting_trades: Vec<ResultingTrade>
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultingTrade {
    #[serde(deserialize_with = "string_to_f64")]
    amount: f64,

    #[serde(deserialize_with = "string_to_utc_datetime")]
    date: DateTime<Utc>,

    #[serde(deserialize_with = "string_to_f64")]
    rate: f64,

    #[serde(deserialize_with = "string_to_f64")]
    total: f64,

    #[serde(rename = "tradeID")]
    trade_id: String,

    #[serde(rename = "type")]
    trade_type: TradeType
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
               "orderNumber":"31226040",
               "resultingTrades":[
                  {
                     "amount":"338.8732",
                     "date":"2014-10-18 23:03:21",
                     "rate":"0.00000173",
                     "total":"0.00058625",
                     "tradeID":"16164",
                     "type":"buy"
                  }
               ]
            }
        "#;

        let order: OpenedOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_number, "31226040");
        assert_eq!(order.resulting_trades.len(), 1);

        let trade = &order.resulting_trades[0];
        assert_eq!(trade.amount, 338.8732);
        assert_eq!(trade.date, Utc.ymd(2014, 10, 18).and_hms(23, 3, 21));
        assert_eq!(trade.rate, 0.00000173);
        assert_eq!(trade.total, 0.00058625);
        assert_eq!(trade.trade_id, "16164");
        assert_eq!(trade.trade_type, TradeType::Buy);
    }
}
