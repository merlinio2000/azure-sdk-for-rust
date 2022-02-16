#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressSpace {
    #[doc = "A list of address blocks reserved for this virtual network in CIDR notation."]
    #[serde(rename = "addressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub address_prefixes: Vec<String>,
}
impl AddressSpace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides details of the entity that created/updated the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatedBy {
    #[doc = "The Object ID that created the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[doc = "The Personal Object ID corresponding to the object ID above"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub puid: Option<String>,
    #[doc = "The application ID of the application that initiated the creation of the workspace. For example, Azure Portal."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
impl CreatedBy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CreatedDateTime = String;
#[doc = "The object that contains details of encryption used on the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Default, Microsoft.Keyvault"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
    #[doc = "The name of KeyVault key."]
    #[serde(rename = "KeyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The version of KeyVault key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyversion: Option<String>,
    #[doc = "The Uri of KeyVault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyvaulturi: Option<String>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Default, Microsoft.Keyvault"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        Default,
        #[serde(rename = "Microsoft.Keyvault")]
        MicrosoftKeyvault,
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::Default
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Indicates which property in the request is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[doc = "A machine readable error code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "Inner error details if they exist."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<String>,
}
impl ErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}
impl ErrorResponse {
    pub fn new(error: ErrorInfo) -> Self {
        Self { error }
    }
}
#[doc = "The Managed Identity details for storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityConfiguration {
    #[doc = "The objectId of the Managed Identity that is linked to the Managed Storage account."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant Id where the Managed Identity is created."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of Identity created. It can be either SystemAssigned or UserAssigned."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedIdentityConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
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
        #[doc = "Service provider: Microsoft.ResourceProvider"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
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
#[doc = "Result of the request to list Resource Provider operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Resource Provider operations supported by the Resource Provider resource provider."]
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
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PeeringProvisioningState {
    Succeeded,
    Updating,
    Deleting,
    Failed,
}
#[doc = "Provisioning status of the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name."]
    pub name: String,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Peerings in a VirtualNetwork resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkPeering {
    #[doc = "Properties of the virtual network peering."]
    pub properties: VirtualNetworkPeeringPropertiesFormat,
    #[doc = "Name of the virtual network peering resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "type of the virtual network peering resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualNetworkPeering {
    pub fn new(properties: VirtualNetworkPeeringPropertiesFormat) -> Self {
        Self {
            properties,
            name: None,
            id: None,
            type_: None,
        }
    }
}
#[doc = "Gets all virtual network peerings under a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPeeringList {
    #[doc = "List of virtual network peerings on workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkPeering>,
    #[doc = "URL to get the next set of virtual network peering list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VirtualNetworkPeeringList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the virtual network peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkPeeringPropertiesFormat {
    #[doc = "Whether the VMs in the local virtual network space would be able to access the VMs in remote virtual network space."]
    #[serde(rename = "allowVirtualNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_virtual_network_access: Option<bool>,
    #[doc = "Whether the forwarded traffic from the VMs in the local virtual network will be allowed/disallowed in remote virtual network."]
    #[serde(rename = "allowForwardedTraffic", default, skip_serializing_if = "Option::is_none")]
    pub allow_forwarded_traffic: Option<bool>,
    #[doc = "If gateway links can be used in remote virtual networking to link to this virtual network."]
    #[serde(rename = "allowGatewayTransit", default, skip_serializing_if = "Option::is_none")]
    pub allow_gateway_transit: Option<bool>,
    #[doc = "If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway."]
    #[serde(rename = "useRemoteGateways", default, skip_serializing_if = "Option::is_none")]
    pub use_remote_gateways: Option<bool>,
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[serde(rename = "databricksVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub databricks_virtual_network: Option<virtual_network_peering_properties_format::DatabricksVirtualNetwork>,
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "databricksAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub databricks_address_space: Option<AddressSpace>,
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[serde(rename = "remoteVirtualNetwork")]
    pub remote_virtual_network: virtual_network_peering_properties_format::RemoteVirtualNetwork,
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "remoteAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub remote_address_space: Option<AddressSpace>,
    #[doc = "The status of the virtual network peering."]
    #[serde(rename = "peeringState", default, skip_serializing_if = "Option::is_none")]
    pub peering_state: Option<virtual_network_peering_properties_format::PeeringState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PeeringProvisioningState>,
}
impl VirtualNetworkPeeringPropertiesFormat {
    pub fn new(remote_virtual_network: virtual_network_peering_properties_format::RemoteVirtualNetwork) -> Self {
        Self {
            allow_virtual_network_access: None,
            allow_forwarded_traffic: None,
            allow_gateway_transit: None,
            use_remote_gateways: None,
            databricks_virtual_network: None,
            databricks_address_space: None,
            remote_virtual_network,
            remote_address_space: None,
            peering_state: None,
            provisioning_state: None,
        }
    }
}
pub mod virtual_network_peering_properties_format {
    use super::*;
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DatabricksVirtualNetwork {
        #[doc = "The Id of the databricks virtual network."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl DatabricksVirtualNetwork {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RemoteVirtualNetwork {
        #[doc = "The Id of the remote virtual network."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl RemoteVirtualNetwork {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The status of the virtual network peering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PeeringState {
        Initiated,
        Connected,
        Disconnected,
    }
}
#[doc = "Information about workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The workspace properties."]
    pub properties: WorkspaceProperties,
    #[doc = "SKU for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource, properties: WorkspaceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            sku: None,
        }
    }
}
#[doc = "The value which should be used for this field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomBooleanParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: bool,
}
impl WorkspaceCustomBooleanParameter {
    pub fn new(value: bool) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "The value which should be used for this field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomObjectParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: serde_json::Value,
}
impl WorkspaceCustomObjectParameter {
    pub fn new(value: serde_json::Value) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "Provisioning status of the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkspaceCustomParameterType {
    Bool,
    Object,
    String,
}
#[doc = "Custom Parameters used for Cluster Creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceCustomParameters {
    #[doc = "The Value."]
    #[serde(rename = "amlWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub aml_workspace_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub custom_virtual_network_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customPublicSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub custom_public_subnet_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customPrivateSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub custom_private_subnet_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "enableNoPublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_no_public_ip: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The Value."]
    #[serde(rename = "loadBalancerBackendPoolName", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_backend_pool_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "loadBalancerId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "natGatewayName", default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "publicIpName", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "prepareEncryption", default, skip_serializing_if = "Option::is_none")]
    pub prepare_encryption: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The object that contains details of encryption used on the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<WorkspaceEncryptionParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The Value."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "storageAccountSkuName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sku_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "vnetAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub vnet_address_prefix: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<WorkspaceCustomObjectParameter>,
}
impl WorkspaceCustomParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomStringParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: String,
}
impl WorkspaceCustomStringParameter {
    pub fn new(value: String) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "The object that contains details of encryption used on the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceEncryptionParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The object that contains details of encryption used on the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Encryption>,
}
impl WorkspaceEncryptionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The array of workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId")]
    pub managed_resource_group_id: String,
    #[doc = "Custom Parameters used for Cluster Creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<WorkspaceCustomParameters>,
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The blob URI where the UI definition file is located."]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "The workspace provider authorizations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<WorkspaceProviderAuthorization>,
    #[doc = "Provides details of the entity that created/updated the workspace."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedBy>,
    #[doc = "Provides details of the entity that created/updated the workspace."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<CreatedBy>,
    #[doc = "The date and time stamp when the workspace was created."]
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<CreatedDateTime>,
    #[doc = "The unique identifier of the databricks workspace in databricks control plane."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The workspace URL which is of the format 'adb-{workspaceId}.{random}.azuredatabricks.net'"]
    #[serde(rename = "workspaceUrl", default, skip_serializing_if = "Option::is_none")]
    pub workspace_url: Option<String>,
    #[doc = "The Managed Identity details for storage account."]
    #[serde(rename = "storageAccountIdentity", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_identity: Option<ManagedIdentityConfiguration>,
}
impl WorkspaceProperties {
    pub fn new(managed_resource_group_id: String) -> Self {
        Self {
            managed_resource_group_id,
            parameters: None,
            provisioning_state: None,
            ui_definition_uri: None,
            authorizations: Vec::new(),
            created_by: None,
            updated_by: None,
            created_date_time: None,
            workspace_id: None,
            workspace_url: None,
            storage_account_identity: None,
        }
    }
}
#[doc = "The workspace provider authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProviderAuthorization {
    #[doc = "The provider's principal identifier. This is the identity that the provider will use to call ARM to manage the workspace resources."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The provider's role definition identifier. This role will define all the permissions that the provider must have on the workspace's container resource group. This role definition cannot have permission to delete the resource group."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl WorkspaceProviderAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[doc = "An update to a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkspaceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
