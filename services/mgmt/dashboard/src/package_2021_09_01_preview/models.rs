#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    Internal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoGeneratedDomainNameLabelScope")]
pub enum AutoGeneratedDomainNameLabelScope {
    TenantReuse,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoGeneratedDomainNameLabelScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoGeneratedDomainNameLabelScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoGeneratedDomainNameLabelScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TenantReuse => serializer.serialize_unit_variant("AutoGeneratedDomainNameLabelScope", 0u32, "TenantReuse"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of identity that created the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CreatedByType")]
pub enum CreatedByType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CreatedByType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CreatedByType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CreatedByType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LastModifiedByType")]
pub enum LastModifiedByType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LastModifiedByType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LastModifiedByType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LastModifiedByType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The grafana resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGrafana {
    #[doc = "ARM id of the grafana resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the grafana resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the grafana resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Properties specific to the grafana resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedGrafanaProperties>,
    #[doc = "The managed identity of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The tags for grafana resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the grafana resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ManagedGrafana {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGrafanaListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedGrafana>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedGrafanaListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagedGrafanaListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the grafana resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGrafanaProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Grafana software version."]
    #[serde(rename = "grafanaVersion", default, skip_serializing_if = "Option::is_none")]
    pub grafana_version: Option<String>,
    #[doc = "The endpoint of the Grafana instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "zoneRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundancy: Option<ZoneRedundancy>,
    #[serde(rename = "autoGeneratedDomainNameLabelScope", default, skip_serializing_if = "Option::is_none")]
    pub auto_generated_domain_name_label_scope: Option<AutoGeneratedDomainNameLabelScope>,
}
impl ManagedGrafanaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a PATCH request to a grafana resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGrafanaUpdateParameters {
    #[doc = "The managed identity of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[doc = "The new tags of the grafana resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ManagedGrafanaUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed identity of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityTypes>,
    #[doc = "The principal id of the system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Dictionary of user assigned identities."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityTypes")]
pub enum ManagedIdentityTypes {
    None,
    SystemAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedIdentityTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedIdentityTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedIdentityTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedIdentityTypes", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedIdentityTypes", 1u32, "SystemAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Localized display information for this particular operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The localized friendly form of the resource provider name, i.e., Microsoft.Dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly name of the resource type related to this operation, e.g., 'grafana'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Read grafana'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of REST API operations supported by Microsoft.Dashboard provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Dashboard provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Microsoft.Dashboard REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name, i.e., {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation applies to data-plane. Set \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Origin>,
    #[doc = "Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionType>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The intended executor of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Origin")]
pub enum Origin {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user,system")]
    UserSystem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Origin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
            Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
            Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<CreatedByType>,
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ZoneRedundancy")]
pub enum ZoneRedundancy {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ZoneRedundancy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ZoneRedundancy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ZoneRedundancy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("ZoneRedundancy", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ZoneRedundancy", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for ZoneRedundancy {
    fn default() -> Self {
        Self::Disabled
    }
}
