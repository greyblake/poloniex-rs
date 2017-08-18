#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartDataItem {
    pub date: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub weighted_average: f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialization() {
        let json = r#"
            {
                "date":1405699200,
                "high":0.0045388,
                "low":0.00403001,
                "open":0.00404545,
                "close":0.00435873,
                "volume":44.34555992,
                "quoteVolume":10311.88079097,
                "weightedAverage":0.00430043
            }
        "#;

        let item: ChartDataItem = serde_json::from_str(json).unwrap();

        assert_eq!(item.date, 1405699200);
        assert_eq!(item.high, 0.0045388);
        assert_eq!(item.low, 0.00403001);
        assert_eq!(item.open, 0.00404545);
        assert_eq!(item.close, 0.00435873);
        assert_eq!(item.volume, 44.34555992);
        assert_eq!(item.quote_volume, 10311.88079097);
        assert_eq!(item.weighted_average, 0.00430043);
    }
}
