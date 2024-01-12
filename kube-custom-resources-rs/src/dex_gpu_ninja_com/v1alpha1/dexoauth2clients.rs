// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gpu-ninja/dex-operator/dex.gpu-ninja.com/v1alpha1/dexoauth2clients.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// DexOAuth2ClientSpec defines the desired state of the OAuth2 client.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dex.gpu-ninja.com", version = "v1alpha1", kind = "DexOAuth2Client", plural = "dexoauth2clients")]
#[kube(namespaced)]
#[kube(status = "DexOAuth2ClientStatus")]
#[kube(schema = "disabled")]
pub struct DexOAuth2ClientSpec {
    /// IdentityProviderRef is a reference to the identity provider which this client is associated with.
    #[serde(rename = "identityProviderRef")]
    pub identity_provider_ref: DexOAuth2ClientIdentityProviderRef,
    /// LogoURL is the URL to a logo for the client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logoURL")]
    pub logo_url: Option<String>,
    /// Name is the human-readable name of the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public indicates that this client is a public client, such as a mobile app. Public clients must use either use a redirectURL 127.0.0.1:X or "urn:ietf:wg:oauth:2.0:oob".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// RedirectURIs is a list of allowed redirect URLs for the client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectURIs")]
    pub redirect_ur_is: Option<Vec<String>>,
    /// SecretName is the name of the secret that will be created to store the OAuth2 client id and client secret.
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// TrustedPeers are a list of peers which can issue tokens on this client's behalf using the dynamic "oauth2:server:client_id:(client_id)" scope. If a peer makes such a request, this client's ID will appear as the ID Token's audience.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustedPeers")]
    pub trusted_peers: Option<Vec<String>>,
}

/// IdentityProviderRef is a reference to the identity provider which this client is associated with.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DexOAuth2ClientIdentityProviderRef {
    /// Name of the referenced DexIdentityProvider.
    pub name: String,
    /// Namespace is the optional namespace of the referenced DexIdentityProvider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// DexOAuth2ClientStatus defines the observed state of the OAuth2 client.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DexOAuth2ClientStatus {
    /// ObservedGeneration is the most recent generation observed for this OAuth2 client by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Phase is the current phase of the OAuth2 client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Reason is a human readable message indicating details about why the OAuth2 client is in this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

