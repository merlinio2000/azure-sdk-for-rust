#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "this is the management partner operations error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "this is the extended error info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
    #[doc = "this is the error response code that management partner operations may return"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[doc = "this is the extended error info message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ErrorResponseCode = String;
#[doc = "this is the extended error info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[doc = "this is the error response code that management partner operations may return"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[doc = "this is the extended error info message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner state: Active or Deleted"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagementPartnerState {
    Active,
    Deleted,
}
#[doc = "this is the management partner operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "the is management partner provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "the is management partner resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "the is management partner operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "the is management partner operation description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner operations list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "this is the operation response list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner operations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[doc = "this is the operation response name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "this is the management partner operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "the is operation response origin information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerProperties {
    #[doc = "This is the partner id"]
    #[serde(rename = "partnerId", default, skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[doc = "This is the partner name"]
    #[serde(rename = "partnerName", default, skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[doc = "This is the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "This is the object id."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "This is the version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "This is the DateTime when the partner was updated."]
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[doc = "This is the DateTime when the partner was created."]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "this is the management partner state: Active or Deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ManagementPartnerState>,
}
impl PartnerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner operations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerResponse {
    #[doc = "Type of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "this is the management partner properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartnerProperties>,
    #[doc = "Type of resource. \"Microsoft.ManagementPartner/partners\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PartnerResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
