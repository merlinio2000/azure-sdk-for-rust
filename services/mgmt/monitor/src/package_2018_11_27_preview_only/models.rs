#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Information about a container with data for a given resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataContainer {
    #[doc = "Information about a Log Analytics Workspace."]
    pub workspace: WorkspaceInfo,
}
impl DataContainer {
    pub fn new(workspace: WorkspaceInfo) -> Self {
        Self { workspace }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error code identifying the specific error."]
    pub code: String,
    #[doc = "Error message in the caller's locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new(code: String) -> Self {
        Self { code, message: None }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithError {
    #[doc = "Error details."]
    pub error: Error,
}
impl ResponseWithError {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}
#[doc = "VM Insights onboarding status for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmInsightsOnboardingStatus {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vm_insights_onboarding_status::Properties>,
}
impl VmInsightsOnboardingStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vm_insights_onboarding_status {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Azure Resource Manager identifier of the resource whose onboarding status is being represented."]
        #[serde(rename = "resourceId")]
        pub resource_id: String,
        #[doc = "The onboarding status for the resource. Note that, a higher level scope, e.g., resource group or subscription, is considered onboarded if at least one resource under it is onboarded."]
        #[serde(rename = "onboardingStatus")]
        pub onboarding_status: properties::OnboardingStatus,
        #[doc = "The status of VM Insights data from the resource. When reported as `present` the data array will contain information about the data containers to which data for the specified resource is being routed."]
        #[serde(rename = "dataStatus")]
        pub data_status: properties::DataStatus,
        #[doc = "Containers that currently store VM Insights data for the specified resource."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub data: Vec<DataContainer>,
    }
    impl Properties {
        pub fn new(resource_id: String, onboarding_status: properties::OnboardingStatus, data_status: properties::DataStatus) -> Self {
            Self {
                resource_id,
                onboarding_status,
                data_status,
                data: Vec::new(),
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "The onboarding status for the resource. Note that, a higher level scope, e.g., resource group or subscription, is considered onboarded if at least one resource under it is onboarded."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OnboardingStatus {
            #[serde(rename = "onboarded")]
            Onboarded,
            #[serde(rename = "notOnboarded")]
            NotOnboarded,
            #[serde(rename = "unknown")]
            Unknown,
        }
        #[doc = "The status of VM Insights data from the resource. When reported as `present` the data array will contain information about the data containers to which data for the specified resource is being routed."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DataStatus {
            #[serde(rename = "present")]
            Present,
            #[serde(rename = "notPresent")]
            NotPresent,
        }
    }
}
#[doc = "Information about a Log Analytics Workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    #[doc = "Azure Resource Manager identifier of the Log Analytics Workspace."]
    pub id: String,
    #[doc = "Location of the Log Analytics workspace."]
    pub location: String,
    #[doc = "Resource properties."]
    pub properties: workspace_info::Properties,
}
impl WorkspaceInfo {
    pub fn new(id: String, location: String, properties: workspace_info::Properties) -> Self {
        Self { id, location, properties }
    }
}
pub mod workspace_info {
    use super::*;
    #[doc = "Resource properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "Log Analytics workspace identifier."]
        #[serde(rename = "customerId")]
        pub customer_id: String,
    }
    impl Properties {
        pub fn new(customer_id: String) -> Self {
            Self { customer_id }
        }
    }
}
