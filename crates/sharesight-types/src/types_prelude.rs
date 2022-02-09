pub use chrono::{NaiveDate, NaiveDateTime};
pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, DisplayFromStr, PickFirst};
use serde_with::{DeserializeAs, SerializeAs};

pub use std::fmt;

pub enum ApiHttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

pub trait ApiEndpoint<'a> {
    const URL_PATH: &'static str;
    const HTTP_METHOD: ApiHttpMethod;

    type UrlDisplay: 'a + fmt::Display;
    type Parameters: Serialize;
    type Success: DeserializeOwned;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay;
}

pub struct DeserializeDate;

impl<'de> DeserializeAs<'de, NaiveDate> for DeserializeDate {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        NaiveDate::parse_from_str(&s, "%d %b %Y").map_err(serde::de::Error::custom)
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
