#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The parameters to list SAS credentials of a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSasParameters {
    #[doc = "The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f)."]
    #[serde(rename = "signedServices")]
    pub signed_services: account_sas_parameters::SignedServices,
    #[doc = "The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files."]
    #[serde(rename = "signedResourceTypes")]
    pub signed_resource_types: account_sas_parameters::SignedResourceTypes,
    #[doc = "The signed permissions for the account SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[serde(rename = "signedPermission")]
    pub signed_permission: account_sas_parameters::SignedPermission,
    #[doc = "An IP address or a range of IP addresses from which to accept requests."]
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[serde(rename = "signedProtocol", default, skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<account_sas_parameters::SignedProtocol>,
    #[doc = "The time at which the SAS becomes valid."]
    #[serde(rename = "signedStart", default, skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[doc = "The time at which the shared access signature becomes invalid."]
    #[serde(rename = "signedExpiry")]
    pub signed_expiry: String,
    #[doc = "The key to sign the account SAS token with."]
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}
impl AccountSasParameters {
    pub fn new(
        signed_services: account_sas_parameters::SignedServices,
        signed_resource_types: account_sas_parameters::SignedResourceTypes,
        signed_permission: account_sas_parameters::SignedPermission,
        signed_expiry: String,
    ) -> Self {
        Self {
            signed_services,
            signed_resource_types,
            signed_permission,
            signed_ip: None,
            signed_protocol: None,
            signed_start: None,
            signed_expiry,
            key_to_sign: None,
        }
    }
}
pub mod account_sas_parameters {
    use super::*;
    #[doc = "The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedServices {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "q")]
        Q,
        #[serde(rename = "t")]
        T,
        #[serde(rename = "f")]
        F,
    }
    #[doc = "The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResourceTypes {
        #[serde(rename = "s")]
        S,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "o")]
        O,
    }
    #[doc = "The signed permissions for the account SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
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
    #[doc = "Gets the reason that a storage account name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "The custom domain assigned to this storage account. This can be set via Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[doc = "Gets or sets the custom domain name assigned to the storage account. Name is the CNAME source."]
    pub name: String,
    #[doc = "Indicates whether indirect CName validation is enabled. Default value is false. This should only be set on updates."]
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
#[doc = "The encryption settings on the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[doc = "A list of services that support encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Storage"]
    #[serde(rename = "keySource")]
    pub key_source: encryption::KeySource,
}
impl Encryption {
    pub fn new(key_source: encryption::KeySource) -> Self {
        Self {
            services: None,
            key_source,
        }
    }
}
pub mod encryption {
    use super::*;
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Storage"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Storage")]
        MicrosoftStorage,
    }
}
#[doc = "A service that allows server-side encryption to be used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionService {
    #[doc = "A boolean indicating whether or not the service encrypts the data as it is stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets a rough estimate of the date/time when the encryption was last enabled by the user. Only returned when encryption is enabled. There might be some unencrypted blobs which were written after this time, as it is just a rough estimate."]
    #[serde(rename = "lastEnabledTime", default, skip_serializing_if = "Option::is_none")]
    pub last_enabled_time: Option<String>,
}
impl EncryptionService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of services that support encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionServices {
    #[doc = "A service that allows server-side encryption to be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
}
impl EncryptionServices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URIs that are used to perform a retrieval of a public blob, queue, or table object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[doc = "Gets the blob endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[doc = "Gets the queue endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[doc = "Gets the table endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[doc = "Gets the file endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
impl Endpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List SAS credentials operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAccountSasResponse {
    #[doc = "List SAS credentials of storage account."]
    #[serde(rename = "accountSasToken", default, skip_serializing_if = "Option::is_none")]
    pub account_sas_token: Option<String>,
}
impl ListAccountSasResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List service SAS credentials operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListServiceSasResponse {
    #[doc = "List service SAS credentials of specific resource."]
    #[serde(rename = "serviceSasToken", default, skip_serializing_if = "Option::is_none")]
    pub service_sas_token: Option<String>,
}
impl ListServiceSasResponse {
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
    #[doc = "Tags assigned to a resource; can be used for viewing and grouping a resource (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to list service SAS credentials of a specific resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSasParameters {
    #[doc = "The canonical path to the signed resource."]
    #[serde(rename = "canonicalizedResource")]
    pub canonicalized_resource: String,
    #[doc = "The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s)."]
    #[serde(rename = "signedResource")]
    pub signed_resource: service_sas_parameters::SignedResource,
    #[doc = "The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[serde(rename = "signedPermission", default, skip_serializing_if = "Option::is_none")]
    pub signed_permission: Option<service_sas_parameters::SignedPermission>,
    #[doc = "An IP address or a range of IP addresses from which to accept requests."]
    #[serde(rename = "signedIp", default, skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[serde(rename = "signedProtocol", default, skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<service_sas_parameters::SignedProtocol>,
    #[doc = "The time at which the SAS becomes valid."]
    #[serde(rename = "signedStart", default, skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[doc = "The time at which the shared access signature becomes invalid."]
    #[serde(rename = "signedExpiry", default, skip_serializing_if = "Option::is_none")]
    pub signed_expiry: Option<String>,
    #[doc = "A unique value up to 64 characters in length that correlates to an access policy specified for the container, queue, or table."]
    #[serde(rename = "signedIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub signed_identifier: Option<String>,
    #[doc = "The start of partition key."]
    #[serde(rename = "startPk", default, skip_serializing_if = "Option::is_none")]
    pub start_pk: Option<String>,
    #[doc = "The end of partition key."]
    #[serde(rename = "endPk", default, skip_serializing_if = "Option::is_none")]
    pub end_pk: Option<String>,
    #[doc = "The start of row key."]
    #[serde(rename = "startRk", default, skip_serializing_if = "Option::is_none")]
    pub start_rk: Option<String>,
    #[doc = "The end of row key."]
    #[serde(rename = "endRk", default, skip_serializing_if = "Option::is_none")]
    pub end_rk: Option<String>,
    #[doc = "The key to sign the account SAS token with."]
    #[serde(rename = "keyToSign", default, skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
    #[doc = "The response header override for cache control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscc: Option<String>,
    #[doc = "The response header override for content disposition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscd: Option<String>,
    #[doc = "The response header override for content encoding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsce: Option<String>,
    #[doc = "The response header override for content language."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rscl: Option<String>,
    #[doc = "The response header override for content type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsct: Option<String>,
}
impl ServiceSasParameters {
    pub fn new(canonicalized_resource: String, signed_resource: service_sas_parameters::SignedResource) -> Self {
        Self {
            canonicalized_resource,
            signed_resource,
            signed_permission: None,
            signed_ip: None,
            signed_protocol: None,
            signed_start: None,
            signed_expiry: None,
            signed_identifier: None,
            start_pk: None,
            end_pk: None,
            start_rk: None,
            end_rk: None,
            key_to_sign: None,
            rscc: None,
            rscd: None,
            rsce: None,
            rscl: None,
            rsct: None,
        }
    }
}
pub mod service_sas_parameters {
    use super::*;
    #[doc = "The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResource {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "f")]
        F,
        #[serde(rename = "s")]
        S,
    }
    #[doc = "The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[doc = "The protocol permitted for a request made with the account SAS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[doc = "The SKU of the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Gets or sets the sku name. Required for account creation; optional for update. Note that in older versions, sku name was called accountType."]
    pub name: sku::Name,
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Gets or sets the sku name. Required for account creation; optional for update. Note that in older versions, sku name was called accountType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
}
#[doc = "The storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets the Kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<storage_account::Kind>,
    #[doc = "Properties of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
impl StorageAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account {
    use super::*;
    #[doc = "Gets the Kind."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[doc = "The parameters used to check the availability of the storage account name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: storage_account_check_name_availability_parameters::Type,
}
impl StorageAccountCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: storage_account_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod storage_account_check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Storage/storageAccounts")]
        MicrosoftStorageStorageAccounts,
    }
}
#[doc = "The parameters used when creating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    #[doc = "The SKU of the storage account."]
    pub sku: Sku,
    #[doc = "Required. Indicates the type of storage account."]
    pub kind: storage_account_create_parameters::Kind,
    #[doc = "Required. Gets or sets the location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update, the request will succeed."]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used for viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key with a length no greater than 128 characters and a value with a length no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The parameters used to create the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
