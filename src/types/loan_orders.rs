use super::deserialize::string_to_f64;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanOrder {
    #[serde(deserialize_with = "string_to_f64")]
    rate: f64,
    #[serde(deserialize_with = "string_to_f64")]
    amount: f64,
    range_min: u32,
    range_max: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoanOrders {
    offers: Vec<LoanOrder>,
    demands: Vec<LoanOrder>
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deseriale() {
        let json = r#"
            {
                "offers":[
                    {"rate":"0.00009400","amount":"0.18728479","rangeMin":2,"rangeMax":2},
                    {"rate":"0.00009500","amount":"4.44010737","rangeMin":2,"rangeMax":2}
                ],
                "demands":[]
            }
        "#;

        let loan_orders: LoanOrders = serde_json::from_str(json).unwrap();

        assert_eq!(loan_orders.offers.len(), 2);
        assert_eq!(loan_orders.demands.len(), 0);

        let offer = &loan_orders.offers[0];
        assert_eq!(offer.rate, 0.00009400);
        assert_eq!(offer.amount, 0.18728479);
        assert_eq!(offer.range_min, 2);
        assert_eq!(offer.range_max, 2);
    }
}
