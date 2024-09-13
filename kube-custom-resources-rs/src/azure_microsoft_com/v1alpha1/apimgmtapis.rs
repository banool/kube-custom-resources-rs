// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/apimgmtapis.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// APIMgmtSpec defines the desired state of APIMgmt
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "APIMgmtAPI", plural = "apimgmtapis")]
#[kube(namespaced)]
#[kube(status = "APIMgmtAPIStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APIMgmtAPISpec {
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "apiService")]
    pub api_service: String,
    pub location: String,
    pub properties: APIMgmtAPIProperties,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIMgmtAPIProperties {
    /// APIRevision - Describes the Revision of the Api. If no value is provided, default revision 1 is created
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiRevision")]
    pub api_revision: Option<String>,
    /// APIRevisionDescription - Description of the Api Revision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiRevisionDescription")]
    pub api_revision_description: Option<String>,
    /// APIVersion - Indicates the Version identifier of the API if the API is versioned
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// APIVersionDescription - Description of the Api Version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersionDescription")]
    pub api_version_description: Option<String>,
    /// APIVersionSetID - A resource identifier for the related ApiVersionSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersionSetId")]
    pub api_version_set_id: Option<String>,
    /// APIVersionSet - APIVersionSetContractDetails an API Version Set contains the common configuration for a set of API versions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersionSets")]
    pub api_version_sets: Option<APIMgmtAPIPropertiesApiVersionSets>,
    /// Description - Description of the API. May include HTML formatting tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// DisplayName - API name. Must be 1 to 300 characters long.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// Format - Format of the Content in which the API is getting imported. Possible values include: 'WadlXML', 'WadlLinkJSON', 'SwaggerJSON', 'SwaggerLinkJSON', 'Wsdl', 'WsdlLink', 'Openapi', 'Openapijson', 'OpenapiLink'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// IsCurrent - Indicates if API revision is current api revision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCurrent")]
    pub is_current: Option<bool>,
    /// IsOnline - READ-ONLY; Indicates if API revision is accessible via the gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isOnline")]
    pub is_online: Option<bool>,
    /// Path - Relative URL uniquely identifying this API and all of its resource paths within the API Management service instance. It is appended to the API endpoint base URL specified during the service instance creation to form a public URL for this API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Protocols - Describes on which protocols the operations in this API can be invoked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// ServiceURL - Absolute URL of the backend service implementing this API. Cannot be more than 2000 characters long.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceUrl")]
    pub service_url: Option<String>,
    /// SourceAPIID - API identifier of the source API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceApiId")]
    pub source_api_id: Option<String>,
    /// SubscriptionRequired - Specifies whether an API or Product subscription is required for accessing the API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionRequired")]
    pub subscription_required: Option<bool>,
}

/// APIVersionSet - APIVersionSetContractDetails an API Version Set contains the common configuration for a set of API versions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIMgmtAPIPropertiesApiVersionSets {
    /// Description - Description of API Version Set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID - Identifier for existing API Version Set. Omit this value to create a new Version Set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name - The display Name of the API Version Set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIMgmtAPIStatus {
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
    pub polling_url_kind: Option<APIMgmtAPIStatusPollingUrlKind>,
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
pub enum APIMgmtAPIStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

