#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Microsoft.Resources operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
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
        #[doc = "Service provider: Microsoft.Resources"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Microsoft.Resources operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Resources operations."]
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
#[doc = "The resource link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLink {
    #[doc = "The fully qualified ID of the resource link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource link object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<serde_json::Value>,
    #[doc = "The resource link properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceLinkProperties>,
}
impl ResourceLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource link filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLinkFilter {
    #[doc = "The ID of the target resource."]
    #[serde(rename = "targetId")]
    pub target_id: String,
}
impl ResourceLinkFilter {
    pub fn new(target_id: String) -> Self {
        Self { target_id }
    }
}
#[doc = "The resource link properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLinkProperties {
    #[doc = "The fully qualified ID of the source resource in the link. "]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[doc = "The fully qualified ID of the target resource in the link."]
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[doc = "Notes about the resource link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
impl ResourceLinkProperties {
    pub fn new(target_id: String) -> Self {
        Self {
            source_id: None,
            target_id,
            notes: None,
        }
    }
}
#[doc = "List of resource links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLinkResult {
    #[doc = "An array of resource links."]
    pub value: Vec<ResourceLink>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceLinkResult {
    pub fn new(value: Vec<ResourceLink>) -> Self {
        Self { value, next_link: None }
    }
}
