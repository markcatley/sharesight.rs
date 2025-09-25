use serde::de::Unexpected;
use serde_with::{DeserializeAs, SerializeAs};

pub use std::fmt;

pub use chrono::{DateTime, FixedOffset, NaiveDate};
pub use serde::de::{DeserializeOwned, Visitor};
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, DefaultOnNull, DisplayFromStr, PickFirst};

pub use crate::codes::*;

#[cfg(all(feature = "rust_decimal", not(feature = "bigdecimal")))]
pub type Number = rust_decimal::Decimal;
#[cfg(all(feature = "bigdecimal", not(feature = "rust_decimal")))]
pub type Number = bigdecimal::BigDecimal;
#[cfg(all(not(feature = "bigdecimal"), not(feature = "rust_decimal")))]
pub type Number = f64;
#[cfg(all(feature = "rust_decimal", feature = "bigdecimal"))]
compile_error!(
    "sharesight: Features rust_decimal and bigdecimal are mutually exclusive. Pick one."
);

pub enum ApiHttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

pub trait ApiEndpoint<'a> {
    const URL_PATH: &'static str;
    const HTTP_METHOD: ApiHttpMethod;
    const VERSION: &'static str;

    type UrlDisplay: 'a + fmt::Display;
    type Parameters: Serialize;
    type Success: DeserializeOwned;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay;

    fn url(api_host: &'a str, parameters: &'a Self::Parameters) -> ApiUrl<'a, Self> {
        ApiUrl(api_host, parameters, Self::VERSION)
    }
}

pub struct ApiUrl<'a, T: ApiEndpoint<'a> + ?Sized>(&'a str, &'a T::Parameters, &'a str);

impl<'a, T: ApiEndpoint<'a>> fmt::Display for ApiUrl<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(api_host, parameters, version) = self;

        let version = if version.starts_with("2.1.") {
            "v2.1"
        } else if version.starts_with("2.") {
            "v2"
        } else {
            "v3"
        };
        write!(
            f,
            "https://{}/api/{}{}",
            api_host,
            version,
            T::url_path(parameters)
        )
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

pub struct DeserializeNumber;

impl<'de> DeserializeAs<'de, Number> for DeserializeNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<T: serde::Serialize> SerializeAs<T> for DeserializeNumber {
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        source.serialize(serializer)
    }
}

pub struct NumberVisitor;

impl<'de> Visitor<'de> for NumberVisitor {
    type Value = Number;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sharesight api number")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        #[cfg(any(feature = "rust_decimal", feature = "bigdecimal"))]
        let result = v
            .try_into()
            .map_err(|_| serde::de::Error::invalid_type(Unexpected::Float(v), &self));
        #[cfg(not(any(feature = "rust_decimal", feature = "bigdecimal")))]
        let result = Ok(v);

        result
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        #[cfg(all(feature = "rust_decimal", not(feature = "bigdecimal")))]
        let infinities = Some((Number::MAX, Number::MIN));
        #[cfg(all(feature = "bigdecimal", not(feature = "rust_decimal")))]
        let infinities = None;
        #[cfg(all(not(feature = "rust_decimal"), not(feature = "bigdecimal")))]
        let infinities = Some((Number::INFINITY, Number::NEG_INFINITY));

        match (v, infinities) {
            ("Infinity", Some((infinity, _))) => Ok(infinity),
            ("-Infinity", Some((_, neg_infinity))) => Ok(neg_infinity),
            _ => Err(serde::de::Error::invalid_type(Unexpected::Str(v), &self)),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct Auth {
    access_token: String,
    expires_in: u32,
    refresh_token: Option<String>,
    created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct AuthWithHost {
    #[serde(flatten)]
    auth: Auth,
    host: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(i64),
    Name(String),
}

impl IdOrName {
    pub fn id(&self) -> Option<i64> {
        match self {
            IdOrName::Id(id) => Some(*id),
            IdOrName::Name(_) => None,
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            IdOrName::Name(name) => Some(name.to_string()),
            IdOrName::Id(_) => None,
        }
    }
}

#[cfg(test)]
mod id_or_name_tests {
    use super::IdOrName;
    use serde::{
        de::{value::Error, IntoDeserializer},
        Deserialize,
    };

    #[test]
    fn serialize() -> Result<(), serde_json::Error> {
        use serde_json::{json, to_value};

        assert_eq!(json!("name"), to_value(IdOrName::Name("name".to_string()))?);
        assert_eq!(json!(0), to_value(IdOrName::Id(0))?);

        Ok(())
    }

    #[test]
    fn deserialize() -> Result<(), Error> {
        assert_eq!(
            IdOrName::Id(0),
            IdOrName::deserialize(0.into_deserializer())?
        );

        assert_eq!(
            IdOrName::Name("name".to_string()),
            IdOrName::deserialize("name".to_string().into_deserializer())?
        );

        Ok(())
    }
}
