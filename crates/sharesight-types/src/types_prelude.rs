use serde_with::{DeserializeAs, SerializeAs};

pub use std::fmt;

pub use chrono::{DateTime, FixedOffset, NaiveDate};
pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, DisplayFromStr, PickFirst};

pub use crate::{codes::*, ApiEndpoint, ApiHttpMethod, IdOrName};

#[cfg(feature = "bigdecimal")]
pub type Float = bigdecimal::BigDecimal;
#[cfg(not(feature = "bigdecimal"))]
pub type Float = f64;

pub struct DeserializeDate;

impl<'de> DeserializeAs<'de, NaiveDate> for DeserializeDate {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .or_else(|_| NaiveDate::parse_from_str(&s, "%d %b %Y"))
            .map_err(serde::de::Error::custom)
    }
}

impl<T: serde::Serialize> SerializeAs<T> for DeserializeDate {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        source.serialize(serializer)
    }
}
