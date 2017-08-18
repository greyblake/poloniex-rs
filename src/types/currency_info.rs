use super::deserialize::{string_to_f64, number_to_bool};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    pub id: u64,

    pub name: String,

    #[serde(deserialize_with = "string_to_f64")]
    pub tx_fee: f64,

    pub min_conf: u64,

    pub deposit_address: Option<String>,

    #[serde(rename = "disabled", deserialize_with = "number_to_bool")]
    pub is_disabled: bool,

    #[serde(rename = "delisted", deserialize_with = "number_to_bool")]
    pub is_delisted: bool,

    #[serde(rename = "frozen", deserialize_with = "number_to_bool")]
    pub is_frozen: bool
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "id":1,
                "name":"1CRedit",
                "txFee":"0.01000000",
                "minConf":3,
                "depositAddress":null,
                "disabled":0,
                "delisted":1,
                "frozen":0
            }
        "#;

        let info: CurrencyInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, 1);
        assert_eq!(info.name, "1CRedit");
        assert_eq!(info.tx_fee, 0.01);
        assert_eq!(info.min_conf, 3);
        assert_eq!(info.deposit_address, None);
        assert_eq!(info.is_disabled, false);
        assert_eq!(info.is_delisted, true);
        assert_eq!(info.is_frozen, false);
    }
}
