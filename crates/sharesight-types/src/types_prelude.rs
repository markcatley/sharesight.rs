pub use chrono::{NaiveDate, NaiveDateTime};
pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{serde_as, DisplayFromStr, PickFirst};

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
