use reqwest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use data_encoding::HEXLOWER;
use chrono::prelude::*;

use std::collections::HashMap;
use std::ops::Range;

use errors::*;
use credentials::{Credentials, CredentialsBuilder, ApiKey, ApiSecret};
use types::{Ticker, CurrencyPair, OrderBook, Period, ChartDataItem, Currency, LoanOrders, TradeHistoryItem, CurrencyInfo, OpenedOrder, OpenOrder, CancelOrderResponse};
use helpers::{parse_response, nonce};
use converters::{convert_balances};

type Tickers = HashMap<CurrencyPair, Ticker>;
type AllOpenOrders = HashMap<CurrencyPair, Vec<OpenOrder>>;

header! {
    #[doc(hidden)]
    (KeyHeader, "Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "Sign") => [String]
}

header! {
    #[doc(hidden)]
    (ContentHeader, "Content-Type") => [String]
}

/// `Client` can call Trading API and Public API.
///
/// # Example
/// ```
/// use poloniex::Client;
///
/// let client = Client::builder()
///     .key("KEY")
///     .secret("SECRET")
///     .build()
///     .unwrap();
///
/// let tickers = client.return_ticker().unwrap();
/// let err = client.return_balances().unwrap_err();
/// assert_eq!(err.description(), "Invalid API key/secret pair.");
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    credentials: Credentials
}

#[derive(Debug)]
pub struct ClientBuilder {
    http_client: Option<reqwest::Client>,
    credentials_builder: CredentialsBuilder
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self { http_client: None, credentials_builder: CredentialsBuilder::new() }
    }

    pub fn key<T: Into<ApiKey>>(mut self, key: T) -> Self {
        self.credentials_builder = self.credentials_builder.key(key);
        self
    }

    pub fn secret<T: Into<ApiSecret>>(mut self, secret: T) -> Self {
        self.credentials_builder = self.credentials_builder.secret(secret);
        self
    }

    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> Result<Client> {
        let http_client = self.http_client.unwrap_or(reqwest::Client::new()?);
        let credentials = self.credentials_builder.build()?;
        let client = Client { http_client, credentials };
        Ok(client)
    }
}


impl Client {
    define_public_api_functions!();

    pub fn new(credentials: Credentials) -> Result<Self> {
        let http_client = reqwest::Client::new()?;
        let client = Self { credentials, http_client };
        Ok(client)
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn return_balances(&self) -> Result<HashMap<Currency, f64>> {
        let body = format!("command=returnBalances&nonce={}", nonce());
        convert_balances(self.post(body)?)
    }

    pub fn buy(&self, currency_pair: CurrencyPair, rate: f64, amount: f64) -> Result<OpenedOrder> {
        let data = format!("command=buy&currencyPair={}&rate={}&amount={}&nonce={}", currency_pair, rate, amount, nonce());
        self.post(data)
    }

    pub fn sell(&self, currency_pair: CurrencyPair, rate: f64, amount: f64) -> Result<OpenedOrder> {
        let data = format!("command=sell&currencyPair={}&rate={}&amount={}&nonce={}", currency_pair, rate, amount, nonce());
        self.post(data)
    }

    pub fn return_open_orders(&self, currency_pair: CurrencyPair) -> Result<Vec<OpenOrder>> {
        let data = format!("command=returnOpenOrders&currencyPair={}&nonce={}", currency_pair, nonce());
        self.post(data)
    }

    pub fn return_all_open_orders(&self) -> Result<AllOpenOrders> {
        let data = format!("command=returnOpenOrders&currencyPair=all&nonce={}", nonce());
        self.post(data)
            // Filter out empty entries
            .map({ |all_open_orders: AllOpenOrders|
                all_open_orders
                    .into_iter()
                    .filter(|&(_, ref orders)| !orders.is_empty() )
                    .collect()
            })
    }

    pub fn cancel_order(&self, order_number: &str) -> Result<CancelOrderResponse> {
        let data = format!("command=cancelOrder&orderNumber={}&nonce={}", order_number, nonce());
        self.post(data)
    }

    fn post<T>(&self, body: String) -> Result<T>
        where T: ::serde::de::DeserializeOwned {

        let url = "https://poloniex.com/tradingApi";
        let sign = self.build_sign(&body);

        let resp = self.http_client
            .post(url)?
            .header(SignHeader(sign))
            .header(KeyHeader(self.credentials.key.0.to_owned()))
            .header(ContentHeader("application/x-www-form-urlencoded".to_owned()))
            .body(body)
            .send()?;
        parse_response(resp)
    }

    fn build_sign(&self, data: &str) -> String {
        let mut hmac = Hmac::new(Sha512::new(), self.credentials.secret.0.as_bytes());
        let bytes_data = data.as_bytes();
        hmac.input(bytes_data);
        HEXLOWER.encode(hmac.result().code())
    }
}
