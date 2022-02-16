#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Details of the certificate to be uploaded to the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateRequest {
    #[doc = "Raw certificate data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawCertificateData>,
}
impl CertificateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Name availability input parameters - Resource type and resource name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "Describes the Resource type: Microsoft.RecoveryServices/Vaults"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource name for which availability needs to be checked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CheckNameAvailabilityParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for check name availability API. Resource provider will set availability as true | false."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localized display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryDisplay {
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "ResourceType for which this Operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operations Name itself."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation having details of what operation is about."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ClientDiscoveryDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox log specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForLogSpecification {
    #[doc = "Name of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blobs created in customer storage account per hour"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl ClientDiscoveryForLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox properties in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForProperties {
    #[doc = "Class to represent shoebox service specification in json client discovery."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ClientDiscoveryForServiceSpecification>,
}
impl ClientDiscoveryForProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox service specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForServiceSpecification {
    #[doc = "List of log specifications of this operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<ClientDiscoveryForLogSpecification>,
}
impl ClientDiscoveryForServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations List response which contains list of available APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryResponse {
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
    #[doc = "Link to the next chunk of the response"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClientDiscoveryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryValueForSingleApi {
    #[doc = "Name of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[doc = "The intended executor of the operation;governs the display of the operation in the RBAC UX and the audit logs UX"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class to represent shoebox properties in json client discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClientDiscoveryForProperties>,
}
impl ClientDiscoveryValueForSingleApi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityData {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: identity_data::Type,
}
impl IdentityData {
    pub fn new(type_: identity_data::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod identity_data {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[doc = "Summary of the replication job data for this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobsSummary {
    #[doc = "Count of failed jobs."]
    #[serde(rename = "failedJobs", default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs: Option<i64>,
    #[doc = "Count of suspended jobs."]
    #[serde(rename = "suspendedJobs", default, skip_serializing_if = "Option::is_none")]
    pub suspended_jobs: Option<i64>,
    #[doc = "Count of in-progress jobs."]
    #[serde(rename = "inProgressJobs", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_jobs: Option<i64>,
}
impl JobsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of the replication monitoring data for this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringSummary {
    #[doc = "Count of unhealthy VMs."]
    #[serde(rename = "unHealthyVmCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_vm_count: Option<i64>,
    #[doc = "Count of unhealthy replication providers."]
    #[serde(rename = "unHealthyProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_provider_count: Option<i64>,
    #[doc = "Count of all critical warnings."]
    #[serde(rename = "eventsCount", default, skip_serializing_if = "Option::is_none")]
    pub events_count: Option<i64>,
    #[doc = "Count of all deprecated recovery service providers."]
    #[serde(rename = "deprecatedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub deprecated_provider_count: Option<i64>,
    #[doc = "Count of all the supported recovery service providers."]
    #[serde(rename = "supportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub supported_provider_count: Option<i64>,
    #[doc = "Count of all the unsupported recovery service providers."]
    #[serde(rename = "unsupportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_provider_count: Option<i64>,
}
impl MonitoringSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameInfo {
    #[doc = "Value of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Localized value of usage."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl NameInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tracked resource with location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchTrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patch Resource information, as returned by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchVault {
    #[serde(flatten)]
    pub patch_tracked_resource: PatchTrackedResource,
    #[doc = "Properties of the vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[doc = "Identifies the unique system identifier for each Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
}
impl PatchVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "Gets or sets id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private Endpoint Connection Response Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[doc = "Gets or sets provisioning state of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection::ProvisioningState>,
    #[doc = "The Private Endpoint network resource that is linked to the Private Endpoint connection."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "Gets or sets private link service connection state."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection {
    use super::*;
    #[doc = "Gets or sets provisioning state of the private endpoint connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
        Pending,
    }
}
#[doc = "Information to be stored in Vault properties as an element of privateEndpointConnections List."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionVaultProperties {
    #[doc = "Format of id subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.[Service]/{resource}/{resourceName}/privateEndpointConnections/{connectionName}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Private Endpoint Connection Response Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "Properties of the private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "e.g. Microsoft.RecoveryServices/vaults/privateLinkResources"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "e.g. f9ad6492-33d4-4690-9999-6bfd52a0d081 (Backup) or f9ad6492-33d4-4690-9999-6bfd52a0d082 (SiteRecovery)"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "[backup-ecs1, backup-prot1, backup-prot1b, backup-prot1c, backup-id1]"]
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
#[doc = "Class which represent the stamps associated with the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResources {
    #[doc = "A collection of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "Link to the next chunk of the response"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateLinkResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets or sets private link service connection state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "Gets or sets the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[doc = "Gets or sets description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets actions required."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "Gets or sets the status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
    }
}
#[doc = "Raw certificate data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RawCertificateData {
    #[doc = "Specifies the authentication type."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<raw_certificate_data::AuthType>,
    #[doc = "The base64 encoded certificate raw data string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
