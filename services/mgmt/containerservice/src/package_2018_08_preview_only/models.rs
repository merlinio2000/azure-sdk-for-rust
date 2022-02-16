#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Profile for enabling a user to access a managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessProfile {
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(rename = "kubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
}
impl AccessProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AgentPoolType represents types of an agent pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AgentPoolType {
    VirtualMachineScaleSets,
    AvailabilitySet,
}
#[doc = "An error response from the Container service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Container service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Container service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for diagnostics on the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceDiagnosticsProfile {
    #[doc = "Profile for diagnostics on the container service VMs."]
    #[serde(rename = "vmDiagnostics")]
    pub vm_diagnostics: ContainerServiceVmDiagnostics,
}
impl ContainerServiceDiagnosticsProfile {
    pub fn new(vm_diagnostics: ContainerServiceVmDiagnostics) -> Self {
        Self { vm_diagnostics }
    }
}
#[doc = "Profile for Linux VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceLinuxProfile {
    #[doc = "The administrator username to use for Linux VMs."]
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[doc = "SSH configuration for Linux-based VMs running on Azure."]
    pub ssh: ContainerServiceSshConfiguration,
}
impl ContainerServiceLinuxProfile {
    pub fn new(admin_username: String, ssh: ContainerServiceSshConfiguration) -> Self {
        Self { admin_username, ssh }
    }
}
#[doc = "Profile for the container service master."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceMasterProfile {
    #[doc = "Number of masters (VMs) in the container service cluster. Allowed values are 1, 3, and 5. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<container_service_master_profile::Count>,
    #[doc = "DNS prefix to be used to create the FQDN for the master pool."]
    #[serde(rename = "dnsPrefix")]
    pub dns_prefix: String,
    #[doc = "Size of agent VMs."]
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[doc = "OS Disk Size in GB to be used to specify the disk size for every machine in this master/agent pool. If you specify 0, it will apply the default osDisk size according to the vmSize specified."]
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[doc = "VNet SubnetID specifies the VNet's subnet identifier."]
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[doc = "FirstConsecutiveStaticIP used to specify the first static ip of masters."]
    #[serde(rename = "firstConsecutiveStaticIP", default, skip_serializing_if = "Option::is_none")]
    pub first_consecutive_static_ip: Option<String>,
    #[doc = "Storage profile specifies what kind of storage used. Choose from StorageAccount and ManagedDisks. Leave it empty, we will choose for you based on the orchestrator choice."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ContainerServiceStorageProfile>,
    #[doc = "FQDN for the master pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl ContainerServiceMasterProfile {
    pub fn new(dns_prefix: String, vm_size: ContainerServiceVmSize) -> Self {
        Self {
            count: None,
            dns_prefix,
            vm_size,
            os_disk_size_gb: None,
            vnet_subnet_id: None,
            first_consecutive_static_ip: None,
            storage_profile: None,
            fqdn: None,
        }
    }
}
pub mod container_service_master_profile {
    use super::*;
    #[doc = "Number of masters (VMs) in the container service cluster. Allowed values are 1, 3, and 5. The default value is 1."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Count {}
}
#[doc = "Profile of network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceNetworkProfile {
    #[doc = "Network plugin used for building Kubernetes network."]
    #[serde(rename = "networkPlugin", default, skip_serializing_if = "Option::is_none")]
    pub network_plugin: Option<container_service_network_profile::NetworkPlugin>,
    #[doc = "Network policy used for building Kubernetes network."]
    #[serde(rename = "networkPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<container_service_network_profile::NetworkPolicy>,
    #[doc = "A CIDR notation IP range from which to assign pod IPs when kubenet is used."]
    #[serde(rename = "podCidr", default, skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<String>,
    #[doc = "A CIDR notation IP range from which to assign service cluster IPs. It must not overlap with any Subnet IP ranges."]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "An IP address assigned to the Kubernetes DNS service. It must be within the Kubernetes service address range specified in serviceCidr."]
    #[serde(rename = "dnsServiceIP", default, skip_serializing_if = "Option::is_none")]
    pub dns_service_ip: Option<String>,
    #[doc = "A CIDR notation IP range assigned to the Docker bridge network. It must not overlap with any Subnet IP ranges or the Kubernetes service address range."]
    #[serde(rename = "dockerBridgeCidr", default, skip_serializing_if = "Option::is_none")]
    pub docker_bridge_cidr: Option<String>,
}
impl ContainerServiceNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_service_network_profile {
    use super::*;
    #[doc = "Network plugin used for building Kubernetes network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NetworkPlugin {
        #[serde(rename = "azure")]
        Azure,
        #[serde(rename = "kubenet")]
        Kubenet,
    }
    impl Default for NetworkPlugin {
        fn default() -> Self {
            Self::Kubenet
        }
    }
    #[doc = "Network policy used for building Kubernetes network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NetworkPolicy {
        #[serde(rename = "calico")]
        Calico,
    }
}
pub type ContainerServiceOsDisk = i32;
#[doc = "SSH configuration for Linux-based VMs running on Azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshConfiguration {
    #[doc = "The list of SSH public keys used to authenticate with Linux-based VMs. Only expect one key specified."]
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<ContainerServiceSshPublicKey>,
}
impl ContainerServiceSshConfiguration {
    pub fn new(public_keys: Vec<ContainerServiceSshPublicKey>) -> Self {
        Self { public_keys }
    }
}
#[doc = "Contains information about SSH certificate public key data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceSshPublicKey {
    #[doc = "Certificate public key used to authenticate with VMs through SSH. The certificate must be in PEM format with or without headers."]
    #[serde(rename = "keyData")]
    pub key_data: String,
}
impl ContainerServiceSshPublicKey {
    pub fn new(key_data: String) -> Self {
        Self { key_data }
    }
}
#[doc = "Storage profile specifies what kind of storage used. Choose from StorageAccount and ManagedDisks. Leave it empty, we will choose for you based on the orchestrator choice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceStorageProfile {
    StorageAccount,
    ManagedDisks,
}
#[doc = "Profile for diagnostics on the container service VMs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceVmDiagnostics {
    #[doc = "Whether the VM diagnostic agent is provisioned on the VM."]
    pub enabled: bool,
    #[doc = "The URI of the storage account where diagnostics are stored."]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl ContainerServiceVmDiagnostics {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            storage_uri: None,
        }
    }
}
#[doc = "Size of agent VMs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerServiceVmSize {
    #[serde(rename = "Standard_A1")]
    StandardA1,
    #[serde(rename = "Standard_A10")]
    StandardA10,
    #[serde(rename = "Standard_A11")]
    StandardA11,
    #[serde(rename = "Standard_A1_v2")]
    StandardA1V2,
    #[serde(rename = "Standard_A2")]
    StandardA2,
    #[serde(rename = "Standard_A2_v2")]
    StandardA2V2,
    #[serde(rename = "Standard_A2m_v2")]
    StandardA2mV2,
    #[serde(rename = "Standard_A3")]
    StandardA3,
    #[serde(rename = "Standard_A4")]
    StandardA4,
    #[serde(rename = "Standard_A4_v2")]
    StandardA4V2,
    #[serde(rename = "Standard_A4m_v2")]
    StandardA4mV2,
    #[serde(rename = "Standard_A5")]
    StandardA5,
    #[serde(rename = "Standard_A6")]
    StandardA6,
    #[serde(rename = "Standard_A7")]
    StandardA7,
    #[serde(rename = "Standard_A8")]
    StandardA8,
    #[serde(rename = "Standard_A8_v2")]
    StandardA8V2,
    #[serde(rename = "Standard_A8m_v2")]
    StandardA8mV2,
    #[serde(rename = "Standard_A9")]
    StandardA9,
    #[serde(rename = "Standard_B2ms")]
    StandardB2ms,
    #[serde(rename = "Standard_B2s")]
    StandardB2s,
    #[serde(rename = "Standard_B4ms")]
    StandardB4ms,
    #[serde(rename = "Standard_B8ms")]
    StandardB8ms,
    #[serde(rename = "Standard_D1")]
    StandardD1,
    #[serde(rename = "Standard_D11")]
    StandardD11,
    #[serde(rename = "Standard_D11_v2")]
    StandardD11V2,
    #[serde(rename = "Standard_D11_v2_Promo")]
    StandardD11V2Promo,
    #[serde(rename = "Standard_D12")]
    StandardD12,
    #[serde(rename = "Standard_D12_v2")]
    StandardD12V2,
    #[serde(rename = "Standard_D12_v2_Promo")]
    StandardD12V2Promo,
    #[serde(rename = "Standard_D13")]
    StandardD13,
    #[serde(rename = "Standard_D13_v2")]
    StandardD13V2,
    #[serde(rename = "Standard_D13_v2_Promo")]
    StandardD13V2Promo,
    #[serde(rename = "Standard_D14")]
    StandardD14,
    #[serde(rename = "Standard_D14_v2")]
    StandardD14V2,
    #[serde(rename = "Standard_D14_v2_Promo")]
    StandardD14V2Promo,
    #[serde(rename = "Standard_D15_v2")]
    StandardD15V2,
    #[serde(rename = "Standard_D16_v3")]
    StandardD16V3,
    #[serde(rename = "Standard_D16s_v3")]
    StandardD16sV3,
    #[serde(rename = "Standard_D1_v2")]
    StandardD1V2,
    #[serde(rename = "Standard_D2")]
    StandardD2,
    #[serde(rename = "Standard_D2_v2")]
    StandardD2V2,
    #[serde(rename = "Standard_D2_v2_Promo")]
    StandardD2V2Promo,
    #[serde(rename = "Standard_D2_v3")]
    StandardD2V3,
    #[serde(rename = "Standard_D2s_v3")]
    StandardD2sV3,
    #[serde(rename = "Standard_D3")]
    StandardD3,
    #[serde(rename = "Standard_D32_v3")]
    StandardD32V3,
    #[serde(rename = "Standard_D32s_v3")]
    StandardD32sV3,
    #[serde(rename = "Standard_D3_v2")]
    StandardD3V2,
    #[serde(rename = "Standard_D3_v2_Promo")]
    StandardD3V2Promo,
    #[serde(rename = "Standard_D4")]
    StandardD4,
    #[serde(rename = "Standard_D4_v2")]
    StandardD4V2,
    #[serde(rename = "Standard_D4_v2_Promo")]
    StandardD4V2Promo,
    #[serde(rename = "Standard_D4_v3")]
    StandardD4V3,
    #[serde(rename = "Standard_D4s_v3")]
    StandardD4sV3,
    #[serde(rename = "Standard_D5_v2")]
    StandardD5V2,
    #[serde(rename = "Standard_D5_v2_Promo")]
    StandardD5V2Promo,
    #[serde(rename = "Standard_D64_v3")]
    StandardD64V3,
    #[serde(rename = "Standard_D64s_v3")]
    StandardD64sV3,
    #[serde(rename = "Standard_D8_v3")]
    StandardD8V3,
    #[serde(rename = "Standard_D8s_v3")]
    StandardD8sV3,
    #[serde(rename = "Standard_DS1")]
    StandardDs1,
    #[serde(rename = "Standard_DS11")]
    StandardDs11,
    #[serde(rename = "Standard_DS11_v2")]
    StandardDs11V2,
    #[serde(rename = "Standard_DS11_v2_Promo")]
    StandardDs11V2Promo,
    #[serde(rename = "Standard_DS12")]
    StandardDs12,
    #[serde(rename = "Standard_DS12_v2")]
    StandardDs12V2,
    #[serde(rename = "Standard_DS12_v2_Promo")]
    StandardDs12V2Promo,
    #[serde(rename = "Standard_DS13")]
    StandardDs13,
    #[serde(rename = "Standard_DS13-2_v2")]
    StandardDs132V2,
    #[serde(rename = "Standard_DS13-4_v2")]
    StandardDs134V2,
    #[serde(rename = "Standard_DS13_v2")]
    StandardDs13V2,
    #[serde(rename = "Standard_DS13_v2_Promo")]
    StandardDs13V2Promo,
    #[serde(rename = "Standard_DS14")]
    StandardDs14,
    #[serde(rename = "Standard_DS14-4_v2")]
    StandardDs144V2,
    #[serde(rename = "Standard_DS14-8_v2")]
    StandardDs148V2,
    #[serde(rename = "Standard_DS14_v2")]
    StandardDs14V2,
    #[serde(rename = "Standard_DS14_v2_Promo")]
    StandardDs14V2Promo,
    #[serde(rename = "Standard_DS15_v2")]
    StandardDs15V2,
    #[serde(rename = "Standard_DS1_v2")]
    StandardDs1V2,
    #[serde(rename = "Standard_DS2")]
    StandardDs2,
    #[serde(rename = "Standard_DS2_v2")]
    StandardDs2V2,
    #[serde(rename = "Standard_DS2_v2_Promo")]
    StandardDs2V2Promo,
    #[serde(rename = "Standard_DS3")]
    StandardDs3,
    #[serde(rename = "Standard_DS3_v2")]
    StandardDs3V2,
    #[serde(rename = "Standard_DS3_v2_Promo")]
    StandardDs3V2Promo,
    #[serde(rename = "Standard_DS4")]
    StandardDs4,
    #[serde(rename = "Standard_DS4_v2")]
    StandardDs4V2,
    #[serde(rename = "Standard_DS4_v2_Promo")]
    StandardDs4V2Promo,
    #[serde(rename = "Standard_DS5_v2")]
    StandardDs5V2,
    #[serde(rename = "Standard_DS5_v2_Promo")]
    StandardDs5V2Promo,
    #[serde(rename = "Standard_E16_v3")]
    StandardE16V3,
    #[serde(rename = "Standard_E16s_v3")]
    StandardE16sV3,
    #[serde(rename = "Standard_E2_v3")]
    StandardE2V3,
    #[serde(rename = "Standard_E2s_v3")]
    StandardE2sV3,
    #[serde(rename = "Standard_E32-16s_v3")]
    StandardE3216sV3,
    #[serde(rename = "Standard_E32-8s_v3")]
    StandardE328sV3,
    #[serde(rename = "Standard_E32_v3")]
    StandardE32V3,
    #[serde(rename = "Standard_E32s_v3")]
    StandardE32sV3,
    #[serde(rename = "Standard_E4_v3")]
    StandardE4V3,
    #[serde(rename = "Standard_E4s_v3")]
    StandardE4sV3,
    #[serde(rename = "Standard_E64-16s_v3")]
    StandardE6416sV3,
    #[serde(rename = "Standard_E64-32s_v3")]
    StandardE6432sV3,
    #[serde(rename = "Standard_E64_v3")]
    StandardE64V3,
    #[serde(rename = "Standard_E64s_v3")]
    StandardE64sV3,
    #[serde(rename = "Standard_E8_v3")]
    StandardE8V3,
    #[serde(rename = "Standard_E8s_v3")]
    StandardE8sV3,
    #[serde(rename = "Standard_F1")]
    StandardF1,
    #[serde(rename = "Standard_F16")]
    StandardF16,
    #[serde(rename = "Standard_F16s")]
    StandardF16s,
    #[serde(rename = "Standard_F16s_v2")]
    StandardF16sV2,
    #[serde(rename = "Standard_F1s")]
    StandardF1s,
    #[serde(rename = "Standard_F2")]
    StandardF2,
    #[serde(rename = "Standard_F2s")]
    StandardF2s,
    #[serde(rename = "Standard_F2s_v2")]
    StandardF2sV2,
    #[serde(rename = "Standard_F32s_v2")]
    StandardF32sV2,
    #[serde(rename = "Standard_F4")]
    StandardF4,
    #[serde(rename = "Standard_F4s")]
    StandardF4s,
    #[serde(rename = "Standard_F4s_v2")]
    StandardF4sV2,
    #[serde(rename = "Standard_F64s_v2")]
    StandardF64sV2,
    #[serde(rename = "Standard_F72s_v2")]
    StandardF72sV2,
    #[serde(rename = "Standard_F8")]
    StandardF8,
    #[serde(rename = "Standard_F8s")]
    StandardF8s,
    #[serde(rename = "Standard_F8s_v2")]
    StandardF8sV2,
    #[serde(rename = "Standard_G1")]
    StandardG1,
    #[serde(rename = "Standard_G2")]
    StandardG2,
    #[serde(rename = "Standard_G3")]
    StandardG3,
    #[serde(rename = "Standard_G4")]
    StandardG4,
    #[serde(rename = "Standard_G5")]
    StandardG5,
    #[serde(rename = "Standard_GS1")]
    StandardGs1,
    #[serde(rename = "Standard_GS2")]
    StandardGs2,
    #[serde(rename = "Standard_GS3")]
    StandardGs3,
    #[serde(rename = "Standard_GS4")]
    StandardGs4,
    #[serde(rename = "Standard_GS4-4")]
    StandardGs44,
    #[serde(rename = "Standard_GS4-8")]
    StandardGs48,
    #[serde(rename = "Standard_GS5")]
    StandardGs5,
    #[serde(rename = "Standard_GS5-16")]
    StandardGs516,
    #[serde(rename = "Standard_GS5-8")]
    StandardGs58,
    #[serde(rename = "Standard_H16")]
    StandardH16,
    #[serde(rename = "Standard_H16m")]
    StandardH16m,
    #[serde(rename = "Standard_H16mr")]
    StandardH16mr,
    #[serde(rename = "Standard_H16r")]
    StandardH16r,
    #[serde(rename = "Standard_H8")]
    StandardH8,
    #[serde(rename = "Standard_H8m")]
    StandardH8m,
    #[serde(rename = "Standard_L16s")]
    StandardL16s,
    #[serde(rename = "Standard_L32s")]
    StandardL32s,
    #[serde(rename = "Standard_L4s")]
    StandardL4s,
    #[serde(rename = "Standard_L8s")]
    StandardL8s,
    #[serde(rename = "Standard_M128-32ms")]
    StandardM12832ms,
    #[serde(rename = "Standard_M128-64ms")]
    StandardM12864ms,
    #[serde(rename = "Standard_M128ms")]
    StandardM128ms,
    #[serde(rename = "Standard_M128s")]
    StandardM128s,
    #[serde(rename = "Standard_M64-16ms")]
    StandardM6416ms,
    #[serde(rename = "Standard_M64-32ms")]
    StandardM6432ms,
    #[serde(rename = "Standard_M64ms")]
    StandardM64ms,
    #[serde(rename = "Standard_M64s")]
    StandardM64s,
    #[serde(rename = "Standard_NC12")]
    StandardNc12,
    #[serde(rename = "Standard_NC12s_v2")]
    StandardNc12sV2,
    #[serde(rename = "Standard_NC12s_v3")]
    StandardNc12sV3,
    #[serde(rename = "Standard_NC24")]
    StandardNc24,
    #[serde(rename = "Standard_NC24r")]
    StandardNc24r,
    #[serde(rename = "Standard_NC24rs_v2")]
    StandardNc24rsV2,
    #[serde(rename = "Standard_NC24rs_v3")]
    StandardNc24rsV3,
    #[serde(rename = "Standard_NC24s_v2")]
    StandardNc24sV2,
    #[serde(rename = "Standard_NC24s_v3")]
    StandardNc24sV3,
    #[serde(rename = "Standard_NC6")]
    StandardNc6,
    #[serde(rename = "Standard_NC6s_v2")]
    StandardNc6sV2,
    #[serde(rename = "Standard_NC6s_v3")]
    StandardNc6sV3,
    #[serde(rename = "Standard_ND12s")]
    StandardNd12s,
    #[serde(rename = "Standard_ND24rs")]
    StandardNd24rs,
    #[serde(rename = "Standard_ND24s")]
    StandardNd24s,
    #[serde(rename = "Standard_ND6s")]
    StandardNd6s,
    #[serde(rename = "Standard_NV12")]
    StandardNv12,
    #[serde(rename = "Standard_NV24")]
    StandardNv24,
    #[serde(rename = "Standard_NV6")]
    StandardNv6,
}
pub type ContainerServiceVnetSubnetId = String;
#[doc = "Profile for Windows VMs in the container service cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerServiceWindowsProfile {
    #[doc = "The administrator username to use for Windows VMs."]
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[doc = "The administrator password to use for Windows VMs."]
    #[serde(rename = "adminPassword")]
    pub admin_password: String,
}
impl ContainerServiceWindowsProfile {
    pub fn new(admin_username: String, admin_password: String) -> Self {
        Self {
            admin_username,
            admin_password,
        }
    }
}
#[doc = "The credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResult {
    #[doc = "The name of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CredentialResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResults {
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kubeconfigs: Vec<CredentialResult>,
}
impl CredentialResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedCluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterProperties>,
}
impl ManagedCluster {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "AADProfile specifies attributes for Azure Active Directory integration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAadProfile {
    #[doc = "The client AAD application ID."]
    #[serde(rename = "clientAppID")]
    pub client_app_id: String,
    #[doc = "The server AAD application ID."]
    #[serde(rename = "serverAppID")]
    pub server_app_id: String,
    #[doc = "The server AAD application secret."]
    #[serde(rename = "serverAppSecret", default, skip_serializing_if = "Option::is_none")]
    pub server_app_secret: Option<String>,
    #[doc = "The AAD tenant ID to use for authentication. If not specified, will use the tenant of the deployment subscription."]
    #[serde(rename = "tenantID", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ManagedClusterAadProfile {
    pub fn new(client_app_id: String, server_app_id: String) -> Self {
        Self {
            client_app_id,
            server_app_id,
            server_app_secret: None,
            tenant_id: None,
        }
    }
}
#[doc = "Managed cluster Access Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAccessProfile {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Profile for enabling a user to access a managed cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessProfile>,
}
impl ManagedClusterAccessProfile {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "A Kubernetes add-on profile for a managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAddonProfile {
    #[doc = "Whether the add-on is enabled or not."]
    pub enabled: bool,
    #[doc = "Key-value pairs for configuring an add-on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}
impl ManagedClusterAddonProfile {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, config: None }
    }
}
#[doc = "Profile for the container service agent pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterAgentPoolProfile {
    #[doc = "Unique name of the agent pool profile in the context of the subscription and resource group."]
    pub name: String,
    #[doc = "Number of agents (VMs) to host docker containers. Allowed values must be in the range of 1 to 100 (inclusive). The default value is 1. "]
    pub count: i32,
    #[doc = "Size of agent VMs."]
    #[serde(rename = "vmSize")]
    pub vm_size: ContainerServiceVmSize,
    #[doc = "OS Disk Size in GB to be used to specify the disk size for every machine in this master/agent pool. If you specify 0, it will apply the default osDisk size according to the vmSize specified."]
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<ContainerServiceOsDisk>,
    #[doc = "VNet SubnetID specifies the VNet's subnet identifier."]
    #[serde(rename = "vnetSubnetID", default, skip_serializing_if = "Option::is_none")]
    pub vnet_subnet_id: Option<ContainerServiceVnetSubnetId>,
    #[doc = "Maximum number of pods that can run on a node."]
    #[serde(rename = "maxPods", default, skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,
    #[doc = "OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Maximum number of nodes for auto-scaling"]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
    #[doc = "Minimum number of nodes for auto-scaling"]
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
    #[doc = "Whether to enable auto-scaler"]
    #[serde(rename = "enableAutoScaling", default, skip_serializing_if = "Option::is_none")]
    pub enable_auto_scaling: Option<bool>,
    #[doc = "AgentPoolType represents types of an agent pool"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AgentPoolType>,
}
impl ManagedClusterAgentPoolProfile {
    pub fn new(name: String, count: i32, vm_size: ContainerServiceVmSize) -> Self {
        Self {
            name,
            count,
            vm_size,
            os_disk_size_gb: None,
            vnet_subnet_id: None,
            max_pods: None,
            os_type: None,
            max_count: None,
            min_count: None,
            enable_auto_scaling: None,
            type_: None,
        }
    }
}
#[doc = "The response from the List Managed Clusters operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterListResult {
    #[doc = "The list of managed clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedCluster>,
    #[doc = "The URL to get the next set of managed cluster results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagedClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available upgrade versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterPoolUpgradeProfile {
    #[doc = "Kubernetes version (major, minor, patch)."]
    #[serde(rename = "kubernetesVersion")]
    pub kubernetes_version: String,
    #[doc = "Pool name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux."]
    #[serde(rename = "osType")]
    pub os_type: OsType,
    #[doc = "List of orchestrator types and versions available for upgrade."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upgrades: Vec<String>,
}
impl ManagedClusterPoolUpgradeProfile {
    pub fn new(kubernetes_version: String, os_type: OsType) -> Self {
        Self {
            kubernetes_version,
            name: None,
            os_type,
            upgrades: Vec::new(),
        }
    }
}
#[doc = "Properties of the managed cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedClusterProperties {
    #[doc = "The current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Version of Kubernetes specified when creating the managed cluster."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "DNS prefix specified when creating the managed cluster."]
    #[serde(rename = "dnsPrefix", default, skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,
    #[doc = "FQDN for the master pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Properties of the agent pool."]
    #[serde(rename = "agentPoolProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ManagedClusterAgentPoolProfile>,
    #[doc = "Profile for Linux VMs in the container service cluster."]
    #[serde(rename = "linuxProfile", default, skip_serializing_if = "Option::is_none")]
    pub linux_profile: Option<ContainerServiceLinuxProfile>,
    #[doc = "Information about a service principal identity for the cluster to use for manipulating Azure APIs."]
    #[serde(rename = "servicePrincipalProfile", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_profile: Option<ManagedClusterServicePrincipalProfile>,
    #[doc = "Profile of managed cluster add-on."]
    #[serde(rename = "addonProfiles", default, skip_serializing_if = "Option::is_none")]
    pub addon_profiles: Option<serde_json::Value>,
    #[doc = "Name of the resource group containing agent pool nodes."]
    #[serde(rename = "nodeResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub node_resource_group: Option<String>,
    #[doc = "Whether to enable Kubernetes Role-Based Access Control."]
    #[serde(rename = "enableRBAC", default, skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,
    #[doc = "Profile of network configuration."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<ContainerServiceNetworkProfile>,
    #[doc = "AADProfile specifies attributes for Azure Active Directory integration."]
    #[serde(rename = "aadProfile", default, skip_serializing_if = "Option::is_none")]
    pub aad_profile: Option<ManagedClusterAadProfile>,
}
impl ManagedClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a service principal identity for the cluster to use for manipulating Azure APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterServicePrincipalProfile {
    #[doc = "The ID for the service principal."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "The secret password associated with the service principal in plain text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl ManagedClusterServicePrincipalProfile {
    pub fn new(client_id: String) -> Self {
        Self { client_id, secret: None }
    }
}
#[doc = "The list of available upgrades for compute pools."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfile {
    #[doc = "Id of upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of upgrade profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of upgrade profile."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Control plane and agent pool upgrade profiles."]
    pub properties: ManagedClusterUpgradeProfileProperties,
}
impl ManagedClusterUpgradeProfile {
    pub fn new(properties: ManagedClusterUpgradeProfileProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Control plane and agent pool upgrade profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedClusterUpgradeProfileProperties {
    #[doc = "The list of available upgrade versions."]
    #[serde(rename = "controlPlaneProfile")]
    pub control_plane_profile: ManagedClusterPoolUpgradeProfile,
    #[doc = "The list of available upgrade versions for agent pools."]
    #[serde(rename = "agentPoolProfiles")]
    pub agent_pool_profiles: Vec<ManagedClusterPoolUpgradeProfile>,
}
impl ManagedClusterUpgradeProfileProperties {
    pub fn new(
        control_plane_profile: ManagedClusterPoolUpgradeProfile,
        agent_pool_profiles: Vec<ManagedClusterPoolUpgradeProfile>,
    ) -> Self {
        Self {
            control_plane_profile,
            agent_pool_profiles,
        }
    }
}
#[doc = "OsType to be used to specify os type. Choose from Linux and Windows. Default to Linux."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Linux,
    Windows,
}
impl Default for OsType {
    fn default() -> Self {
        Self::Linux
    }
}
#[doc = "The List Compute Operation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of compute operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a Compute Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about orchestrator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestratorProfile {
    #[doc = "Orchestrator type."]
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: String,
    #[doc = "Orchestrator version (major, minor, patch)."]
    #[serde(rename = "orchestratorVersion")]
    pub orchestrator_version: String,
}
impl OrchestratorProfile {
    pub fn new(orchestrator_type: String, orchestrator_version: String) -> Self {
        Self {
            orchestrator_type,
            orchestrator_version,
        }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
