extern crate poloniex3;
extern crate chrono;

use chrono::prelude::*;
use chrono::naive::NaiveDate;

use poloniex3::PublicClient;
use poloniex3::types::{CurrencyPair, Period, Currency};

fn main() {
    let start = Utc.ymd(2017, 8, 18).and_hms(10, 0, 45);
    let end = Utc.ymd(2017, 8, 18).and_hms(10, 2, 10);

    let client = PublicClient::new().unwrap();

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
