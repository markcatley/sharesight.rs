use std::{collections::HashMap, env, fmt, fs::File, io::Write, path::PathBuf};

use clap::Parser;
use heck::CamelCase;
use indexmap::IndexMap;
use log::{error, info, warn};
use serde::{de::IntoDeserializer, Deserialize};
use serde_with::{rust::StringWithSeparator, Separator};

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

#[derive(Debug, Deserialize)]
struct ApiData {
    api: Vec<ApiEndpoint>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiEndpoint {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    method: Method,
    url: String,
    #[allow(dead_code)]
    title: String,
    name: String,
    #[allow(dead_code)]
    group: String,
    version: String,
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    header: FieldsAndExamples,
    #[serde(default)]
    parameter: ApiParameters,
    #[serde(default)]
    #[allow(dead_code)]
    success: FieldsAndExamples,
    #[serde(default)]
    #[allow(dead_code)]
    error: FieldsAndExamples,
    #[allow(dead_code)]
    filename: String,
    #[allow(dead_code)]
    group_title: String,
}

impl fmt::Display for ApiEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameter_fields = &self.parameter.fields.parameter;
        if parameter_fields
            .iter()
            .any(|f| f.field.iter().any(|s| s.contains('/')))
        {
            warn!("Endpoint {} has parameter fields with / in them", self.name);
            return Ok(());
        }
        if parameter_fields.is_empty() {
            info!("Endpoint {} has no parameters", self.name);
            return Ok(());
        }

        let fields = parameter_fields
            .iter()
            .map(|f| {
                let (_, prefix) = f.field.split_last().unwrap();

                (prefix, f)
            })
            .fold(IndexMap::<_, Vec<_>>::new(), |mut acc, (k, v)| {
                acc.entry(k).or_default().push(v);
                acc
            });

        if !fields.contains_key(&[][..]) {
            warn!("Endpoint {} parameters have no root node", self.name);
            return Ok(());
        }

        let endpoint_name = self.name.to_camel_case();

        writeln!(f, "pub struct {};", endpoint_name)?;
        writeln!(f)?;

