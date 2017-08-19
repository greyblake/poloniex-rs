extern crate poloniex;
extern crate dotenv;

use dotenv::dotenv;
use poloniex::{Credentials, Client};
use poloniex::types::*;

use std::env;

fn main() {
    dotenv().unwrap();

    let credentials = Credentials::new(
        env::var("POLONIEX_KEY").expect("Fetch POLONIEX_KEY"),
        env::var("POLONIEX_SECRET").expect("Fetch POLONIEX_SECRET")
    );
    let client = Client::new(credentials).unwrap();

    println!("return_balances()\n");
    let balances = client.return_balances().unwrap();
    println!("{:?}\n\n", balances);

    println!("buy()\n");
    let opened_order = client.buy(CurrencyPair::BtcEth, 0.01, 0.01).unwrap();
    println!("{:?}\n\n", opened_order);

    println!("sell()\n");
    let opened_order = client.sell(CurrencyPair::BtcEth, 10.0, 0.01).unwrap();
    println!("{:?}\n\n", opened_order);
}