impl StorageAccountCreateParameters {
    pub fn new(sku: Sku, kind: storage_account_create_parameters::Kind, location: String) -> Self {
        Self {
            sku,
            kind,
            location,
            tags: None,
            properties: None,
        }
    }
}
pub mod storage_account_create_parameters {
    use super::*;
    #[doc = "Required. Indicates the type of storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[doc = "An access key for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountKey {
    #[doc = "Name of the key."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Base 64-encoded value of the key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Permissions for the key -- read-only or full permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<storage_account_key::Permissions>,
}
impl StorageAccountKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_key {
    use super::*;
    #[doc = "Permissions for the key -- read-only or full permissions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        Read,
        Full,
    }
}
#[doc = "The response from the ListKeys operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListKeysResult {
    #[doc = "Gets the list of storage account keys and their properties for the specified storage account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<StorageAccountKey>,
}
impl StorageAccountListKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Storage Accounts operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListResult {
    #[doc = "Gets the list of storage accounts and their properties."]
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
    #[doc = "Gets the status of the storage account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue, or table object."]
    #[serde(rename = "primaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[doc = "Gets the location of the primary data center for the storage account."]
    #[serde(rename = "primaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,
    #[doc = "Gets the status indicating whether the primary location of the storage account is available or unavailable."]
    #[serde(rename = "statusOfPrimary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[doc = "Gets the timestamp of the most recent instance of a failover to the secondary location. Only the most recent timestamp is retained. This element is not returned if there has never been a failover instance. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "lastGeoFailoverTime", default, skip_serializing_if = "Option::is_none")]
    pub last_geo_failover_time: Option<String>,
    #[doc = "Gets the location of the geo-replicated secondary for the storage account. Only available if the accountType is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "secondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub secondary_location: Option<String>,
    #[doc = "Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[serde(rename = "statusOfSecondary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[doc = "Gets the creation date and time of the storage account in UTC."]
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The URIs that are used to perform a retrieval of a public blob, queue, or table object."]
    #[serde(rename = "secondaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties::AccessTier>,
}
impl StorageAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties {
    use super::*;
    #[doc = "Gets the status of the storage account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[doc = "Gets the status indicating whether the primary location of the storage account is available or unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[doc = "Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[doc = "The parameters used to create the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesCreateParameters {
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_create_parameters::AccessTier>,
}
impl StorageAccountPropertiesCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[doc = "The parameters used when updating a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[doc = "The custom domain assigned to this storage account. This can be set via Update."]
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[doc = "The encryption settings on the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_update_parameters::AccessTier>,
}
impl StorageAccountPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[doc = "Required for storage accounts where kind = BlobStorage. The access tier used for billing."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
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
#[doc = "The parameters that can be provided when updating the storage account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountUpdateParameters {
    #[doc = "The SKU of the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater in length than 128 characters and a value no greater in length than 256 characters."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Gets the unit of measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "Gets the current count of the allocated resources in the subscription."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Gets the maximum count of the resources that can be allocated in the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The usage names that can be used; currently limited to StorageAccount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "Gets the unit of measurement."]
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
#[doc = "The response from the List Usages operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "Gets or sets the list of Storage Resource Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The usage names that can be used; currently limited to StorageAccount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "Gets a string describing the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets a localized string describing the resource name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
