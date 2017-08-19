extern crate poloniex3;
extern crate dotenv;

use dotenv::dotenv;
use poloniex3::{Credentials, Client};

use std::env;

fn main() {
    dotenv().unwrap();
    //let start = Utc.ymd(2017, 8, 18).and_hms(10, 0, 45);
    //let end = Utc.ymd(2017, 8, 18).and_hms(10, 2, 10);

    let credentials = Credentials::new(
        env::var("POLONIEX_KEY").expect("Fetch POLONIEX_KEY"),
        env::var("POLONIEX_SECRET").expect("Fetch POLONIEX_SECRET")
    );
    let client = Client::new(credentials).unwrap();

    let balances = client.return_balances().unwrap();
    println!("balances = {:?}", balances);
}
