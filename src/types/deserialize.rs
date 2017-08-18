use serde::de::{self, Deserialize};
use chrono::naive::NaiveDateTime;
use chrono::prelude::*;

pub fn string_to_f64<'de, D>(d: D) -> Result<f64, D::Error>
    where D: de::Deserializer<'de> {

    String::deserialize(d)?.parse().map_err(de::Error::custom)
}

pub fn string_number_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
    where D: de::Deserializer<'de> {

    let num: i32 = String::deserialize(d)?.parse().map_err(de::Error::custom)?;
    Ok(num != 0)
}

pub fn number_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
    where D: de::Deserializer<'de> {

    let num = u32::deserialize(d)?;
    Ok( num != 0)
}

pub fn string_to_utc_datetime<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where D: de::Deserializer<'de> {

    String::deserialize(d).and_then(|s| {
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(de::Error::custom)
            .map(|naive_dt| DateTime::<Utc>::from_utc(naive_dt, Utc) )
    })
}
