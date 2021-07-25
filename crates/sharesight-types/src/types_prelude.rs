pub use chrono::{NaiveDate, NaiveDateTime};
pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};

pub use std::fmt;

pub trait ApiEndpoint<'a> {
    const URL_PATH: &'static str;

    type UrlDisplay: 'a + fmt::Display;
    type Parameters: Serialize;
    type Success: DeserializeOwned;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay;
}
