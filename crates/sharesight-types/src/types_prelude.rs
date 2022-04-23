pub use chrono::{DateTime, FixedOffset, NaiveDate};
pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, DisplayFromStr, PickFirst};
use serde_with::{DeserializeAs, SerializeAs};

pub use std::fmt;

#[cfg(feature = "bigdecimal")]
pub type Float = bigdecimal::BigDecimal;
#[cfg(not(feature = "bigdecimal"))]
pub type Float = f64;

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

    fn url(api_host: &'a str, parameters: &'a Self::Parameters) -> ApiUrl<'a, Self> {
        ApiUrl(api_host, parameters)
    }
}

pub struct ApiUrl<'a, T: ApiEndpoint<'a> + ?Sized>(&'a str, &'a T::Parameters);

impl<'a, T: ApiEndpoint<'a>> fmt::Display for ApiUrl<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(api_host, parameters) = self;

        write!(f, "https://{}/api/v2{}", api_host, T::url_path(parameters))
    }
}

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Auth {
    access_token: String,
    expires_in: u32,
    refresh_token: Option<String>,
    created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthWithHost {
    #[serde(flatten)]
    auth: Auth,
    host: String,
}
