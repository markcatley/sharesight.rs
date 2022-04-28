use std::{env, fs::File, io::Write, path::PathBuf};

mod api_data;
mod display;

use api_data::ApiData;
use clap::Parser;
use display::ApiEndpointStruct;
use log::info;

/// Generate sharesight types from the swagger manifest
#[derive(Debug, Parser)]
struct Opt {
    /// The swagger manifest json file
    input: PathBuf,

    /// The rust file generate. The file will be overwritten if it already exists
    output: PathBuf,

    /// Only generate the endpoints with the names listed
    #[clap(long, short)]
    only: Option<Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let opt = Opt::parse();

    let ApiData {
        api: mut api_endpoints,
    } = serde_json::from_reader::<_, ApiData>(File::open(opt.input)?)?;
    let mut f = File::create(opt.output)?;

    writeln!(f, "use crate::types_prelude::*;")?;
    writeln!(f)?;

    for api_endpoint in &mut api_endpoints {
        api_endpoint.fix();

        if let Some(ref only) = opt.only {
            if !only.iter().any(|name| name == &api_endpoint.name) {
                continue;
            }
        }
        if api_endpoint.version == "2.0.0" {
            write!(f, "{}", ApiEndpointStruct(api_endpoint))?;
        } else {
            info!("Skipping {} ({})", api_endpoint.name, api_endpoint.version);
        }
    }

    Ok(())
}

fn init_logger() {
    if Err(env::VarError::NotPresent) == env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }

    env_logger::init();
}
