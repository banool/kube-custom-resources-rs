// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/flagger/flagger.app/v1beta1/alertproviders.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AlertProviderSpec defines the desired state of a AlertProvider.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "flagger.app", version = "v1beta1", kind = "AlertProvider", plural = "alertproviders")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AlertProviderSpec {
    /// Hook URL address of this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Alert channel for this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Http/s proxy of this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// Kubernetes secret reference containing the provider address
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<AlertProviderSecretRef>,
    /// Type of this provider
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<AlertProviderType>,
    /// Bot username for this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Kubernetes secret reference containing the provider address
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AlertProviderSecretRef {
    /// Name of the Kubernetes secret
    pub name: String,
}

/// AlertProviderSpec defines the desired state of a AlertProvider.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertProviderType {
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "msteams")]
    Msteams,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "gchat")]
    Gchat,
}

