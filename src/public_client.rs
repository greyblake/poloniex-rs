use reqwest;
use errors::*;

use types::{Ticker, CurrencyPair, OrderBook, Period, ChartDataItem, Currency, LoanOrders};

use std::collections::HashMap;

type Tickers = HashMap<CurrencyPair, Ticker>;

#[derive(Debug)]
pub struct PublicClient {
    reqwest_client: reqwest::Client
}

impl PublicClient {
    pub fn new() -> Result<Self> {
        let reqwest_client = reqwest::Client::new()?;
        let client = Self { reqwest_client };
        Ok(client)
    }

    pub fn return_ticker(&self) -> Result<Tickers> {
        self.get("https://poloniex.com/public?command=returnTicker")
    }


    // Currency pair keys are mixed with "totalBTC", "totalETH", etc.
    // So it can not be parsed correctly
    //
    //pub fn return_24_volume(&self) -> Result<_> {
    //    self.get("https://poloniex.com/public?command=return24hVolume")
    //}

    pub fn return_order_book(&self, currency_pair: CurrencyPair, depth: u32) -> Result<OrderBook> {
        let url = format!("https://poloniex.com/public?command=returnOrderBook&currencyPair={}&depth={}", currency_pair, depth);
        self.get(&url)
    }

    //pub fn return_trade_history(&self) ->

    pub fn return_chart_data(&self, currency_pair: CurrencyPair, start: u64, end: u64, period: Period) -> Result<Vec<ChartDataItem>> {
        let url = format!("https://poloniex.com/public?command=returnChartData&currencyPair={}&start={}&end={}&period={}", currency_pair, start, end, period);
        self.get(&url)
    }

    // returnCurrencies


    pub fn return_loan_orders(&self, currency: Currency) -> Result<LoanOrders> {
        let url = format!("https://poloniex.com/public?command=returnLoanOrders&currency={}", currency);
        self.get(&url)
    }

    fn get<'de, T>(&self, url: &str) -> Result<T>
        where T: ::serde::de::DeserializeOwned
        {

        let data =
            self.reqwest_client
            .get(url)?
            .send()?
            .json::<T>()?;
        Ok(data)
    }
}
