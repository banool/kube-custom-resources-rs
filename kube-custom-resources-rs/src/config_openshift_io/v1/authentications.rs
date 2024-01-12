// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/config.openshift.io/v1/authentications.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec holds user settable values for configuration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "config.openshift.io", version = "v1", kind = "Authentication", plural = "authentications")]
#[kube(status = "AuthenticationStatus")]
#[kube(schema = "disabled")]
pub struct AuthenticationSpec {
    /// oauthMetadata contains the discovery endpoint data for OAuth 2.0 Authorization Server Metadata for an external OAuth server. This discovery document can be viewed from its served location: oc get --raw '/.well-known/oauth-authorization-server' For further details, see the IETF Draft: https://tools.ietf.org/html/draft-ietf-oauth-discovery-04#section-2 If oauthMetadata.name is non-empty, this value has precedence over any metadata reference stored in status. The key "oauthMetadata" is used to locate the data. If specified and the config map or expected key is not found, no metadata is served. If the specified metadata is not valid, no metadata is served. The namespace for this config map is openshift-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oauthMetadata")]
    pub oauth_metadata: Option<AuthenticationOauthMetadata>,
    /// serviceAccountIssuer is the identifier of the bound service account token issuer. The default is https://kubernetes.default.svc WARNING: Updating this field will not result in immediate invalidation of all bound tokens with the previous issuer value. Instead, the tokens issued by previous service account issuer will continue to be trusted for a time period chosen by the platform (currently set to 24h). This time period is subject to change over time. This allows internal components to transition to use new service account issuer without service distruption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountIssuer")]
    pub service_account_issuer: Option<String>,
    /// type identifies the cluster managed, user facing authentication mode in use. Specifically, it manages the component that responds to login attempts. The default is IntegratedOAuth.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<AuthenticationType>,
    /// webhookTokenAuthenticator configures a remote token reviewer. These remote authentication webhooks can be used to verify bearer tokens via the tokenreviews.authentication.k8s.io REST API. This is required to honor bearer tokens that are provisioned by an external authentication service. 
    ///  Can only be set if "Type" is set to "None".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webhookTokenAuthenticator")]
    pub webhook_token_authenticator: Option<AuthenticationWebhookTokenAuthenticator>,
    /// webhookTokenAuthenticators is DEPRECATED, setting it has no effect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webhookTokenAuthenticators")]
    pub webhook_token_authenticators: Option<Vec<AuthenticationWebhookTokenAuthenticators>>,
}

/// oauthMetadata contains the discovery endpoint data for OAuth 2.0 Authorization Server Metadata for an external OAuth server. This discovery document can be viewed from its served location: oc get --raw '/.well-known/oauth-authorization-server' For further details, see the IETF Draft: https://tools.ietf.org/html/draft-ietf-oauth-discovery-04#section-2 If oauthMetadata.name is non-empty, this value has precedence over any metadata reference stored in status. The key "oauthMetadata" is used to locate the data. If specified and the config map or expected key is not found, no metadata is served. If the specified metadata is not valid, no metadata is served. The namespace for this config map is openshift-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationOauthMetadata {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// spec holds user settable values for configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthenticationType {
    #[serde(rename = "")]
    KopiumEmpty,
    None,
    IntegratedOAuth,
}

/// webhookTokenAuthenticator configures a remote token reviewer. These remote authentication webhooks can be used to verify bearer tokens via the tokenreviews.authentication.k8s.io REST API. This is required to honor bearer tokens that are provisioned by an external authentication service. 
///  Can only be set if "Type" is set to "None".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationWebhookTokenAuthenticator {
    /// kubeConfig references a secret that contains kube config file data which describes how to access the remote webhook service. The namespace for the referenced secret is openshift-config. 
    ///  For further details, see: 
    ///  https://kubernetes.io/docs/reference/access-authn-authz/authentication/#webhook-token-authentication 
    ///  The key "kubeConfig" is used to locate the data. If the secret or expected key is not found, the webhook is not honored. If the specified kube config data is not valid, the webhook is not honored.
    #[serde(rename = "kubeConfig")]
    pub kube_config: AuthenticationWebhookTokenAuthenticatorKubeConfig,
}

/// kubeConfig references a secret that contains kube config file data which describes how to access the remote webhook service. The namespace for the referenced secret is openshift-config. 
///  For further details, see: 
///  https://kubernetes.io/docs/reference/access-authn-authz/authentication/#webhook-token-authentication 
///  The key "kubeConfig" is used to locate the data. If the secret or expected key is not found, the webhook is not honored. If the specified kube config data is not valid, the webhook is not honored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationWebhookTokenAuthenticatorKubeConfig {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// deprecatedWebhookTokenAuthenticator holds the necessary configuration options for a remote token authenticator. It's the same as WebhookTokenAuthenticator but it's missing the 'required' validation on KubeConfig field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationWebhookTokenAuthenticators {
    /// kubeConfig contains kube config file data which describes how to access the remote webhook service. For further details, see: https://kubernetes.io/docs/reference/access-authn-authz/authentication/#webhook-token-authentication The key "kubeConfig" is used to locate the data. If the secret or expected key is not found, the webhook is not honored. If the specified kube config data is not valid, the webhook is not honored. The namespace for this secret is determined by the point of use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeConfig")]
    pub kube_config: Option<AuthenticationWebhookTokenAuthenticatorsKubeConfig>,
}

/// kubeConfig contains kube config file data which describes how to access the remote webhook service. For further details, see: https://kubernetes.io/docs/reference/access-authn-authz/authentication/#webhook-token-authentication The key "kubeConfig" is used to locate the data. If the secret or expected key is not found, the webhook is not honored. If the specified kube config data is not valid, the webhook is not honored. The namespace for this secret is determined by the point of use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationWebhookTokenAuthenticatorsKubeConfig {
    /// name is the metadata.name of the referenced secret
    pub name: String,
}

/// status holds observed values from the cluster. They may not be overridden.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationStatus {
    /// integratedOAuthMetadata contains the discovery endpoint data for OAuth 2.0 Authorization Server Metadata for the in-cluster integrated OAuth server. This discovery document can be viewed from its served location: oc get --raw '/.well-known/oauth-authorization-server' For further details, see the IETF Draft: https://tools.ietf.org/html/draft-ietf-oauth-discovery-04#section-2 This contains the observed value based on cluster state. An explicitly set value in spec.oauthMetadata has precedence over this field. This field has no meaning if authentication spec.type is not set to IntegratedOAuth. The key "oauthMetadata" is used to locate the data. If the config map or expected key is not found, no metadata is served. If the specified metadata is not valid, no metadata is served. The namespace for this config map is openshift-config-managed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "integratedOAuthMetadata")]
    pub integrated_o_auth_metadata: Option<AuthenticationStatusIntegratedOAuthMetadata>,
}

/// integratedOAuthMetadata contains the discovery endpoint data for OAuth 2.0 Authorization Server Metadata for the in-cluster integrated OAuth server. This discovery document can be viewed from its served location: oc get --raw '/.well-known/oauth-authorization-server' For further details, see the IETF Draft: https://tools.ietf.org/html/draft-ietf-oauth-discovery-04#section-2 This contains the observed value based on cluster state. An explicitly set value in spec.oauthMetadata has precedence over this field. This field has no meaning if authentication spec.type is not set to IntegratedOAuth. The key "oauthMetadata" is used to locate the data. If the config map or expected key is not found, no metadata is served. If the specified metadata is not valid, no metadata is served. The namespace for this config map is openshift-config-managed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AuthenticationStatusIntegratedOAuthMetadata {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

