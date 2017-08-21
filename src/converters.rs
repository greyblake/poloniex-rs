use std::collections::HashMap;

use errors::Result;
use types::Currency;

pub fn convert_balances(input: HashMap<Currency, String>) -> Result<HashMap<Currency, f64>> {
    let mut output: HashMap<Currency, f64> = HashMap::new();
    for (currency, balance_str) in input {
        let balance: f64 = balance_str.parse()?;
        output.insert(currency, balance);
    }
    Ok(output)
}
