// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/hcpvaultsecretsapps.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// HCPVaultSecretsAppSpec defines the desired state of HCPVaultSecretsApp
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "HCPVaultSecretsApp", plural = "hcpvaultsecretsapps")]
#[kube(namespaced)]
#[kube(status = "HCPVaultSecretsAppStatus")]
#[kube(schema = "disabled")]
pub struct HCPVaultSecretsAppSpec {
    /// AppName of the Vault Secrets Application that is to be synced.
    #[serde(rename = "appName")]
    pub app_name: String,
    /// Destination provides configuration necessary for syncing the HCP Vault Application secrets to Kubernetes.
    pub destination: HCPVaultSecretsAppDestination,
    /// HCPAuthRef to the HCPAuth resource, can be prefixed with a namespace, eg: `namespaceA/vaultAuthRefB`. If no namespace prefix is provided it will default to the namespace of the HCPAuth CR. If no value is specified for HCPAuthRef the Operator will default to the `default` HCPAuth, configured in its own Kubernetes namespace. HCPAuthRef string `json:"hcpAuthRef,omitempty"`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hcpAuthRef")]
    pub hcp_auth_ref: Option<String>,
    /// RefreshAfter a period of time, in duration notation e.g. 30s, 1m, 24h
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshAfter")]
    pub refresh_after: Option<String>,
    /// RolloutRestartTargets should be configured whenever the application(s) consuming the HCP Vault Secrets App does not support dynamically reloading a rotated secret. In that case one, or more RolloutRestartTarget(s) can be configured here. The Operator will trigger a "rollout-restart" for each target whenever the Vault secret changes between reconciliation events. See RolloutRestartTarget for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutRestartTargets")]
    pub rollout_restart_targets: Option<Vec<HCPVaultSecretsAppRolloutRestartTargets>>,
}

/// Destination provides configuration necessary for syncing the HCP Vault Application secrets to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HCPVaultSecretsAppDestination {
    /// Annotations to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Create the destination Secret. If the Secret already exists this should be set to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    /// Labels to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the Secret
    pub name: String,
    /// Type of Kubernetes Secret. Requires Create to be set to true. Defaults to Opaque.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HCPVaultSecretsAppRolloutRestartTargets {
    pub kind: HCPVaultSecretsAppRolloutRestartTargetsKind,
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HCPVaultSecretsAppRolloutRestartTargetsKind {
    Deployment,
    DaemonSet,
    StatefulSet,
}

/// HCPVaultSecretsAppStatus defines the observed state of HCPVaultSecretsApp
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HCPVaultSecretsAppStatus {
    /// SecretMAC used when deciding whether new Vault secret data should be synced. 
    ///  The controller will compare the "new" HCP Vault Secrets App data to this value using HMAC, if they are different, then the data will be synced to the Destination. 
    ///  The SecretMac is also used to detect drift in the Destination Secret's Data. If drift is detected the data will be synced to the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretMAC")]
    pub secret_mac: Option<String>,
}

