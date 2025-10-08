use std::fmt;

use heck::ToUpperCamelCase;
use indexmap::IndexMap;
use log::{error, warn};

use crate::api_data::{group_fields_by_prefix, ApiEndpoint, Field, FieldType, FieldTypeBase};

pub struct ApiEndpointStruct<'a>(pub &'a ApiEndpoint);

impl<'a> fmt::Display for ApiEndpointStruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(data) = self;

        let parameter_fields = &data.parameter.fields.parameter;
        if parameter_fields
            .iter()
            .any(|f| f.field.iter().any(|s| s.contains('/')))
        {
            warn!("Endpoint {} has parameter fields with / in them", data.name);
            return Ok(());
        }

        let parameter_fields = group_fields_by_prefix(parameter_fields);

        let endpoint_name = data.name.to_upper_camel_case();

        write!(f, "{}", DocComment(&data.description))?;
        writeln!(f, "pub struct {};", endpoint_name)?;
        writeln!(f)?;

        let path = data
            .url
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let extension = s
                    .split('.')
                    .nth(1)
                    .map(|ext| format!(".{}", ext))
                    .unwrap_or_default();
                if s.starts_with([':', '{']) {
                    format!("{{}}{}", extension)
                } else {
                    s.to_string()
                }
            })
            .fold(String::new(), |x, y| x + "/" + &y);
        let params = data.url_params();

        writeln!(f, "impl<'a> ApiEndpoint<'a> for {} {{", endpoint_name)?;
        writeln!(f, "    const URL_PATH: &'static str = \"{}\";", data.url)?;
        writeln!(
            f,
            "    const HTTP_METHOD: ApiHttpMethod = ApiHttpMethod::{};",
            data.method.api_http_method()
        )?;
        writeln!(f, "    const VERSION: &'static str = \"{}\";", data.version)?;
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
        if data.success.is_empty() {
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

        let success_fields = group_fields_by_prefix(data.success.api_fields());

        writeln!(f, "{}", ApiStruct::success(&endpoint_name, &success_fields))?;
        writeln!(f)?;

        Ok(())
    }
}

