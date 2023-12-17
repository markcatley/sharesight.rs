generate:
	cargo run --bin sharesight-generate -- crates/sharesight-types/src/types.rs crates/sharesight-generate/assets/api_data_*.json
	cargo fmt
	cargo clippy

format-toml:
	npx -p prettier -p prettier-plugin-toml prettier --ignore-path .gitignore --write .
