// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/VictoriaMetrics/operator/operator.victoriametrics.com/v1beta1/vmusers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// VMUserSpec defines the desired state of VMUser
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.victoriametrics.com", version = "v1beta1", kind = "VMUser", plural = "vmusers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VMUserSpec {
    /// BearerToken Authorization header value for accessing protected endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bearerToken")]
    pub bearer_token: Option<String>,
    /// DefaultURLs backend url for non-matching paths filter
    /// usually used for default backend with error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_url: Option<Vec<String>>,
    /// DisableSecretCreation skips related secret creation for vmuser
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_secret_creation: Option<bool>,
    /// DiscoverBackendIPs instructs discovering URLPrefix backend IPs via DNS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discover_backend_ips: Option<bool>,
    /// DropSrcPathPrefixParts is the number of `/`-delimited request path prefix parts to drop before proxying the request to backend.
    /// See https://docs.victoriametrics.com/vmauth.html#dropping-request-path-prefix for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_src_path_prefix_parts: Option<i64>,
    /// GeneratePassword instructs operator to generate password for user
    /// if spec.password if empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatePassword")]
    pub generate_password: Option<bool>,
    /// Headers represent additional http headers, that vmauth uses
    /// in form of ["header_key: header_value"]
    /// multiple values for header key:
    /// ["header_key: value1,value2"]
    /// it's available since 1.68.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// IPFilters defines per target src ip filters
    /// supported only with enterprise version of vmauth
    /// https://docs.victoriametrics.com/vmauth.html#ip-filters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_filters: Option<VMUserIpFilters>,
    /// LoadBalancingPolicy defines load balancing policy to use for backend urls.
    /// Supported policies: least_loaded, first_available.
    /// See https://docs.victoriametrics.com/vmauth.html#load-balancing for more details (default "least_loaded")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancing_policy: Option<VMUserLoadBalancingPolicy>,
    /// MaxConcurrentRequests defines max concurrent requests per user
    /// 300 is default value for vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests: Option<i64>,
    /// MetricLabels - additional labels for metrics exported by vmauth for given user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_labels: Option<BTreeMap<String, String>>,
    /// Name of the VMUser object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Password basic auth password for accessing protected endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// PasswordRef allows fetching password from user-create secret by its name and key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordRef")]
    pub password_ref: Option<VMUserPasswordRef>,
    /// ResponseHeaders represent additional http headers, that vmauth adds for request response
    /// in form of ["header_key: header_value"]
    /// multiple values for header key:
    /// ["header_key: value1,value2"]
    /// it's available since 1.93.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<String>>,
    /// RetryStatusCodes defines http status codes in numeric format for request retries
    /// e.g. [429,503]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_status_codes: Option<Vec<i64>>,
    /// TargetRefs - reference to endpoints, which user may access.
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<VMUserTargetRefs>,
    /// TLSConfig specifies TLSConfig configuration parameters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsConfig")]
    pub tls_config: Option<VMUserTlsConfig>,
    /// TokenRef allows fetching token from user-created secrets by its name and key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenRef")]
    pub token_ref: Option<VMUserTokenRef>,
    /// UserName basic auth user name for accessing protected endpoint,
    /// will be replaced with metadata.name of VMUser if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// IPFilters defines per target src ip filters
/// supported only with enterprise version of vmauth
/// https://docs.victoriametrics.com/vmauth.html#ip-filters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserIpFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_list: Option<Vec<String>>,
}

/// VMUserSpec defines the desired state of VMUser
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VMUserLoadBalancingPolicy {
    #[serde(rename = "least_loaded")]
    LeastLoaded,
    #[serde(rename = "first_available")]
    FirstAvailable,
}

