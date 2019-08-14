use crate::drive::drive_item::calculatedcolumn::CalculatedColumn;
use crate::drive::drive_item::choicecolumn::ChoiceColumn;
use crate::drive::drive_item::currencycolumn::CurrencyColumn;
use crate::drive::drive_item::datetimecolumn::DateTimeColumn;
use crate::drive::drive_item::defaultcolumnvalue::DefaultColumnValue;
use crate::drive::drive_item::lookupcolumn::LookupColumn;
use crate::drive::drive_item::numbercolumn::NumberColumn;
use crate::drive::drive_item::personorgroupcolumn::PersonOrGroupColumn;
use crate::drive::drive_item::textcolumn::TextColumn;
use std::collections::BTreeMap;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/columndefinition?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ColumnDefinition {
    #[serde(rename = "columnGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    column_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(rename = "enforceUniqueValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_unique_values: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boolean: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calculated: Option<CalculatedColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choice: Option<ChoiceColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<CurrencyColumn>,
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    date_time: Option<DateTimeColumn>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<DefaultColumnValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup: Option<LookupColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<NumberColumn>,
    #[serde(rename = "personOrGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    person_or_group: Option<PersonOrGroupColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<TextColumn>,
}

impl Eq for ColumnDefinition {}
