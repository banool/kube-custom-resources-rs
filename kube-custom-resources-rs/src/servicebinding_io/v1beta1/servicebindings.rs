// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/redhat-developer/service-binding-operator/servicebinding.io/v1beta1/servicebindings.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ServiceBindingSpec defines the desired state of ServiceBinding
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "servicebinding.io", version = "v1beta1", kind = "ServiceBinding", plural = "servicebindings")]
#[kube(namespaced)]
#[kube(status = "ServiceBindingStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ServiceBindingSpec {
    /// Env is the collection of mappings from Secret entries to environment variables
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ServiceBindingEnv>>,
    /// Name is the name of the service as projected into the workload container.  Defaults to .metadata.name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Provider is the provider of the service as projected into the workload container
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Service is a reference to an object that fulfills the ProvisionedService duck type
    pub service: ServiceBindingService,
    /// Type is the type of the service as projected into the workload container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Workload is a reference to an object
    pub workload: ServiceBindingWorkload,
}

/// EnvMapping defines a mapping from the value of a Secret entry to an environment variable
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingEnv {
    /// Key is the key in the Secret that will be exposed
    pub key: String,
    /// Name is the name of the environment variable
    pub name: String,
}

/// Service is a reference to an object that fulfills the ProvisionedService duck type
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingService {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// Workload is a reference to an object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingWorkload {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Containers describes which containers in a Pod should be bound to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<String>>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selector is a query that selects the workload or workloads to bind the service to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ServiceBindingWorkloadSelector>,
}

/// Selector is a query that selects the workload or workloads to bind the service to
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingWorkloadSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ServiceBindingWorkloadSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingWorkloadSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ServiceBindingStatus defines the observed state of ServiceBinding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingStatus {
    /// Binding exposes the projected secret for this ServiceBinding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<ServiceBindingStatusBinding>,
    /// Conditions are the conditions of this ServiceBinding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the 'Generation' of the ServiceBinding that was last processed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Binding exposes the projected secret for this ServiceBinding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceBindingStatusBinding {
    /// Name of the referent secret. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

