#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Input values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The name of the service instance to check."]
    pub name: String,
    #[doc = "The fully qualified resource type which includes provider namespace."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsInternal>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetailsInternal {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetailsInternal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Default value is 'user,system'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft.HealthcareApis"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: Services"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly description for the operation,"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of service operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link used to get the next page of service description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of service operations supported by the Microsoft.HealthcareApis resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties indicating the operation result of an operation on a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultsDescription {
    #[doc = "The ID of the operation returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the operation being performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_results_description::Status>,
    #[doc = "The time that the operation was started."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Additional properties of the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationResultsDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_results_description {
    use super::*;
    #[doc = "The status of the operation being performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionDescription {
    #[serde(flatten)]
    pub private_endpoint_connection: PrivateEndpointConnection,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnectionDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResultDescription {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnectionDescription>,
}
impl PrivateEndpointConnectionListResultDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceDescription {
    #[serde(flatten)]
    pub private_link_resource: PrivateLinkResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateLinkResourceDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResultDescription {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResourceDescription>,
}
impl PrivateLinkResourceListResultDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceAccessPoliciesInfo = Vec<ServiceAccessPolicyEntry>;
#[doc = "An access policy entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccessPolicyEntry {
    #[doc = "An Azure AD object ID (User or Apps) that is allowed access to the FHIR service."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl ServiceAccessPolicyEntry {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
    }
}
#[doc = "Azure container registry configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAcrConfigurationInfo {
    #[doc = "The list of the ACR login servers."]
    #[serde(rename = "loginServers", default, skip_serializing_if = "Vec::is_empty")]
    pub login_servers: Vec<String>,
}
impl ServiceAcrConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authentication configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAuthenticationConfigurationInfo {
    #[doc = "The authority url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "The audience url for the service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "If the SMART on FHIR proxy is enabled"]
    #[serde(rename = "smartProxyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub smart_proxy_enabled: Option<bool>,
}
impl ServiceAuthenticationConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceCorsConfigurationHeaderEntry = String;
#[doc = "The settings for the CORS configuration of the service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceCorsConfigurationInfo {
    #[doc = "The origins to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<ServiceCorsConfigurationOriginEntry>,
    #[doc = "The headers to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<ServiceCorsConfigurationHeaderEntry>,
    #[doc = "The methods to be allowed via CORS."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<ServiceCorsConfigurationMethodEntry>,
    #[doc = "The max age to be allowed via CORS."]
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    #[doc = "If credentials are allowed via CORS."]
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
}
impl ServiceCorsConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceCorsConfigurationMethodEntry = String;
pub type ServiceCorsConfigurationOriginEntry = String;
#[doc = "The settings for the Cosmos DB database backing the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceCosmosDbConfigurationInfo {
    #[doc = "The provisioned throughput for the backing database."]
    #[serde(rename = "offerThroughput", default, skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<i64>,
    #[doc = "The URI of the customer-managed key for the backing database."]
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
}
impl ServiceCosmosDbConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export operation configuration information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceExportConfigurationInfo {
    #[doc = "The name of the default export storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
}
impl ServiceExportConfigurationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesDescription {
    #[serde(flatten)]
    pub services_resource: ServicesResource,
    #[doc = "The properties of a service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ServicesDescription {
    pub fn new(services_resource: ServicesResource) -> Self {
        Self {
            services_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "A list of service description objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesDescriptionListResult {
    #[doc = "The link used to get the next page of service description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of service description objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServicesDescription>,
}
impl ServicesDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties indicating whether a given service name is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesNameAvailabilityInfo {
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<services_name_availability_info::Reason>,
    #[doc = "The detailed reason message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ServicesNameAvailabilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_name_availability_info {
    use super::*;
    #[doc = "The reason for unavailability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "The description of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesPatchDescription {
    #[doc = "Instance tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties for updating a service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServicesPropertiesUpdateParameters>,
}
impl ServicesPatchDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<services_properties::ProvisioningState>,
    #[doc = "The access policies of the service instance."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<ServiceAccessPoliciesInfo>,
    #[doc = "The settings for the Cosmos DB database backing the service."]
    #[serde(rename = "cosmosDbConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cosmos_db_configuration: Option<ServiceCosmosDbConfigurationInfo>,
    #[doc = "Authentication configuration information"]
    #[serde(rename = "authenticationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<ServiceAuthenticationConfigurationInfo>,
    #[doc = "The settings for the CORS configuration of the service instance."]
    #[serde(rename = "corsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<ServiceCorsConfigurationInfo>,
    #[doc = "Export operation configuration information"]
    #[serde(rename = "exportConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub export_configuration: Option<ServiceExportConfigurationInfo>,
    #[doc = "The list of private endpoint connections that are set up for this resource."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties::PublicNetworkAccess>,
    #[doc = "Azure container registry configuration information"]
    #[serde(rename = "acrConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub acr_configuration: Option<ServiceAcrConfigurationInfo>,
}
impl ServicesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_properties {
    use super::*;
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Creating,
        Accepted,
        Verifying,
        Updating,
        Failed,
        Canceled,
        Deprovisioned,
    }
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[doc = "The properties for updating a service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicesPropertiesUpdateParameters {
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<services_properties_update_parameters::PublicNetworkAccess>,
}
impl ServicesPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod services_properties_update_parameters {
    use super::*;
    #[doc = "Control permission for data plane traffic coming from public networks while private endpoint is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[doc = "The common properties of a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicesResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The kind of the service."]
    pub kind: services_resource::Kind,
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An etag associated with the resource, used for optimistic concurrency when editing it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<services_resource::Identity>,
}
impl ServicesResource {
    pub fn new(kind: services_resource::Kind, location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            kind,
            location,
            tags: None,
            etag: None,
            identity: None,
        }
    }
}
pub mod services_resource {
    use super::*;
    #[doc = "The kind of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "fhir")]
        Fhir,
        #[serde(rename = "fhir-Stu3")]
        FhirStu3,
        #[serde(rename = "fhir-R4")]
        FhirR4,
    }
    #[doc = "Setting indicating whether the service has a managed identity associated with it."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Identity {
        #[doc = "The principal ID of the resource identity."]
        #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
        pub principal_id: Option<String>,
        #[doc = "The tenant ID of the resource."]
        #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
        pub tenant_id: Option<String>,
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<identity::Type>,
    }
    impl Identity {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod identity {
        use super::*;
        #[doc = "Type of identity being specified, currently SystemAssigned and None are allowed."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            SystemAssigned,
            None,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