/// PasswordRef allows fetching password from user-create secret by its name and key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserPasswordRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TargetRef describes target for user traffic forwarding.
/// one of target types can be chosen:
/// crd or static per targetRef.
/// user can define multiple targetRefs with different ref Types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefs {
    /// CRD describes exist operator's CRD object,
    /// operator generates access url based on CRD params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd: Option<VMUserTargetRefsCrd>,
    /// DiscoverBackendIPs instructs discovering URLPrefix backend IPs via DNS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discover_backend_ips: Option<bool>,
    /// DropSrcPathPrefixParts is the number of `/`-delimited request path prefix parts to drop before proxying the request to backend.
    /// See https://docs.victoriametrics.com/vmauth.html#dropping-request-path-prefix for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_src_path_prefix_parts: Option<i64>,
    /// RequestHeaders represent additional http headers, that vmauth uses
    /// in form of ["header_key: header_value"]
    /// multiple values for header key:
    /// ["header_key: value1,value2"]
    /// it's available since 1.68.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// LoadBalancingPolicy defines load balancing policy to use for backend urls.
    /// Supported policies: least_loaded, first_available.
    /// See https://docs.victoriametrics.com/vmauth.html#load-balancing for more details (default "least_loaded")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancing_policy: Option<VMUserTargetRefsLoadBalancingPolicy>,
    /// Paths - matched path to route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// ResponseHeaders represent additional http headers, that vmauth adds for request response
    /// in form of ["header_key: header_value"]
    /// multiple values for header key:
    /// ["header_key: value1,value2"]
    /// it's available since 1.93.0 version of vmauth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<String>>,
    /// RetryStatusCodes defines http status codes in numeric format for request retries
    /// Can be defined per target or at VMUser.spec level
    /// e.g. [429,503]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_status_codes: Option<Vec<i64>>,
    /// SrcHeaders is an optional list of headers, which must match request headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_headers: Option<Vec<String>>,
    /// SrcQueryArgs is an optional list of query args, which must match request URL query args.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_query_args: Option<Vec<String>>,
    /// Static - user defined url for traffic forward,
    /// for instance http://vmsingle:8429
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "static")]
    pub r#static: Option<VMUserTargetRefsStatic>,
    /// TargetRefBasicAuth allow an target endpoint to authenticate over basic authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefBasicAuth")]
    pub target_ref_basic_auth: Option<VMUserTargetRefsTargetRefBasicAuth>,
    /// TargetPathSuffix allows to add some suffix to the target path
    /// It allows to hide tenant configuration from user with crd as ref.
    /// it also may contain any url encoded params.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_path_suffix: Option<String>,
}

/// CRD describes exist operator's CRD object,
/// operator generates access url based on CRD params.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsCrd {
    /// Kind one of:
    /// VMAgent VMAlert VMCluster VMSingle or VMAlertManager
    pub kind: String,
    /// Name target CRD object name
    pub name: String,
    /// Namespace target CRD object namespace.
    pub namespace: String,
}

/// TargetRef describes target for user traffic forwarding.
/// one of target types can be chosen:
/// crd or static per targetRef.
/// user can define multiple targetRefs with different ref Types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VMUserTargetRefsLoadBalancingPolicy {
    #[serde(rename = "least_loaded")]
    LeastLoaded,
    #[serde(rename = "first_available")]
    FirstAvailable,
}

/// Static - user defined url for traffic forward,
/// for instance http://vmsingle:8429
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsStatic {
    /// URL http url for given staticRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URLs allows setting multiple urls for load-balancing at vmauth-side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// TargetRefBasicAuth allow an target endpoint to authenticate over basic authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsTargetRefBasicAuth {
    /// The secret in the service scrape namespace that contains the password
    /// for authentication.
    /// It must be at them same namespace as CRD
    pub password: VMUserTargetRefsTargetRefBasicAuthPassword,
    /// The secret in the service scrape namespace that contains the username
    /// for authentication.
    /// It must be at them same namespace as CRD
    pub username: VMUserTargetRefsTargetRefBasicAuthUsername,
}

/// The secret in the service scrape namespace that contains the password
/// for authentication.
/// It must be at them same namespace as CRD
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsTargetRefBasicAuthPassword {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The secret in the service scrape namespace that contains the username
/// for authentication.
/// It must be at them same namespace as CRD
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTargetRefsTargetRefBasicAuthUsername {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TLSConfig specifies TLSConfig configuration parameters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfig {
    /// Stuct containing the CA cert to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<VMUserTlsConfigCa>,
    /// Path to the CA cert in the container to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caFile")]
    pub ca_file: Option<String>,
    /// Struct containing the client cert file for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<VMUserTlsConfigCert>,
    /// Path to the client cert file in the container for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certFile")]
    pub cert_file: Option<String>,
    /// Disable target certificate validation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipVerify")]
    pub insecure_skip_verify: Option<bool>,
    /// Path to the client key file in the container for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyFile")]
    pub key_file: Option<String>,
    /// Secret containing the client key file for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySecret")]
    pub key_secret: Option<VMUserTlsConfigKeySecret>,
    /// Used to verify the hostname for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
}

/// Stuct containing the CA cert to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCa {
    /// ConfigMap containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<VMUserTlsConfigCaConfigMap>,
    /// Secret containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<VMUserTlsConfigCaSecret>,
}

/// ConfigMap containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCaConfigMap {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Secret containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCaSecret {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Struct containing the client cert file for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCert {
    /// ConfigMap containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<VMUserTlsConfigCertConfigMap>,
    /// Secret containing data to use for the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<VMUserTlsConfigCertSecret>,
}

/// ConfigMap containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCertConfigMap {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Secret containing data to use for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigCertSecret {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Secret containing the client key file for the targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTlsConfigKeySecret {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TokenRef allows fetching token from user-created secrets by its name and key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserTokenRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// VMUserStatus defines the observed state of VMUser
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VMUserStatus {
}

