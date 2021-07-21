generate:
	cargo run --bin sharesight-generate -- crates/sharesight-generate/assets/api_data.json crates/sharesight-types/src/types.rs
	cargo fmt
	cargo clippy

format-toml:
	npx -p prettier -p prettier-plugin-toml prettier --ignore-path .gitignore --write .
