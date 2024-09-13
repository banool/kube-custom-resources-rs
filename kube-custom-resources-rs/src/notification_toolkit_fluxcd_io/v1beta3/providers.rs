// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta3/providers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ProviderSpec defines the desired state of the Provider.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta3", kind = "Provider", plural = "providers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ProviderSpec {
    /// Address specifies the endpoint, in a generic sense, to where alerts are sent.
    /// What kind of endpoint depends on the specific Provider type being used.
    /// For the generic Provider, for example, this is an HTTP/S address.
    /// For other Provider types this could be a project ID or a namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// CertSecretRef specifies the Secret containing
    /// a PEM-encoded CA certificate (in the `ca.crt` key).
    /// 
    /// Note: Support for the `caFile` key has
    /// been deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<ProviderCertSecretRef>,
    /// Channel specifies the destination channel where events should be posted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Interval at which to reconcile the Provider with its Secret references.
    /// Deprecated and not used in v1beta3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Proxy the HTTP/S address of the proxy server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// SecretRef specifies the Secret containing the authentication
    /// credentials for this Provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ProviderSecretRef>,
    /// Suspend tells the controller to suspend subsequent
    /// events handling for this Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for sending alerts to the Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Type specifies which Provider implementation to use.
    #[serde(rename = "type")]
    pub r#type: ProviderType,
    /// Username specifies the name under which events are posted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// CertSecretRef specifies the Secret containing
/// a PEM-encoded CA certificate (in the `ca.crt` key).
/// 
/// Note: Support for the `caFile` key has
/// been deprecated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProviderCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// SecretRef specifies the Secret containing the authentication
/// credentials for this Provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProviderSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ProviderSpec defines the desired state of the Provider.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProviderType {
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "msteams")]
    Msteams,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "generic-hmac")]
    GenericHmac,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "bitbucketserver")]
    Bitbucketserver,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "azuredevops")]
    Azuredevops,
    #[serde(rename = "googlechat")]
    Googlechat,
    #[serde(rename = "googlepubsub")]
    Googlepubsub,
    #[serde(rename = "webex")]
    Webex,
    #[serde(rename = "sentry")]
    Sentry,
    #[serde(rename = "azureeventhub")]
    Azureeventhub,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "opsgenie")]
    Opsgenie,
    #[serde(rename = "alertmanager")]
    Alertmanager,
    #[serde(rename = "grafana")]
    Grafana,
    #[serde(rename = "githubdispatch")]
    Githubdispatch,
    #[serde(rename = "pagerduty")]
    Pagerduty,
    #[serde(rename = "datadog")]
    Datadog,
    #[serde(rename = "nats")]
    Nats,
}

