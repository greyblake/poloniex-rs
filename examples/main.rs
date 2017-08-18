extern crate poloniex3;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use poloniex3::PublicClient;
use poloniex3::types::CurrencyPair;

fn main() {
    let client = PublicClient::new().unwrap();
    println!("client = {:?}", client);

    //let tickers = client.return_ticker().unwrap();
    //println!("tickers = {:?}\n\n", tickers);

    let order_book = client.return_order_book(CurrencyPair::BtcZec, 2).unwrap();
    println!("order_book = {:?}", order_book);
}
