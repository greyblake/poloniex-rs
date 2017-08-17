use serde::de::{self, Deserialize};

pub fn string_to_f64<'de, D>(d: D) -> Result<f64, D::Error> where D: de::Deserializer<'de> {
    String::deserialize(d)?.parse().map_err(de::Error::custom)
}

pub fn string_number_to_bool<'de, D>(d: D) -> Result<bool, D::Error> where D: de::Deserializer<'de> {
    let num: i32 = String::deserialize(d)?.parse().map_err(de::Error::custom)?;
    Ok(num != 0)
}
