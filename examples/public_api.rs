extern crate poloniex;
extern crate chrono;
extern crate reqwest;

use chrono::prelude::*;

use poloniex::PublicClient;
use poloniex::types::{CurrencyPair, Period, Currency};

use std::time::Duration;

fn main() {
    let http_client = reqwest::Client::builder().unwrap()
        .timeout(Duration::from_millis(5000))
        .build().unwrap();

    let client = PublicClient::builder()
        .http_client(http_client)
        .build().unwrap();

    let start = Utc.ymd(2017, 8, 18).and_hms(10, 0, 45);
    let end = Utc.ymd(2017, 8, 18).and_hms(10, 2, 10);

    let tickers = client.return_ticker().unwrap();
    println!("tickers = \n{:?}\n\n", tickers);

    let order_book = client.return_order_book(CurrencyPair::BtcZec, 2).unwrap();
    println!("order_book = \n{:?}\n\n", order_book);

    let trade_history = client.return_trade_history(CurrencyPair::BtcEth, start..end).unwrap();
    println!("trade_history = \n{:?}\n\n", trade_history);

    let chart_data = client.return_chart_data(CurrencyPair::BtcXmr, Period::M15, start..end).unwrap();
    println!("chart_data = \n{:?}\n\n", chart_data);

    let loan_orders = client.return_loan_orders(Currency::Btc).unwrap();
    println!("loan_orders = \n{:?}\n\n", loan_orders);

    let currencies = client.return_currencies().unwrap();
    println!("currencies = \n{:?}\n\n", currencies);
}
