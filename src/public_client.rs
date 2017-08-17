use reqwest;
use errors::*;

use types::{Ticker, CurrencyPair};

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
        let tickers =
            self.reqwest_client
            .get("https://poloniex.com/public?command=returnTicker")?
            .send()?
            .json::<Tickers>()?;

        Ok(tickers)
    }
}
