use std::{collections::BTreeMap, env, fs::File, io::Write, path::PathBuf};

mod api_data;
mod display;

use api_data::{ApiData, ApiEndpoint};
use clap::Parser;
use display::ApiEndpointStruct;
use indexmap::IndexMap;
use log::info;

/// Generate sharesight types from the swagger manifest
#[derive(Debug, Parser)]
struct Opt {
    /// The rust file generate. The file will be overwritten if it already exists
    output: PathBuf,

    /// The swagger manifest json files
    input: Vec<PathBuf>,

    /// Only generate the endpoints with the names listed
    #[clap(long, short)]
    only: Option<Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let opt = Opt::parse();

    let mut f = File::create(opt.output)?;
    let mut api_endpoints = Vec::<ApiEndpoint>::new();

    for input in &opt.input {
        info!("Reading {:?}", input);

        let ApiData { api: endpoints } = serde_json::from_reader::<_, ApiData>(File::open(input)?)?;
        api_endpoints.extend(endpoints.into_iter());
    }

    writeln!(f, "use crate::types_prelude::*;")?;
    writeln!(f)?;

    let mut by_name_and_version = IndexMap::<String, BTreeMap<String, ApiEndpoint>>::new();

    for mut api_endpoint in api_endpoints {
        api_endpoint.fix();

        let by_version = by_name_and_version
            .entry(api_endpoint.name.clone())
            .or_default();
        by_version.insert(api_endpoint.version.clone(), api_endpoint);
    }

    for api_endpoint in by_name_and_version
        .values_mut()
        .filter_map(|v| v.remove("2.0.0"))
    {
        if let Some(ref only) = opt.only {
            if !only.iter().any(|name| name == &api_endpoint.name) {
                continue;
            }
        }

        write!(f, "{}", ApiEndpointStruct(&api_endpoint))?;
    }

    Ok(())
}

fn init_logger() {
    if Err(env::VarError::NotPresent) == env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }

    env_logger::init();
}
