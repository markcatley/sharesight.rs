mod auth_types;
mod codes;
mod types;
mod types_prelude;

pub use auth_types::*;
pub use codes::*;
pub use types::*;
pub use types_prelude::{ApiEndpoint, ApiHttpMethod};

pub const DEFAULT_API_HOST: &str = "api.sharesight.com";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
