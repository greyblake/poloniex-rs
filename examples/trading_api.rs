extern crate poloniex;
extern crate dotenv;

use dotenv::dotenv;
use poloniex::{Credentials, Client};

use std::env;

fn main() {
    dotenv().unwrap();

    let credentials = Credentials::new(
        env::var("POLONIEX_KEY").expect("Fetch POLONIEX_KEY"),
        env::var("POLONIEX_SECRET").expect("Fetch POLONIEX_SECRET")
    );
    let client = Client::new(credentials).unwrap();

    let balances = client.return_balances().unwrap();
    println!("balances = {:?}", balances);
}
