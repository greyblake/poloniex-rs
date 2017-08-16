use serde_json;
use serde::de::{self, Deserialize};

//use std::collections::HashMap;
//type Tickers = HashMap<String, Ticker>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(deserialize_with = "string_to_f64")]
    pub last: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub lowest_ask: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub highest_bid: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub percent_change: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub base_volume: f64,

    #[serde(deserialize_with = "string_to_f64")]
    pub quote_volume: f64
}

fn string_to_f64<'de, D>(d: D) -> Result<f64, D::Error> where D: de::Deserializer<'de>
{
    Ok((String::deserialize(d)?).parse().expect("Parse error"))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "last":"0.0251",
                "lowestAsk":"0.0258",
                "highestBid":"0.0252",
                "percentChange":"0.023",
                "baseVolume":"6.16",
                "quoteVolume":"245.82"
            }
        "#;

        let ticker : Ticker = serde_json::from_str(json).unwrap();

        assert_eq!(ticker.last, 0.0251);
        assert_eq!(ticker.lowest_ask, 0.0258);
        assert_eq!(ticker.highest_bid, 0.0252);
        assert_eq!(ticker.percent_change, 0.023);
        assert_eq!(ticker.base_volume, 6.16);
        assert_eq!(ticker.quote_volume, 245.82);
    }
}
