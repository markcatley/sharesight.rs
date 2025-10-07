use std::{collections::BTreeMap, env, fs::File, io::Write, path::PathBuf, str::FromStr};

mod api_data;
mod display;

use api_data::{ApiData, ApiEndpoint};
use clap::Parser;
use display::ApiEndpointStruct;
use indexmap::IndexMap;
use log::info;
use loose_semver::Version;

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

    let mut by_name_and_version = IndexMap::<String, BTreeMap<Version, ApiEndpoint>>::new();

    for mut api_endpoint in api_endpoints {
        api_endpoint.fix();

        if api_endpoint.version.ends_with("-internal") || api_endpoint.version.ends_with("-mobile")
        {
            continue;
        }

        if [
            "Performance_Index_Chart",
            "PerformanceShow",
            "HoldingPortfolioList",
            "HoldingShow",
            "HoldingList",
            "HoldingDelete",
            "Custom_InvestmentUpdate",
            "Custom_Investment_PriceShow",
        ]
        .contains(&api_endpoint.name.as_str())
        {
            continue;
        }

        let version = api_endpoint.version.parse::<Version>()?;

        let by_version = by_name_and_version
            .entry(api_endpoint.name.clone())
            .or_default();
        by_version.insert(version, api_endpoint);
    }

    for api_endpoint in by_name_and_version.into_values().filter_map(|mut v| {
        v.remove(&Version::from_str("3.0.0").unwrap())
            .or_else(|| v.remove(&Version::from_str("2.0.0").unwrap()))
            .or_else(|| v.remove(&Version::from_str("2.1.0").unwrap()))
            .or_else(|| v.into_values().next_back())
    }) {
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
