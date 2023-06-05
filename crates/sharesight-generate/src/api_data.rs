use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{
    de::{IntoDeserializer, Unexpected},
    Deserialize,
};
use serde_with::{formats::Separator, serde_as, StringWithSeparator};

#[derive(Debug, Deserialize)]
pub struct ApiData {
    pub api: Vec<ApiEndpoint>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiEndpoint {
    #[serde(rename = "type")]
    pub method: Method,
    pub url: String,
    #[allow(dead_code)]
    pub title: String,
    pub name: String,
    #[allow(dead_code)]
    pub group: String,
    pub version: String,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub header: ApiHeaders,
    #[serde(default)]
    pub parameter: ApiParameters,
    #[serde(default)]
    pub success: ApiSuccess,
    #[serde(default)]
    #[allow(dead_code)]
    pub error: ApiErrors,
    #[allow(dead_code)]
    pub filename: String,
    #[allow(dead_code)]
    pub group_title: String,
}

impl ApiEndpoint {
    pub fn url_params(&self) -> Vec<String> {
        self.url
            .split('/')
            .filter_map(|s| s.strip_prefix(':'))
            .filter_map(|s| s.split('.').next())
            .map(String::from)
            .collect::<Vec<_>>()
    }

    pub fn fix(&mut self) {
        self.fix_url_params();
        self.fix_container_params();
        self.fix_dates();
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

    fn fix_dates(&mut self) {
        for field in self.success.fields.fields.iter_mut() {
            if matches!(field.field_type, FieldType::Scalar(FieldTypeBase::String))
                && field
                    .description
                    .contains("(format <code>YYYY-MM-DD</code>)")
            {
                field.field_type = FieldType::Scalar(FieldTypeBase::Date);
            }
        }
    }
}

pub fn group_fields_by_prefix(fields: &[Field]) -> IndexMap<&[String], Vec<&Field>> {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
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
    pub fn api_http_method(&self) -> &'static str {
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
pub struct ApiHeaders {
    pub fields: HashMap<String, Vec<Field>>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiParameters {
    pub fields: ApiParameterFields,
    pub examples: Vec<Example>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ApiParameterFields {
    pub parameter: Vec<Field>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ApiSuccess {
    pub fields: ApiSuccessField,
    pub examples: Vec<Example>,
}

#[derive(Debug, Default)]
pub struct ApiSuccessField {
    #[allow(dead_code)]
    pub status: ApiHttpStatus,
    pub fields: Vec<Field>,
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
pub struct ApiErrors {
    pub fields: HashMap<ApiHttpStatus, Vec<Field>>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiHttpStatus(u16, String);

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
pub struct Example {
    #[allow(dead_code)]
    pub title: String,
    #[allow(dead_code)]
    pub content: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub example_type: ExampleType,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[allow(dead_code)]
    pub group: String,
    #[serde(rename = "type", default = "FieldType::unit")]
    pub field_type: FieldType,
    pub optional: bool,
    #[serde_as(as = "StringWithSeparator::<PeriodSeparator, String>")]
    pub field: Vec<String>,
    #[allow(dead_code)]
    pub description: String,
}

#[derive(Debug)]
pub enum FieldType {
    Scalar(FieldTypeBase),
    Array(FieldTypeBase),
}

impl FieldType {
    fn unit() -> Self {
        FieldType::Scalar(FieldTypeBase::Unit)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, FieldType::Array(_))
    }

    pub fn is_hash(&self) -> bool {
        matches!(
            self,
            FieldType::Scalar(t) | FieldType::Array(t) if t.is_hash()
        )
    }

    pub fn is_string(&self) -> bool {
        matches!(
            self,
            FieldType::Scalar(FieldTypeBase::String) | FieldType::Array(FieldTypeBase::String)
        )
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let field_type = String::deserialize(deserializer)?;

        if field_type == "[]" {
            Ok(FieldType::Array(FieldTypeBase::Hash))
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

#[derive(Debug, Deserialize)]
pub enum FieldTypeBase {
    String,
    #[serde(alias = "Array")]
    Hash,
    Integer,
    Date,
    Float,
    DateTime,
    #[serde(rename = "")]
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
    pub fn is_hash(&self) -> bool {
        use FieldTypeBase::*;
        matches!(
            self,
            Hash | Group
                | Portfolio
                | Price
                | Instruments
                | Membership
                | Currency
                | DrpTrade
                | Payout
                | Parcel
                | PortfolioDiversity
                | PortfolioDiversityGroup
                | PortfolioDiversityHolding
                | CustomGroup
                | Market
                | IndustryClassification
                | SectorClassification
                | InvestmentType
                | Country
                | CustomGroupCategory
                | Object
                | PortfolioPerformance
                | Holding
                | CashAccount
                | SubTotal
                | PortfolioValuation
                | User
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExampleType {
    Json,
}

struct PeriodSeparator;

impl Separator for PeriodSeparator {
    fn separator() -> &'static str {
        "."
    }
}
