// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/istio/istio/security.istio.io/v1beta1/requestauthentications.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Request authentication configuration for workloads. See more details at: https://istio.io/docs/reference/config/security/request_authentication.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "RequestAuthentication", plural = "requestauthentications")]
#[kube(namespaced)]
#[kube(status = "RequestAuthenticationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RequestAuthenticationSpec {
    /// Define the list of JWTs that can be validated at the selected workloads' proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtRules")]
    pub jwt_rules: Option<Vec<RequestAuthenticationJwtRules>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<RequestAuthenticationSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<RequestAuthenticationTargetRef>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefs")]
    pub target_refs: Option<Vec<RequestAuthenticationTargetRefs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationJwtRules {
    /// The list of JWT [audiences](https://tools.ietf.org/html/rfc7519#section-4.1.3) that are allowed to access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// If set to true, the original token will be kept for the upstream request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardOriginalToken")]
    pub forward_original_token: Option<bool>,
    /// List of cookie names from which JWT is expected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromCookies")]
    pub from_cookies: Option<Vec<String>>,
    /// List of header locations from which JWT is expected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromHeaders")]
    pub from_headers: Option<Vec<RequestAuthenticationJwtRulesFromHeaders>>,
    /// List of query parameters from which JWT is expected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromParams")]
    pub from_params: Option<Vec<String>>,
    /// Identifies the issuer that issued the JWT.
    pub issuer: String,
    /// JSON Web Key Set of public keys to validate signature of the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks: Option<String>,
    /// URL of the provider's public key set to validate signature of the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwksUri")]
    pub jwks_uri: Option<String>,
    /// URL of the provider's public key set to validate signature of the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwks_uri")]
    pub jwks_uri_x: Option<String>,
    /// This field specifies a list of operations to copy the claim to HTTP headers on a successfully verified token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputClaimToHeaders")]
    pub output_claim_to_headers: Option<Vec<RequestAuthenticationJwtRulesOutputClaimToHeaders>>,
    /// This field specifies the header name to output a successfully verified JWT payload to the backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputPayloadToHeader")]
    pub output_payload_to_header: Option<String>,
    /// The maximum amount of time that the resolver, determined by the PILOT_JWT_ENABLE_REMOTE_JWKS environment variable, will spend waiting for the JWKS to be fetched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationJwtRulesFromHeaders {
    /// The HTTP header name.
    pub name: String,
    /// The prefix that should be stripped before decoding the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationJwtRulesOutputClaimToHeaders {
    /// The name of the claim to be copied from.
    pub claim: String,
    /// The name of the header to be created.
    pub header: String,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which a policy should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationTargetRef {
    /// group is the group of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// kind is kind of the target resource.
    pub kind: String,
    /// name is the name of the target resource.
    pub name: String,
    /// namespace is the namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationTargetRefs {
    /// group is the group of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// kind is kind of the target resource.
    pub kind: String,
    /// name is the name of the target resource.
    pub name: String,
    /// namespace is the namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resource Generation to which the Reconciled Condition refers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<RequestAuthenticationStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<RequestAuthenticationStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RequestAuthenticationStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RequestAuthenticationStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RequestAuthenticationStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

