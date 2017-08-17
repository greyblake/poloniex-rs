use super::deserialize::{string_to_f64, string_number_to_bool};

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookItem {
    #[serde(deserialize_with = "string_to_f64")]
    pub rate: f64,
    pub volume: f64
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub asks: Vec<OrderBookItem>,
    pub bids: Vec<OrderBookItem>,
    #[serde(deserialize_with = "string_number_to_bool")]
    pub is_frozen: bool,
    pub seq: u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "asks":[["0.09950659",7.69904723],["0.09950929",62.543]],
                "bids":[["0.09940000",47.1320727],["0.09939998",0.07609418]],
                "isFrozen":"0",
                "seq":375042151
            }
        "#;


        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.is_frozen, false);
        assert_eq!(order_book.seq, 375042151);

        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.asks[0].rate, 0.09950659);
        assert_eq!(order_book.asks[0].volume, 7.69904723);

        assert_eq!(order_book.bids.len(), 2);
        assert_eq!(order_book.bids[1].rate, 0.09939998);
        assert_eq!(order_book.bids[1].volume, 0.07609418);
    }
}
