extern crate poloniex3;
extern crate reqwest;

use poloniex3::errors::*;

pub struct PublicClient {
    reqwest_client: reqwest::Client
}


impl PublicClient {
    fn new() -> Result<Self> {
        let reqwest_client = reqwest::Client::new()?;
        let client = Self { reqwest_client };
        Ok(client)
    }

    //pub fn return_ticker(&self) -> Result {
    //    let url = format!("https://poloniex.com/public?command=returnTicker");
    //    self.http_client.get(&url).send()
    //}
}

fn main() {
    let hello = 12;
    println!("hello = {:?}", hello);
}
