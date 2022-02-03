use std::{collections::HashMap, env, fmt, fs::File, io::Write, path::PathBuf};

use clap::Parser;
use heck::CamelCase;
use indexmap::IndexMap;
use log::{error, info, warn};
use serde::{
    de::{IntoDeserializer, Unexpected},
    Deserialize,
};
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
    header: ApiHeaders,
    #[serde(default)]
    parameter: ApiParameters,
    #[serde(default)]
    success: ApiSuccess,
    #[serde(default)]
    #[allow(dead_code)]
    error: ApiErrors,
    #[allow(dead_code)]
    filename: String,
    #[allow(dead_code)]
    group_title: String,
}

impl ApiEndpoint {
    fn url_params(&self) -> Vec<String> {
        self.url
            .split('/')
            .filter_map(|s| s.strip_prefix(':'))
            .filter_map(|s| s.split('.').next())
            .map(String::from)
            .collect::<Vec<_>>()
    }

    fn fix(&mut self) {
        self.fix_url_params();
        self.fix_container_params();
    }

    fn fix_url_params(&mut self) {
        for url_param in self.url_params() {
            let parameter_fields = &mut self.parameter.fields.parameter;

            if !parameter_fields
                .iter()
                .any(|f| matches!(&f.field[..], [name] if name == &url_param))
            {
                parameter_fields.push(Field {
                    group: String::new(),
                    field_type: FieldType::Scalar(FieldTypeBase::Integer),
                    optional: false,
                    field: vec![url_param.clone()],
                    description: String::new(),
                });
            }
        }
    }

    fn fix_container_params(&mut self) {
        let parameter_fields = &self.parameter.fields.parameter;
        let mut new_fields = Vec::new();

        for field in parameter_fields {
            if let Some(field) = field.field.get(0) {
                if !parameter_fields
                    .iter()
                    .any(|f| matches!(&f.field[..], [name] if name == field))
                    && !new_fields
                        .iter()
                        .any(|f: &Field| matches!(&f.field[..], [name] if name == field))
                {
                    new_fields.push(Field {
                        group: String::new(),
                        field_type: FieldType::Scalar(FieldTypeBase::Hash),
                        optional: false,
                        field: vec![field.clone()],
                        description: String::new(),
                    })
                }
            }
        }

        let parameter_fields = &mut self.parameter.fields.parameter;
        parameter_fields.extend(new_fields);
    }
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

        let parameter_fields = group_fields_by_prefix(parameter_fields);

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
        let params = self.url_params();

        writeln!(f, "impl<'a> ApiEndpoint<'a> for {} {{", endpoint_name)?;
        writeln!(f, "    const URL_PATH: &'static str = \"{}\";", self.url)?;
        writeln!(
            f,
            "    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::{};",
            self.method.api_http_method()
        )?;
        writeln!(f)?;
        if params.is_empty() {
            writeln!(f, "    type UrlDisplay = &'static str;")?;
        } else {
            writeln!(f, "    type UrlDisplay = {}UrlDisplay<'a>;", endpoint_name)?;
        }
        write!(f, "    type Parameters = ")?;
        if parameter_fields.is_empty() {
            write!(f, "()")?;
        } else {
            write!(f, "{}Parameters", endpoint_name)?;
        }
        writeln!(f, ";")?;
        if self.success.fields.fields.is_empty() {
            writeln!(f, "    type Success = ();")?;
        } else {
            writeln!(f, "    type Success = {}Success;", endpoint_name)?;
        }
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

        if !parameter_fields.is_empty() {
            writeln!(
                f,
                "{}",
                ApiStruct::parameters(&endpoint_name, &parameter_fields)
            )?;
        }
        writeln!(f)?;

        let success_fields = group_fields_by_prefix(&self.success.fields.fields);

        writeln!(f, "{}", ApiStruct::success(&endpoint_name, &success_fields))?;
        writeln!(f)?;

        Ok(())
    }
}

fn group_fields_by_prefix(fields: &[Field]) -> IndexMap<&[String], Vec<&Field>> {
    fields
        .iter()
        .map(|f| {
            let (_, prefix) = f.field.split_last().unwrap();

            (prefix, f)
        })
        .fold(IndexMap::<_, Vec<_>>::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        })
}

struct ApiStruct<'a> {
    tag: &'a str,
    label: &'a str,
    endpoint_name: &'a str,
    fields: &'a IndexMap<&'a [std::string::String], Vec<&'a Field>>,
    derives: &'a [&'static str],
}

impl<'a> ApiStruct<'a> {
    fn parameters(
        endpoint_name: &'a str,
        fields: &'a IndexMap<&'a [std::string::String], Vec<&'a Field>>,
    ) -> Self {
        ApiStruct {
            tag: "Parameters",
            label: "parameter",
            endpoint_name,
            fields,
            derives: &["Serialize"],
        }
    }

