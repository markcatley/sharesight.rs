mod auth_types;
mod codes;
mod types;
mod types_prelude;

use std::fmt;

use serde::{de::DeserializeOwned, Serialize};

pub use auth_types::*;
pub use codes::*;
pub use types::*;

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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(i64),
    Name(String),
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
