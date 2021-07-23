pub use chrono::{NaiveDate, NaiveDateTime};
pub use serde::Serialize;

pub use std::fmt;

pub trait ApiEndpoint<'a> {
    const URL_PATH: &'static str;

    type UrlDisplay: 'a + fmt::Display;
    type Parameters: Serialize;

    fn url_path(parameters: &'a Self::Parameters) -> Self::UrlDisplay;
}