impl RawCertificateData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod raw_certificate_data {
    use super::*;
    #[doc = "Specifies the authentication type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthType {
        Invalid,
        #[serde(rename = "ACS")]
        Acs,
        #[serde(rename = "AAD")]
        Aad,
        AccessControlService,
        AzureActiveDirectory,
    }
}
#[doc = "Replication usages of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUsage {
    #[doc = "Summary of the replication monitoring data for this vault."]
    #[serde(rename = "monitoringSummary", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_summary: Option<MonitoringSummary>,
    #[doc = "Summary of the replication job data for this vault."]
    #[serde(rename = "jobsSummary", default, skip_serializing_if = "Option::is_none")]
    pub jobs_summary: Option<JobsSummary>,
    #[doc = "Number of replication protected items for this vault."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[doc = "Number of replication recovery plans for this vault."]
    #[serde(rename = "recoveryPlanCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_plan_count: Option<i64>,
    #[doc = "Number of servers registered to this vault."]
    #[serde(rename = "registeredServersCount", default, skip_serializing_if = "Option::is_none")]
    pub registered_servers_count: Option<i64>,
    #[doc = "The authentication type of recovery service providers in the vault."]
    #[serde(rename = "recoveryServicesProviderAuthType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_provider_auth_type: Option<i64>,
}
impl ReplicationUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication usages for vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationUsageList {
    #[doc = "The list of replication usages for the given vault."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationUsage>,
}
impl ReplicationUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Optional ETag."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate details representing the Vault credentials for AAD."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAadDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[doc = "AAD tenant authority."]
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
    #[doc = "AAD tenant Id."]
    #[serde(rename = "aadTenantId")]
    pub aad_tenant_id: String,
    #[doc = "AAD service principal clientId."]
    #[serde(rename = "servicePrincipalClientId")]
    pub service_principal_client_id: String,
    #[doc = "AAD service principal ObjectId."]
    #[serde(rename = "servicePrincipalObjectId")]
    pub service_principal_object_id: String,
    #[doc = "Azure Management Endpoint Audience."]
    #[serde(rename = "azureManagementEndpointAudience")]
    pub azure_management_endpoint_audience: String,
}
impl ResourceCertificateAndAadDetails {
    pub fn new(
        resource_certificate_details: ResourceCertificateDetails,
        aad_authority: String,
        aad_tenant_id: String,
        service_principal_client_id: String,
        service_principal_object_id: String,
        azure_management_endpoint_audience: String,
    ) -> Self {
        Self {
            resource_certificate_details,
            aad_authority,
            aad_tenant_id,
            service_principal_client_id,
            service_principal_object_id,
            azure_management_endpoint_audience,
        }
    }
}
#[doc = "Certificate details representing the Vault credentials for ACS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAcsDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[doc = "ACS namespace name - tenant for our service."]
    #[serde(rename = "globalAcsNamespace")]
    pub global_acs_namespace: String,
    #[doc = "Acs mgmt host name to connect to."]
    #[serde(rename = "globalAcsHostName")]
    pub global_acs_host_name: String,
    #[doc = "Global ACS namespace RP realm."]
    #[serde(rename = "globalAcsRPRealm")]
    pub global_acs_rp_realm: String,
}
impl ResourceCertificateAndAcsDetails {
    pub fn new(
        resource_certificate_details: ResourceCertificateDetails,
        global_acs_namespace: String,
        global_acs_host_name: String,
        global_acs_rp_realm: String,
    ) -> Self {
        Self {
            resource_certificate_details,
            global_acs_namespace,
            global_acs_host_name,
            global_acs_rp_realm,
        }
    }
}
#[doc = "Certificate details representing the Vault credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateDetails {
    #[doc = "This property will be used as the discriminator for deciding the specific types in the polymorphic chain of types."]
    #[serde(rename = "authType")]
    pub auth_type: String,
    #[doc = "The base64 encoded certificate raw data string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Certificate friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "Resource ID of the vault."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i64>,
    #[doc = "Certificate Subject Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Certificate thumbprint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Certificate Validity start Date time."]
    #[serde(rename = "validFrom", default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[doc = "Certificate Validity End Date time."]
    #[serde(rename = "validTo", default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}
