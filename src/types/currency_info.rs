use super::deserialize::{string_to_f64, string_number_to_bool};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    pub id: u64,

    pub name: String,

    #[serde(deserialize_with = "string_to_f64")]
    pub tx_fee: f64,

    pub min_conf: u64,

    pub deposit_address: Option<String>,

    pub disabled: u8,

    pub delisted: u8,

    pub frozen: u8
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
        assert_eq!(info.disabled, 0);
        assert_eq!(info.delisted, 1);
        assert_eq!(info.frozen, 0);
    }
}
