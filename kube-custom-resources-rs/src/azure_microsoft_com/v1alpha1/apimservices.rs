// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/apimservices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ApimServiceSpec defines the desired state of ApimService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "ApimService", plural = "apimservices")]
#[kube(namespaced)]
#[kube(status = "ApimServiceStatus")]
#[kube(schema = "disabled")]
pub struct ApimServiceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appInsightsName")]
    pub app_insights_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appInsightsResourceGroup")]
    pub app_insights_resource_group: Option<String>,
    pub location: String,
    #[serde(rename = "publisherEmail")]
    pub publisher_email: String,
    #[serde(rename = "publisherName")]
    pub publisher_name: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vnetName")]
    pub vnet_name: Option<String>,
    #[serde(rename = "vnetResourceGroup")]
    pub vnet_resource_group: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vnetSubnetName")]
    pub vnet_subnet_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vnetType")]
    pub vnet_type: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApimServiceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containsUpdate")]
    pub contains_update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedProvisioning")]
    pub failed_provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flattenedSecrets")]
    pub flattened_secrets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrl")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrlKind")]
    pub polling_url_kind: Option<ApimServiceStatusPollingUrlKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApimServiceStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