    fn success(
        endpoint_name: &'a str,
        fields: &'a IndexMap<&'a [std::string::String], Vec<&'a Field>>,
    ) -> Self {
        ApiStruct {
            tag: "Success",
            label: "success",
            endpoint_name,
            fields,
            derives: &["Deserialize"],
        }
    }
}

impl<'a> fmt::Display for ApiStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ApiStruct {
            tag,
            label,
            endpoint_name,
            fields,
            derives,
        } = *self;

        for (prefix, fields) in fields.iter() {
            write!(f, "#[serde_as]")?;
            write!(f, "#[derive(Debug, Clone")?;
            for derive in derives {
                write!(f, ", {}", derive)?;
            }
            writeln!(f, ")]")?;
            write!(f, "pub struct {}", endpoint_name)?;
            for prefix_segment in prefix.iter() {
                write!(f, "{}", prefix_segment.to_camel_case())?;
            }
            writeln!(f, "{} {{", tag)?;
            for parameter in fields {
                if let [ref prefix_segments @ .., ref field_name] = parameter.field[..] {
                    if field_name == "self" {
                        writeln!(f, "    #[serde(rename = \"self\")]")?;
                    }

                    if matches!(
                        parameter.field_type,
                        FieldType::Scalar(FieldTypeBase::Integer)
                    ) {
                        if parameter.optional {
                            writeln!(
                                f,
                                "    #[serde_as(as = \"Option<PickFirst<(_, DisplayFromStr)>>\")]"
                            )?;
                        } else {
                            writeln!(
                                f,
                                "    #[serde_as(as = \"PickFirst<(_, DisplayFromStr)>\")]"
                            )?;
                        }
                    }

                    if matches!(parameter.field_type, FieldType::Scalar(FieldTypeBase::Date)) {
                        if parameter.optional {
                            writeln!(f, "    #[serde_as(as = \"Option<DeserializeDate>\")]")?;
                        } else {
                            writeln!(f, "    #[serde_as(as = \"DeserializeDate\")]")?;
                        }
                    }

                    if parameter.optional {
                        writeln!(f, "    #[serde(default)]")?;
                    }

                    write!(
                        f,
                        "    pub {}: ",
                        if field_name == "self" {
                            "itself"
                        } else {
                            field_name
                        }
                    )?;
                    if parameter.optional {
                        write!(f, "Option<")?;
                    }
                    if parameter.field_type.is_hash() {
                        write!(f, "{}", endpoint_name)?;
                        for prefix_segment in prefix_segments.iter() {
                            write!(f, "{}", prefix_segment.to_camel_case())?;
                        }
                        write!(f, "{}{}", field_name.to_camel_case(), tag)?;
                    } else {
                        write!(f, "{}", parameter.field_type.rust_type_name())?;
                    }
                    if parameter.optional {
                        write!(f, ">")?;
                    }
                    writeln!(f, ",")?;
                } else {
                    error!(
                        "Endpoint {} has {} field with no field name: {:?} ",
                        endpoint_name, label, parameter
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

impl Method {
    fn api_http_method(&self) -> &'static str {
        match self {
            Method::Delete => "Delete",
            Method::Get | Method::Show => "Get",
            Method::Post => "Post",
            Method::Put => "Put",
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ApiHeaders {
    fields: HashMap<String, Vec<Field>>,
    examples: Vec<Example>,
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
struct ApiSuccess {
    fields: ApiSuccessField,
    examples: Vec<Example>,
}

#[derive(Debug, Default)]
struct ApiSuccessField {
    #[allow(dead_code)]
    status: ApiHttpStatus,
    fields: Vec<Field>,
}

impl<'de> Deserialize<'de> for ApiSuccessField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let map = HashMap::<ApiHttpStatus, Vec<Field>>::deserialize(deserializer)?;

        if map.len() != 1 {
            return Err(D::Error::invalid_length(map.len(), &"1"));
        }

        let (status, fields) = map.into_iter().next().unwrap();

        Ok(ApiSuccessField { status, fields })
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ApiErrors {
    fields: HashMap<ApiHttpStatus, Vec<Field>>,
    examples: Vec<Example>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ApiHttpStatus(u16, String);

impl<'de> Deserialize<'de> for ApiHttpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        let status = s
            .find(' ')
            .map(|loc| (s[..loc].parse(), s[(loc + 1)..].to_string()));

        if let Some((Ok(code), name)) = status {
            Ok(Self(code, name))
        } else {
            Err(D::Error::invalid_value(
                Unexpected::Str(&s),
                &"<u16:code> <str:label>",
            ))
        }
    }
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