impl ResourceCertificateDetails {
    pub fn new(auth_type: String) -> Self {
        Self {
            auth_type,
            certificate: None,
            friendly_name: None,
            issuer: None,
            resource_id: None,
            subject: None,
            thumbprint: None,
            valid_from: None,
            valid_to: None,
        }
    }
}
#[doc = "Identifies the unique system identifier for each Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The Sku name."]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The Sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        #[serde(rename = "RS0")]
        Rs0,
    }
}
#[doc = "Tracked resource with location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[doc = "Details for upgrading vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeDetails {
    #[doc = "ID of the vault upgrade operation."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "UTC time at which the upgrade operation has started."]
    #[serde(rename = "startTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    #[doc = "UTC time at which the upgrade operation status was last updated."]
    #[serde(rename = "lastUpdatedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_utc: Option<String>,
    #[doc = "UTC time at which the upgrade operation has ended."]
    #[serde(rename = "endTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub end_time_utc: Option<String>,
    #[doc = "Status of the vault upgrade operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upgrade_details::Status>,
    #[doc = "Message to the user containing information about the upgrade operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The way the vault upgrade was triggered."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<upgrade_details::TriggerType>,
    #[doc = "Resource ID of the upgraded vault."]
    #[serde(rename = "upgradedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub upgraded_resource_id: Option<String>,
    #[doc = "Resource ID of the vault before the upgrade."]
    #[serde(rename = "previousResourceId", default, skip_serializing_if = "Option::is_none")]
    pub previous_resource_id: Option<String>,
}
impl UpgradeDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upgrade_details {
    use super::*;
    #[doc = "Status of the vault upgrade operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        InProgress,
        Upgraded,
        Failed,
    }
    #[doc = "The way the vault upgrade was triggered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        UserTriggered,
        ForcedUpgrade,
    }
}
#[doc = "Resource information, as returned by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
    #[doc = "Properties of the vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[doc = "Identifies the unique system identifier for each Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Vault {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "Certificate corresponding to a vault that can be used by clients to register themselves with the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultCertificateResponse {
    #[doc = "Resource name associated with the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type represents the complete path of the form Namespace/ResourceType/ResourceType/..."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource Id represents the complete path to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Certificate details representing the Vault credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceCertificateDetails>,
}
impl VaultCertificateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault extended information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultExtendedInfo {
    #[doc = "Integrity key."]
    #[serde(rename = "integrityKey", default, skip_serializing_if = "Option::is_none")]
    pub integrity_key: Option<String>,
    #[doc = "Encryption key."]
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[doc = "Encryption key thumbprint."]
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[doc = "Algorithm for Vault ExtendedInfo"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}
impl VaultExtendedInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault extended information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultExtendedInfoResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Vault extended information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultExtendedInfo>,
}
impl VaultExtendedInfoResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for a list of Vaults."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VaultList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultProperties {
    #[doc = "Provisioning State."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Details for upgrading vault."]
    #[serde(rename = "upgradeDetails", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_details: Option<UpgradeDetails>,
    #[doc = "List of private endpoint connection."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionVaultProperties>,
    #[doc = "Private endpoint state for backup."]
    #[serde(rename = "privateEndpointStateForBackup", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_backup: Option<vault_properties::PrivateEndpointStateForBackup>,
    #[doc = "Private endpoint state for site recovery."]
    #[serde(rename = "privateEndpointStateForSiteRecovery", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_site_recovery: Option<vault_properties::PrivateEndpointStateForSiteRecovery>,
}
impl VaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_properties {
    use super::*;
    #[doc = "Private endpoint state for backup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateEndpointStateForBackup {
        None,
        Enabled,
    }
    #[doc = "Private endpoint state for site recovery."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateEndpointStateForSiteRecovery {
        None,
        Enabled,
    }
}
#[doc = "Usages of a vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultUsage {
    #[doc = "Unit of the usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<vault_usage::Unit>,
    #[doc = "Quota period of usage."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Next reset time of usage."]
    #[serde(rename = "nextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[doc = "Current value of usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "Limit of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The name of usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NameInfo>,
}
impl VaultUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_usage {
    use super::*;
    #[doc = "Unit of the usage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
    }
}
#[doc = "Usage for vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultUsageList {
    #[doc = "The list of usages for the given vault."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VaultUsage>,
}
impl VaultUsageList {
    pub fn new() -> Self {
        Self::default()
    }
}
