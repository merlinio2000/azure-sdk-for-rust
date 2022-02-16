#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing dimensions. It contains a list of available dimensions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionsListResult {
    #[doc = "The list of dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dimension>,
}
impl DimensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Cost management REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.CostManagement."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Dimensions, Query."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of listing cost management operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of cost management operations supported by the Microsoft.CostManagement resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Query {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
impl Query {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QueryColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryProperties {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of columns"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<serde_json::Value>>,
}
impl QueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of query. It contains all columns listed under groupings and aggregation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResult {
    #[doc = "The list of usage data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Query>,
}
impl QueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A report config resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfig {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the report config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportConfigProperties>,
}
impl ReportConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregation expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    #[doc = "The name of the column to aggregate."]
    pub name: String,
    #[doc = "The name of the aggregation function to use."]
    pub function: report_config_aggregation::Function,
}
impl ReportConfigAggregation {
    pub fn new(name: String, function: report_config_aggregation::Function) -> Self {
        Self { name, function }
    }
}
pub mod report_config_aggregation {
    use super::*;
    #[doc = "The name of the aggregation function to use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[doc = "The type of the column in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReportConfigColumnType {
    Tag,
    Dimension,
}
#[doc = "The comparison expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: report_config_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl ReportConfigComparisonExpression {
    pub fn new(name: String, operator: report_config_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod report_config_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
    }
}
#[doc = "The definition of data present in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigDataset {
    #[doc = "The granularity of rows in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[doc = "The configuration of dataset in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[doc = "Dictionary of aggregation expression to use in the report. The key of each item in the dictionary is the alias for the aggregated column. Report can have up to 2 aggregation clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[doc = "Array of group by expression to use in the report. Report can have up to 2 group by clauses."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ReportConfigGrouping>,
    #[doc = "The filter expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
impl ReportConfigDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod report_config_dataset {
    use super::*;
    #[doc = "The granularity of rows in the report."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[doc = "The configuration of dataset in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigDatasetConfiguration {
    #[doc = "Array of column names to be included in the report. Any valid report column name is allowed. If not provided, then report includes all columns."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
impl ReportConfigDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a report config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[doc = "The type of the report."]
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    #[doc = "The time frame for pulling data for the report. If custom, then a specific time period must be provided."]
    pub timeframe: report_config_definition::Timeframe,
    #[doc = "The start and end date for pulling data for the report."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[doc = "The definition of data present in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<ReportConfigDataset>,
}
impl ReportConfigDefinition {
    pub fn new(type_: report_config_definition::Type, timeframe: report_config_definition::Timeframe) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            dataset: None,
        }
    }
}
pub mod report_config_definition {
    use super::*;
    #[doc = "The type of the report."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[doc = "The time frame for pulling data for the report. If custom, then a specific time period must be provided."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
    }
}
#[doc = "The destination information for the delivery of the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDeliveryDestination {
    #[doc = "The resource id of the storage account where reports will be delivered."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The name of the container where reports will be uploaded."]
    pub container: String,
    #[doc = "The name of the directory where reports will be uploaded."]
    #[serde(rename = "rootFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
}
impl ReportConfigDeliveryDestination {
    pub fn new(resource_id: String, container: String) -> Self {
        Self {
            resource_id,
            container,
            root_folder_path: None,
        }
    }
}
#[doc = "The delivery information associated with a report config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDeliveryInfo {
    #[doc = "The destination information for the delivery of the report."]
    pub destination: ReportConfigDeliveryDestination,
}
impl ReportConfigDeliveryInfo {
    pub fn new(destination: ReportConfigDeliveryDestination) -> Self {
        Self { destination }
    }
}
#[doc = "The filter expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<ReportConfigFilter>,
    #[doc = "The logical \"OR\" expression. Must have at least 2 items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<ReportConfigFilter>,
    #[doc = "The filter expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<ReportConfigFilter>>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ReportConfigComparisonExpression>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ReportConfigComparisonExpression>,
}
impl ReportConfigFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The group by expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[doc = "The type of the column in the report."]
    #[serde(rename = "columnType")]
    pub column_type: ReportConfigColumnType,
    #[doc = "The name of the column to group."]
    pub name: String,
}
impl ReportConfigGrouping {
    pub fn new(column_type: ReportConfigColumnType, name: String) -> Self {
        Self { column_type, name }
    }
}
#[doc = "Result of listing report configs. It contains a list of available report configurations in the scope provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigListResult {
    #[doc = "The list of report configs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReportConfig>,
}
impl ReportConfigListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the report config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigProperties {
    #[doc = "The schedule associated with a report config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ReportConfigSchedule>,
    #[doc = "The format of the report being delivered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<report_config_properties::Format>,
    #[doc = "The delivery information associated with a report config."]
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ReportConfigDeliveryInfo,
    #[doc = "The definition of a report config."]
    pub definition: ReportConfigDefinition,
}
impl ReportConfigProperties {
    pub fn new(delivery_info: ReportConfigDeliveryInfo, definition: ReportConfigDefinition) -> Self {
        Self {
            schedule: None,
            format: None,
            delivery_info,
            definition,
        }
    }
}
pub mod report_config_properties {
    use super::*;
    #[doc = "The format of the report being delivered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Csv,
    }
}
#[doc = "The start and end date for recurrence schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigRecurrencePeriod {
    #[doc = "The start date of recurrence."]
    pub from: String,
    #[doc = "The end date of recurrence. If not provided, we default this to 10 years from the start date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl ReportConfigRecurrencePeriod {
    pub fn new(from: String) -> Self {
        Self { from, to: None }
    }
}
#[doc = "The schedule associated with a report config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSchedule {
    #[doc = "The status of the schedule. Whether active or not. If inactive, the report's scheduled execution is paused."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<report_config_schedule::Status>,
    #[doc = "The schedule recurrence."]
    pub recurrence: report_config_schedule::Recurrence,
    #[doc = "The start and end date for recurrence schedule."]
    #[serde(rename = "recurrencePeriod")]
    pub recurrence_period: ReportConfigRecurrencePeriod,
}
impl ReportConfigSchedule {
    pub fn new(recurrence: report_config_schedule::Recurrence, recurrence_period: ReportConfigRecurrencePeriod) -> Self {
        Self {
            status: None,
            recurrence,
            recurrence_period,
        }
    }
}
pub mod report_config_schedule {
    use super::*;
    #[doc = "The status of the schedule. Whether active or not. If inactive, the report's scheduled execution is paused."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Inactive,
    }
    #[doc = "The schedule recurrence."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
    }
}
#[doc = "The start and end date for pulling data for the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    #[doc = "The start date to pull data from."]
    pub from: String,
    #[doc = "The end date to pull data to."]
    pub to: String,
}
impl ReportConfigTimePeriod {
    pub fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
