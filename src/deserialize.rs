use crate::types::U256;

use serde::{Deserialize, Deserializer};

pub fn de_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>
{
    let buf = String::deserialize(deserializer)?;
    if buf == "1" {
        return Ok(true);
    }
    else {
        return Ok(false);
    }
}

/// Look at example at https://serde.rs/stream-array.html
pub fn de_string_to_numeric<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + serde::Deserialize<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display // std::str::FromStr has `Err` type, see https://doc.rust-lang.org/std/str/trait.FromStr.html
{
    let buf = String::deserialize(deserializer)?;
    // convert into serde's custom Error type
    buf.parse::<T>().map_err(serde::de::Error::custom)
}

#[allow(non_snake_case)]
pub fn de_string_to_U256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>
{
    let buf = String::deserialize(deserializer)?;
    U256::from_dec_str(&buf).map_err(serde::de::Error::custom)
}
