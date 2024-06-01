// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/SparebankenVest/azure-key-vault-to-kubernetes/spv.no/v1/azurekeyvaultsecrets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AzureKeyVaultSecretSpec is the spec for a AzureKeyVaultSecret resource
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "spv.no", version = "v1", kind = "AzureKeyVaultSecret", plural = "azurekeyvaultsecrets")]
#[kube(namespaced)]
#[kube(status = "AzureKeyVaultSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct AzureKeyVaultSecretSpec {
    /// AzureKeyVaultOutput defines output sources, currently only support Secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<AzureKeyVaultSecretOutput>,
    /// AzureKeyVault contains information needed to get the Azure Key Vault secret from Azure Key Vault
    pub vault: AzureKeyVaultSecretVault,
}

/// AzureKeyVaultOutput defines output sources, currently only support Secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureKeyVaultSecretOutput {
    /// AzureKeyVaultOutputSecret has information needed to output a secret from Azure Key Vault to Kubernetes as a Secret resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<AzureKeyVaultSecretOutputSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<String>>,
}

/// AzureKeyVaultOutputSecret has information needed to output a secret from Azure Key Vault to Kubernetes as a Secret resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureKeyVaultSecretOutputSecret {
    /// By setting chainOrder to ensureserverfirst the server certificate will be moved first in the chain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chainOrder")]
    pub chain_order: Option<AzureKeyVaultSecretOutputSecretChainOrder>,
    /// The key to use in Kubernetes secret when setting the value from Azure Key Vault object data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataKey")]
    pub data_key: Option<String>,
    /// Name for Kubernetes secret
    pub name: String,
    /// Type of Secret in Kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// AzureKeyVaultOutputSecret has information needed to output a secret from Azure Key Vault to Kubernetes as a Secret resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AzureKeyVaultSecretOutputSecretChainOrder {
    #[serde(rename = "ensureserverfirst")]
    Ensureserverfirst,
}

/// AzureKeyVault contains information needed to get the Azure Key Vault secret from Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AzureKeyVaultSecretVault {
    /// Name of the Azure Key Vault
    pub name: String,
    /// AzureKeyVaultObject has information about the Azure Key Vault object to get from Azure Key Vault
    pub object: AzureKeyVaultSecretVaultObject,
}

/// AzureKeyVaultObject has information about the Azure Key Vault object to get from Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AzureKeyVaultSecretVaultObject {
    /// AzureKeyVaultObjectContentType defines what content type a secret contains, only used when type is multi-key-value-secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentType")]
    pub content_type: Option<AzureKeyVaultSecretVaultObjectContentType>,
    /// The object name in Azure Key Vault
    pub name: String,
    /// AzureKeyVaultObjectType defines which Object type to get from Azure Key Vault
    #[serde(rename = "type")]
    pub r#type: AzureKeyVaultSecretVaultObjectType,
    /// The object version in Azure Key Vault
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// AzureKeyVaultObject has information about the Azure Key Vault object to get from Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AzureKeyVaultSecretVaultObjectContentType {
    #[serde(rename = "application/x-json")]
    ApplicationXJson,
    #[serde(rename = "application/x-yaml")]
    ApplicationXYaml,
}

/// AzureKeyVaultObject has information about the Azure Key Vault object to get from Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AzureKeyVaultSecretVaultObjectType {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "certificate")]
    Certificate,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "multi-key-value-secret")]
    MultiKeyValueSecret,
}

/// AzureKeyVaultSecretStatus is the status for a AzureKeyVaultSecret resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureKeyVaultSecretStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAzureUpdate")]
    pub last_azure_update: Option<String>,
    #[serde(rename = "secretHash")]
    pub secret_hash: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

