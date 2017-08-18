extern crate poloniex3;

use poloniex3::PublicClient;
use poloniex3::types::{CurrencyPair, Period, Currency};

fn main() {
    let client = PublicClient::new().unwrap();
    println!("client = {:?}", client);

    let tickers = client.return_ticker().unwrap();
    println!("tickers = {:?}\n\n", tickers);

    let order_book = client.return_order_book(CurrencyPair::BtcZec, 2).unwrap();
    println!("order_book = {:?}\n\n", order_book);

    let chart_data = client.return_chart_data(CurrencyPair::BtcXmr, 1405699200, 1405717900, Period::M15).unwrap();
    println!("chart_data = {:?}\n\n", chart_data);

    let loan_orders = client.return_loan_orders(Currency::Btc).unwrap();
    println!("loan_orders = {:?}\n\n", loan_orders);
}
