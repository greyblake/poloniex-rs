use reqwest;
use chrono::prelude::*;

use std::collections::HashMap;
use std::ops::Range;

use errors::*;
use types::{Ticker, CurrencyPair, OrderBook, Period, ChartDataItem, Currency, LoanOrders, TradeHistoryItem, CurrencyInfo};
use helpers::parse_response;

type Tickers = HashMap<CurrencyPair, Ticker>;

/// `PublicClient` provides methods to make calls to Poloniex Public API.
/// It's based on `reqwest` HTTP client library, which allows to setup timeout
/// or proxy.
///
/// # Example
/// ```
/// use poloniex::PublicClient;
///
/// let client = PublicClient::new().unwrap();
/// let tickers = client.return_ticker().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct PublicClient {
    http_client: reqwest::Client
}

#[derive(Debug)]
pub struct PublicClientBuilder {
    http_client: Option<reqwest::Client>
}

impl PublicClientBuilder {
    fn new() -> Self {
        PublicClientBuilder {
            http_client: None
        }
    }

    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> Result<PublicClient> {
        let http_client = self.http_client.unwrap_or(reqwest::Client::new()?);
        let client = PublicClient { http_client };
        Ok(client)
    }
}


impl PublicClient {
    /// Constructs a new client for Public API.
    ///
    /// # Errors
    ///
    /// This method fails if native TLS backend cannot be created or initialized.
    ///
    /// # Example
    /// ```
    /// use poloniex::PublicClient;
    ///
    /// let client = PublicClient::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::new()?;
        let client = Self { http_client };
        Ok(client)
    }

    /// Creates `PublicClientBuilder` to build `PublicClient`
    pub fn builder() -> PublicClientBuilder {
        PublicClientBuilder::new()
    }


    pub fn return_ticker(&self) -> Result<Tickers> {
        self.get("command=returnTicker")
    }

    // Currency pair keys are mixed with "totalBTC", "totalETH", etc.
    // So it can not be parsed correctly
    //
    //pub fn return_24_volume(&self) -> Result<_> {
    //    self.get("https://poloniex.com/public?command=return24hVolume")
    //}

    pub fn return_order_book(&self, currency_pair: CurrencyPair, depth: u32) -> Result<OrderBook> {
        let query = format!("command=returnOrderBook&currencyPair={}&depth={}", currency_pair, depth);
        self.get(&query)
    }

    pub fn return_trade_history(&self, currency_pair: CurrencyPair, time_frame: Range<DateTime<Utc>>) -> Result<Vec<TradeHistoryItem>> {
        let start = time_frame.start.timestamp();
        let end = time_frame.end.timestamp();
        let query = format!("command=returnTradeHistory&currencyPair={}&start={}&end={}", currency_pair, start, end);
        self.get(&query)
    }

    pub fn return_chart_data(&self, currency_pair: CurrencyPair, period: Period, time_frame: Range<DateTime<Utc>>) -> Result<Vec<ChartDataItem>> {
        let start = time_frame.start.timestamp();
        let end = time_frame.end.timestamp();
        let query = format!("command=returnChartData&currencyPair={}&start={}&end={}&period={}", currency_pair, start, end, period);
        self.get(&query)
    }

    pub fn return_loan_orders(&self, currency: Currency) -> Result<LoanOrders> {
        let query = format!("command=returnLoanOrders&currency={}", currency);
        self.get(&query)
    }

    pub fn return_currencies(&self) -> Result<HashMap<Currency, CurrencyInfo>> {
        self.get("command=returnCurrencies")
    }

    fn get<'de, T>(&self, query: &str) -> Result<T>
        where T: ::serde::de::DeserializeOwned {

        let url = format!("https://poloniex.com/public?{}", query);
        let resp = self.http_client.get(&url)?.send()?;
        parse_response(resp)
    }
}
