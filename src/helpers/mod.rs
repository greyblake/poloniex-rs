use serde_json;
use reqwest;

use std::io::Read;

use errors::*;
use types::ErrorMessage;

pub fn parse_response<'de, T>(mut resp: reqwest::Response) -> Result<T>
    where T: ::serde::de::DeserializeOwned {

    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    parse_json::<T>(&content)
}

fn parse_json<'de, T>(s: &str) -> Result<T>
    where T: ::serde::de::DeserializeOwned {

    match serde_json::from_str::<T>(s) {
        Ok(data) => Ok(data),
        Err(e) => {
            match serde_json::from_str::<ErrorMessage>(s) {
                Ok(error_message) => {
                    let err_kind = ErrorKind::Msg(error_message.error);
                    Err(Error::from_kind(err_kind))
                },
                Err(_) => {
                    Err(Error::from(e))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::OrderBook;

    #[test]
    fn test_deserialize_struct() {
        let json = r#"
            {
                "asks":[],
                "bids":[],
                "isFrozen":"0",
                "seq":375042151
            }
        "#;
        let order_book: OrderBook = parse_json(json).unwrap();
        assert_eq!(order_book.seq, 375042151);
    }

    #[test]
    fn test_deserialize_error_message() {
        let json = r#"
            {
                "error":"Oops"
            }
        "#;
        let res: Result<OrderBook> = parse_json(json);
        let err = res.unwrap_err();
        assert_eq!(err.description(), "Oops");
    }
}
