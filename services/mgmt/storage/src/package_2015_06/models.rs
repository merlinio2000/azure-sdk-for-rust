#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "The error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "The reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "The custom domain assigned to this storage account. This can be set via Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[doc = "The custom domain name. Name is the CNAME source."]
    pub name: String,
    #[doc = "Indicates whether indirect CName validation is enabled. Default value is false. This should only be set on updates"]
    #[serde(rename = "useSubDomainName", default, skip_serializing_if = "Option::is_none")]
    pub use_sub_domain_name: Option<bool>,
}
impl CustomDomain {
    pub fn new(name: String) -> Self {
        Self {
            name,
            use_sub_domain_name: None,
        }
    }
}
#[doc = "The URIs that are used to perform a retrieval of a public blob, queue or table object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[doc = "The blob endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[doc = "The queue endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[doc = "The table endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "The file endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
impl Endpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a storage resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to check the availability of the storage account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl StorageAccountCheckNameAvailabilityParameters {
    pub fn new(name: String) -> Self {
        Self { name, type_: None }
    }
}
#[doc = "The parameters to provide for the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    #[doc = "The location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update, the request will succeed."]
    pub location: String,
    #[doc = "A list of key value pairs that describe the resource. These tags can be used for viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key with a length no greater than 128 characters and a value with a length no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters used to create the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
impl StorageAccountCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
        }
    }
}
#[doc = "The access keys for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountKeys {
    #[doc = "The value of key 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "The value of key 2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl StorageAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list storage accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListResult {
    #[doc = "The list of storage accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccount>,
}
impl StorageAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountProperties {
    #[doc = "The status of the storage account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[doc = "The type of the storage account."]
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<storage_account_properties::AccountType>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue or table object."]
    #[serde(rename = "primaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[doc = "The location of the primary data center for the storage account."]
    #[serde(rename = "primaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,
    #[doc = "The status indicating whether the primary location of the storage account is available or unavailable."]
    #[serde(rename = "statusOfPrimary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[doc = "The timestamp of the most recent instance of a failover to the secondary location. Only the most recent timestamp is retained. This element is not returned if there has never been a failover instance. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "lastGeoFailoverTime", default, skip_serializing_if = "Option::is_none")]
    pub last_geo_failover_time: Option<String>,
    #[doc = "The location of the geo-replicated secondary for the storage account. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "secondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub secondary_location: Option<String>,
    #[doc = "The status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "statusOfSecondary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[doc = "The creation date and time of the storage account in UTC."]
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue or table object."]
    #[serde(rename = "secondaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
}
impl StorageAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties {
    use super::*;
    #[doc = "The status of the storage account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[doc = "The type of the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
    #[doc = "The status indicating whether the primary location of the storage account is available or unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        Available,
        Unavailable,
    }
    #[doc = "The status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        Available,
        Unavailable,
    }
}
#[doc = "The parameters used to create the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesCreateParameters {
    #[doc = "The sku name. Required for account creation; optional for update. Note that in older versions, sku name was called accountType."]
    #[serde(rename = "accountType")]
    pub account_type: storage_account_properties_create_parameters::AccountType,
}
impl StorageAccountPropertiesCreateParameters {
    pub fn new(account_type: storage_account_properties_create_parameters::AccountType) -> Self {
        Self { account_type }
    }
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[doc = "The sku name. Required for account creation; optional for update. Note that in older versions, sku name was called accountType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[doc = "The parameters used when updating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[doc = "The account type. Note that StandardZRS and PremiumLRS accounts cannot be changed to other account types, and other account types cannot be changed to StandardZRS or PremiumLRS."]
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<storage_account_properties_update_parameters::AccountType>,
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
}
impl StorageAccountPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[doc = "The account type. Note that StandardZRS and PremiumLRS accounts cannot be changed to other account types, and other account types cannot be changed to StandardZRS or PremiumLRS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[doc = "The parameters used to regenerate the storage account key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl StorageAccountRegenerateKeyParameters {
    pub fn new(key_name: String) -> Self {
        Self { key_name }
    }
}
#[doc = "The parameters to update on the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountUpdateParameters {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters used when updating a storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
}
impl StorageAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Storage Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[doc = "The unit of measurement."]
    pub unit: usage::Unit,
    #[doc = "The current count of the allocated resources in the subscription."]
    #[serde(rename = "currentValue")]
    pub current_value: i32,
    #[doc = "The maximum count of the resources that can be allocated in the subscription."]
    pub limit: i32,
    #[doc = "The Usage Names."]
    pub name: UsageName,
}
impl Usage {
    pub fn new(unit: usage::Unit, current_value: i32, limit: i32, name: UsageName) -> Self {
        Self {
            unit,
            current_value,
            limit,
            name,
        }
    }
}
pub mod usage {
    use super::*;
    #[doc = "The unit of measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[doc = "The List Usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "The list Storage Resource Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "A string describing the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "A localized string describing the resource name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