        let path = self
            .url
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let extension = s
                    .split('.')
                    .nth(1)
                    .map(|ext| format!(".{}", ext))
                    .unwrap_or_else(String::new);
                if s.starts_with(':') {
                    format!("{{}}{}", extension)
                } else {
                    s.to_string()
                }
            })
            .fold(String::new(), |x, y| x + "/" + &y);
        let params = self
            .url
            .split('/')
            .filter_map(|s| s.strip_prefix(':'))
            .filter_map(|s| s.split('.').next())
            .map(String::from)
            .collect::<Vec<_>>();

        writeln!(f, "impl<'a> ApiEndpoint<'a> for {} {{", endpoint_name)?;
        writeln!(f, "    const URL_PATH: &'static str = \"{}\";", self.url)?;
        writeln!(f)?;
        if params.is_empty() {
            writeln!(f, "    type UrlDisplay = &'static str;")?;
        } else {
            writeln!(f, "    type UrlDisplay = {}UrlDisplay<'a>;", endpoint_name)?;
        }
        writeln!(f, "    type Parameters = {}Parameters;", endpoint_name)?;
        writeln!(f)?;
        writeln!(
            f,
            "    fn url_path({}parameters: &'a Self::Parameters) -> Self::UrlDisplay {{",
            if params.is_empty() { "_" } else { "" }
        )?;
        if params.is_empty() {
            write!(f, "        \"{}\"", path)?;
        } else {
            write!(f, "        {}UrlDisplay(parameters)", endpoint_name)?;
        }
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        if !params.is_empty() {
            writeln!(
                f,
                "pub struct {}UrlDisplay<'a>(&'a {}Parameters);",
                endpoint_name, endpoint_name
            )?;
            writeln!(f)?;
            writeln!(
                f,
                "impl<'a> fmt::Display for {}UrlDisplay<'a> {{",
                endpoint_name
            )?;
            writeln!(
                f,
                "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
            )?;
            writeln!(f, "        let parameters = self.0;")?;
            writeln!(f)?;
            write!(f, "        write!(f, \"{}\"", path)?;
            for param in params {
                write!(f, ", parameters.{}", param)?;
            }
            writeln!(f, ")")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
            writeln!(f)?;
        }

        for (prefix, fields) in fields.into_iter() {
            writeln!(f, "#[derive(Debug, Clone, Serialize)]")?;
            writeln!(
                f,
                "pub struct {}{}Parameters {{",
                endpoint_name,
                prefix
                    .iter()
                    .map(|s| s.to_camel_case())
                    .fold(String::new(), |acc, ref item| acc + item)
            )?;
            for parameter in fields {
                if let [.., ref field_name] = parameter.field[..] {
                    write!(f, "    pub {}: ", field_name)?;
                    if parameter.optional {
                        write!(f, "Option<")?;
                    }
                    if parameter.field_type.is_hash() {
                        write!(
                            f,
                            "{}{}Parameters",
                            endpoint_name,
                            field_name.to_camel_case(),
                        )?;
                    } else {
                        write!(f, "{}", parameter.field_type.rust_type_name())?;
                    }
                    if parameter.optional {
                        write!(f, ">")?;
                    }
                    writeln!(f, ",")?;
                } else {
                    error!(
                        "Endpoint {} has parameter field with no field name: {:?} ",
                        endpoint_name, parameter
                    );
                }
            }
            writeln!(f, "}}")?;
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Method {
    #[serde(alias = "DELETE")]
    Delete,
    #[serde(alias = "GET")]
    Get,
    #[serde(alias = "POST")]
    Post,
    #[serde(alias = "PUT")]
    Put,
    #[serde(alias = "SHOW")]
    Show,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ApiParameters {
    fields: ApiParameterFields,
    examples: Vec<Example>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct ApiParameterFields {
    parameter: Vec<Field>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct FieldsAndExamples {
    fields: HashMap<String, Vec<Field>>,
    examples: Vec<Example>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Example {
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    content: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    example_type: ExampleType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Field {
    #[allow(dead_code)]
    group: String,
    #[serde(rename = "type", default = "FieldType::unit")]
    field_type: FieldType,
    optional: bool,
    #[serde(with = "StringWithSeparator::<PeriodSeparator>")]
    field: Vec<String>,
    #[allow(dead_code)]
    description: String,
}

#[derive(Debug)]
enum FieldType {
    Scalar(FieldTypeBase),
    Array(FieldTypeBase),
}

impl FieldType {
    fn unit() -> Self {
        FieldType::Scalar(FieldTypeBase::Unit)
    }

    fn rust_type_name(&self) -> FieldTypeRustTypeNameDisplay {
        FieldTypeRustTypeNameDisplay(self)
    }

    fn is_hash(&self) -> bool {
        matches!(self, FieldType::Scalar(FieldTypeBase::Hash))
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let field_type = String::deserialize(deserializer)?;

        if field_type == "Array" {
            Ok(FieldType::Array(FieldTypeBase::Unit))
        } else if field_type.ends_with("[]") {
            Ok(FieldType::Array(FieldTypeBase::deserialize(
                field_type.trim_end_matches("[]").into_deserializer(),
            )?))
        } else {
            Ok(FieldType::Scalar(FieldTypeBase::deserialize(
                field_type.into_deserializer(),
            )?))
        }
    }
}

struct FieldTypeRustTypeNameDisplay<'a>(&'a FieldType);

impl<'a> fmt::Display for FieldTypeRustTypeNameDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let FieldTypeRustTypeNameDisplay(field_type) = *self;

        match field_type {
            FieldType::Scalar(t) => write!(f, "{}", t.rust_type_name()),
            FieldType::Array(t) => write!(f, "Vec<{}>", t.rust_type_name()),
        }
    }
}

#[derive(Debug, Deserialize)]
enum FieldTypeBase {
    String,
    Hash,
    Integer,
    Date,
    Float,
    DateTime,
    #[serde(rename = "", alias = "Array")]
    Unit,
    File,
    Boolean,
    Group,
    Portfolio,
    Price,
    Instruments,
    Membership,
    Currency,
    #[serde(rename = "DRP_Trade")]
    DrpTrade,
    Payout,
    Parcel,
    PortfolioDiversity,
    PortfolioDiversityGroup,
    PortfolioDiversityHolding,
    CustomGroup,
    Market,
    IndustryClassification,
    SectorClassification,
    InvestmentType,
    Country,
    CustomGroupCategory,
    Object,
    PortfolioPerformance,
    Holding,
    CashAccount,
    SubTotal,
    PortfolioValuation,
    User,
}

impl FieldTypeBase {
    fn rust_type_name(&self) -> FieldTypeBaseRustTypeNameDisplay {
        FieldTypeBaseRustTypeNameDisplay(self)
    }
}

struct FieldTypeBaseRustTypeNameDisplay<'a>(&'a FieldTypeBase);

impl<'a> fmt::Display for FieldTypeBaseRustTypeNameDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let FieldTypeBaseRustTypeNameDisplay(field_type) = *self;

        match field_type {
            FieldTypeBase::String => write!(f, "String"),
            FieldTypeBase::Hash => unreachable!(),
            FieldTypeBase::Integer => write!(f, "i64"),
            FieldTypeBase::Date => write!(f, "NaiveDate"),
            FieldTypeBase::Float => write!(f, "f64"),
            FieldTypeBase::DateTime => write!(f, "NaiveDateTime"),
            FieldTypeBase::Unit => write!(f, "()"),
            FieldTypeBase::File => write!(f, "()"),
            FieldTypeBase::Boolean => write!(f, "bool"),
            FieldTypeBase::Group => write!(f, "()"),
            FieldTypeBase::Portfolio => write!(f, "()"),
            FieldTypeBase::Price => write!(f, "()"),
            FieldTypeBase::Instruments => write!(f, "()"),
            FieldTypeBase::Membership => write!(f, "()"),
            FieldTypeBase::Currency => write!(f, "()"),
            FieldTypeBase::DrpTrade => write!(f, "()"),
            FieldTypeBase::Payout => write!(f, "()"),
            FieldTypeBase::Parcel => write!(f, "()"),
            FieldTypeBase::PortfolioDiversity => write!(f, "()"),
            FieldTypeBase::PortfolioDiversityGroup => write!(f, "()"),
            FieldTypeBase::PortfolioDiversityHolding => write!(f, "()"),
            FieldTypeBase::CustomGroup => write!(f, "()"),
            FieldTypeBase::Market => write!(f, "()"),
            FieldTypeBase::IndustryClassification => write!(f, "()"),
            FieldTypeBase::SectorClassification => write!(f, "()"),
            FieldTypeBase::InvestmentType => write!(f, "()"),
            FieldTypeBase::Country => write!(f, "()"),
            FieldTypeBase::CustomGroupCategory => write!(f, "()"),
            FieldTypeBase::Object => write!(f, "()"),
            FieldTypeBase::PortfolioPerformance => write!(f, "()"),
            FieldTypeBase::Holding => write!(f, "()"),
            FieldTypeBase::CashAccount => write!(f, "()"),
            FieldTypeBase::SubTotal => write!(f, "()"),
            FieldTypeBase::PortfolioValuation => write!(f, "()"),
            FieldTypeBase::User => write!(f, "()"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ExampleType {
    Json,
}

fn main() -> anyhow::Result<()> {
    init_logger();

    let opt = Opt::parse();

    let ApiData { api: api_endpoints } =
        serde_json::from_reader::<_, ApiData>(File::open(opt.input)?)?;
    let mut f = File::create(opt.output)?;

    writeln!(f, "use crate::types_prelude::*;")?;
    writeln!(f)?;

    for api_endpoint in &api_endpoints {
        if let Some(ref only) = opt.only {
            if !only.iter().any(|name| name == &api_endpoint.name) {
                continue;
            }
        }
        if api_endpoint.version == "2.0.0" {
            write!(f, "{}", api_endpoint)?;
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

struct PeriodSeparator;

impl Separator for PeriodSeparator {
    fn separator() -> &'static str {
        "."
    }
}
