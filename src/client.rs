use reqwest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use data_encoding::HEXLOWER;

use std::collections::HashMap;

use errors::*;
use credentials::Credentials;
use types::{Currency};

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

#[derive(Debug, Clone)]
pub struct Client {
    reqwest_client: reqwest::Client,
    credentials: Credentials
}

impl Client {
    pub fn new(credentials: Credentials) -> Result<Self> {
        let reqwest_client = reqwest::Client::new()?;
        let client = Self { credentials, reqwest_client };
        Ok(client)
    }

    pub fn return_balances(&self) -> Result<HashMap<Currency, String>> {
        let body = format!("command=returnBalances&nonce={}", nonce());
        self.post(body)
    }

    fn post<'de, T>(&self, body: String) -> Result<T>
        where T: ::serde::de::DeserializeOwned {

        let url = "https://poloniex.com/tradingApi";
        let sign = self.build_sign(&body);

        let data = self.reqwest_client
            .post(url)?
            .header(SignHeader(sign))
            .header(KeyHeader(self.credentials.key.to_owned()))
            .header(ContentHeader("application/x-www-form-urlencoded".to_owned()))
            .body(body)
            .send()?
            .json::<T>()?;
        Ok(data)
    }

    fn build_sign(&self, post_data: &str) -> String {
        let mut hmac = Hmac::new(Sha512::new(), self.credentials.secret.as_bytes());
        let data = post_data.as_bytes();
        hmac.input(data);
        HEXLOWER.encode(hmac.result().code())
    }
}

fn nonce() -> String {
    let current_time = ::time::get_time();
    ((current_time.sec as i64 * 1000_000_000) + (current_time.nsec as i64)).to_string()
}