pub struct ApiStruct<'a> {
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
                write!(f, "{}", prefix_segment.to_upper_camel_case())?;
            }
            writeln!(f, "{} {{", tag)?;
            for parameter in fields {
                if let [ref prefix_segments @ .., ref field_name] = parameter.field[..] {
                    if field_name == "supported_denominations" {
                        continue;
                    }

                    write!(f, "{}", DocComment(&parameter.description))?;

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

                    if matches!(
                        parameter.field_type,
                        FieldType::Scalar(FieldTypeBase::Number)
                    ) {
                        if parameter.optional {
                            writeln!(f, "    #[serde_as(as = \"Option<DeserializeNumber>\")]")?;
                        } else {
                            writeln!(f, "    #[serde_as(as = \"DeserializeNumber\")]")?;
                        }
                    }

                    if parameter.optional {
                        writeln!(f, "    #[serde(default)]")?;
                    } else if parameter.field_type.is_string()
                        && string_enum_type(field_name, endpoint_name).is_none()
                    {
                        writeln!(f, "    #[serde(default)]")?;
                        writeln!(f, "    #[serde_as(deserialize_as = \"DefaultOnNull\")]")?;
                    }

                    write!(
                        f,
                        "    pub {}: ",
                        if field_name == "self" {
                            "itself"
                        } else if field_name == "type" {
                            "r#type"
                        } else {
                            field_name
                        }
                    )?;
                    if parameter.optional {
                        write!(f, "Option<")?;
                    }
                    if parameter.field_type.is_hash() {
                        if parameter.field_type.is_array() {
                            write!(f, "Vec<")?;
                        }
                        if field_name == "cash_account_transaction_type" {
                            write!(f, "CashAccountTransactionType")?;
                        } else {
                            write!(f, "{}", endpoint_name)?;
                            for prefix_segment in prefix_segments.iter() {
                                write!(f, "{}", prefix_segment.to_upper_camel_case())?;
                            }
                            write!(f, "{}{}", field_name.to_upper_camel_case(), tag)?;
                        }
                        if parameter.field_type.is_array() {
                            write!(f, ">")?;
                        }
                    } else if parameter.field_type.is_string()
                        && string_enum_type(field_name, endpoint_name).is_some()
                    {
                        write!(
                            f,
                            "{}",
                            string_enum_type(field_name, endpoint_name).unwrap()
                        )?;
                    } else {
                        write!(f, "{}", FieldTypeRustTypeNameDisplay(&parameter.field_type))?;
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

fn string_enum_type(s: &str, endpoint_name: &str) -> Option<&'static str> {
    match s {
        s if s.ends_with("currency_code") || s.ends_with("currency") => Some("Currency"),
        "country_code" => Some("Country"),
        "market" | "market_code" => Some("Market"),
        "transaction_type" => Some("TradeDescription"),
        "transaction_description" => Some("PayoutDescription"),
        "default_sale_allocation_method" => Some("SaleAllocationMethod"),
        "type_name" => Some("CashAccountTransactionTypeName"),
        "id" if endpoint_name == "GroupsList" => Some("IdOrName"),
        _ => None,
    }
}

struct FieldTypeRustTypeNameDisplay<'a>(&'a FieldType);

impl<'a> fmt::Display for FieldTypeRustTypeNameDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let FieldTypeRustTypeNameDisplay(field_type) = *self;

        match field_type {
            FieldType::Scalar(t) => write!(f, "{}", FieldTypeBaseRustTypeNameDisplay(t)),
            FieldType::Array(t) => write!(f, "Vec<{}>", FieldTypeBaseRustTypeNameDisplay(t)),
        }
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
            FieldTypeBase::Number => write!(f, "Number"),
            FieldTypeBase::DateTime => write!(f, "DateTime<FixedOffset>"),
            FieldTypeBase::Unit => write!(f, "()"),
            FieldTypeBase::File => write!(f, "()"),
            FieldTypeBase::Boolean => write!(f, "bool"),
            FieldTypeBase::Group => unreachable!(),
            FieldTypeBase::Portfolio => unreachable!(),
            FieldTypeBase::Price => unreachable!(),
            FieldTypeBase::Instruments => unreachable!(),
            FieldTypeBase::Membership => unreachable!(),
            FieldTypeBase::Currency => unreachable!(),
            FieldTypeBase::DrpTrade => unreachable!(),
            FieldTypeBase::Payout => unreachable!(),
            FieldTypeBase::Parcel => unreachable!(),
            FieldTypeBase::PortfolioDiversity => unreachable!(),
            FieldTypeBase::PortfolioDiversityGroup => unreachable!(),
            FieldTypeBase::PortfolioDiversityHolding => unreachable!(),
            FieldTypeBase::CustomGroup => unreachable!(),
            FieldTypeBase::Market => unreachable!(),
            FieldTypeBase::IndustryClassification => unreachable!(),
            FieldTypeBase::SectorClassification => unreachable!(),
            FieldTypeBase::InvestmentType => unreachable!(),
            FieldTypeBase::Country => unreachable!(),
            FieldTypeBase::CustomGroupCategory => unreachable!(),
            FieldTypeBase::Object => unreachable!(),
            FieldTypeBase::PortfolioPerformance => unreachable!(),
            FieldTypeBase::Holding => unreachable!(),
            FieldTypeBase::CashAccount => unreachable!(),
            FieldTypeBase::SubTotal => unreachable!(),
            FieldTypeBase::PortfolioValuation => unreachable!(),
            FieldTypeBase::User => unreachable!(),
        }
    }
}

struct DocComment<'a>(&'a str);

impl<'a> fmt::Display for DocComment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let DocComment(source) = *self;
        let mut paragraphs = source
            .split("<p>")
            .map(|s| s.trim().trim_end_matches("</p>").trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.replace("<code>", "`")
                    .replace("</code>", "`")
                    .replace("&quot;", "\"")
                    .replace("&gt;", "\\<")
                    .replace("&lt;", "\\>")
            })
            .flat_map(|s| {
                s.split_inclusive("<h5>")
                    .flat_map(|s| s.split_inclusive("</h5>"))
                    .map(str::trim)
                    .filter_map(|s| {
                        if s.starts_with("<h5>") || s.ends_with("</h5>") {
                            let s = s
                                .trim_start_matches("<h5>")
                                .trim_end_matches("</h5>")
                                .trim();
                            if !s.is_empty() {
                                Some(format!("## {}", s))
                            } else {
                                None
                            }
                        } else {
                            Some(s.to_string())
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            });

        if let Some(paragraph) = paragraphs.next() {
            writeln!(f, "/// {}", paragraph)?;
        }

        for paragraph in paragraphs {
            writeln!(f, "///")?;
            writeln!(f, "/// {}", paragraph)?;
        }
        Ok(())
    }
}
