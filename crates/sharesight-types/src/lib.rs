mod types;
mod types_prelude;

pub use types::*;
pub use types_prelude::ApiEndpoint;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
