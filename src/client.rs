use reqwest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use data_encoding::HEXLOWER;

use std::collections::HashMap;

use errors::*;
use credentials::Credentials;
use types::{Currency, CurrencyPair, OpenedOrder, OpenOrder, CancelOrderResponse};
use helpers::{parse_response, nonce};
use converters::{convert_balances};

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


type AllOpenOrders = HashMap<CurrencyPair, Vec<OpenOrder>>;

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    credentials: Credentials
}

impl Client {
    pub fn new(credentials: Credentials) -> Result<Self> {
        let http_client = reqwest::Client::new()?;
        let client = Self { credentials, http_client };
        Ok(client)
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

    fn post<'de, T>(&self, body: String) -> Result<T>
        where T: ::serde::de::DeserializeOwned {

        let url = "https://poloniex.com/tradingApi";
        let sign = self.build_sign(&body);

        let resp = self.http_client
            .post(url)?
            .header(SignHeader(sign))
            .header(KeyHeader(self.credentials.key.to_owned()))
            .header(ContentHeader("application/x-www-form-urlencoded".to_owned()))
            .body(body)
            .send()?;
        parse_response(resp)
    }

    fn build_sign(&self, data: &str) -> String {
        let mut hmac = Hmac::new(Sha512::new(), self.credentials.secret.as_bytes());
        let bytes_data = data.as_bytes();
        hmac.input(bytes_data);
        HEXLOWER.encode(hmac.result().code())
    }
}
