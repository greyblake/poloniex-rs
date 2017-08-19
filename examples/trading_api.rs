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

    //let balances = client.return_balances().unwrap();
    //println!("balances = \n{:?}\n\n", balances);

    let opened_order = client.buy(CurrencyPair::BtcEth, 0.01, 0.01).unwrap();
    println!("BUY: \n{:?}\n\n", opened_order);
}
