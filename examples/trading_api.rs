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

    //println!("return_balances()");
    //let balances = client.return_balances().unwrap();
    //println!("{:?}\n\n", balances);

    //println!("buy()");
    //let opened_order = client.buy(CurrencyPair::BtcEth, 0.01, 0.01).unwrap();
    //println!("{:?}\n\n", opened_order);

    //println!("sell()");
    //let opened_order = client.sell(CurrencyPair::BtcEth, 10.0, 0.01).unwrap();
    //println!("{:?}\n\n", opened_order);

    println!("return_open_orders()");
    let orders = client.return_open_orders(CurrencyPair::BtcEth).unwrap();
    println!("{:?}\n\n", orders);

    println!("return_all_open_orders()");
    let all_orders = client.return_all_open_orders().unwrap();
    println!("{:?}\n\n", all_orders);
}
