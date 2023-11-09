// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/fluxcd/image-reflector-controller/image.toolkit.fluxcd.io/v1beta1/imagerepositories.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ImageRepositorySpec defines the parameters for scanning an image repository, e.g., `fluxcd/flux`.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "image.toolkit.fluxcd.io", version = "v1beta1", kind = "ImageRepository", plural = "imagerepositories")]
#[kube(namespaced)]
#[kube(status = "ImageRepositoryStatus")]
#[kube(schema = "disabled")]
pub struct ImageRepositorySpec {
    /// AccessFrom defines an ACL for allowing cross-namespace references to the ImageRepository object based on the caller's namespace labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<ImageRepositoryAccessFrom>,
    /// CertSecretRef can be given the name of a secret containing either or both of 
    ///  - a PEM-encoded client certificate (`certFile`) and private key (`keyFile`); - a PEM-encoded CA certificate (`caFile`) 
    ///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<ImageRepositoryCertSecretRef>,
    /// ExclusionList is a list of regex strings used to exclude certain tags from being stored in the database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusionList")]
    pub exclusion_list: Option<Vec<String>>,
    /// Image is the name of the image repository
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Interval is the length of time to wait between scans of the image repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// SecretRef can be given the name of a secret containing credentials to use for the image registry. The secret should be created with `kubectl create secret docker-registry`, or the equivalent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ImageRepositorySecretRef>,
    /// ServiceAccountName is the name of the Kubernetes ServiceAccount used to authenticate the image pull if the service account has attached pull secrets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// This flag tells the controller to suspend subsequent image scans. It does not apply to already started scans. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for image scanning. Defaults to 'Interval' duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// AccessFrom defines an ACL for allowing cross-namespace references to the ImageRepository object based on the caller's namespace labels.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies. Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<ImageRepositoryAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies. An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// CertSecretRef can be given the name of a secret containing either or both of 
///  - a PEM-encoded client certificate (`certFile`) and private key (`keyFile`); - a PEM-encoded CA certificate (`caFile`) 
///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// SecretRef can be given the name of a secret containing credentials to use for the image registry. The secret should be created with `kubectl create secret docker-registry`, or the equivalent.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ImageRepositoryStatus defines the observed state of ImageRepository
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryStatus {
    /// CanonicalName is the name of the image repository with all the implied bits made explicit; e.g., `docker.io/library/alpine` rather than `alpine`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canonicalImageName")]
    pub canonical_image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ImageRepositoryStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// LastScanResult contains the number of fetched tags.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScanResult")]
    pub last_scan_result: Option<ImageRepositoryStatusLastScanResult>,
    /// ObservedGeneration is the last reconciled generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: ImageRepositoryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImageRepositoryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// LastScanResult contains the number of fetched tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageRepositoryStatusLastScanResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scanTime")]
    pub scan_time: Option<String>,
    #[serde(rename = "tagCount")]
    pub tag_count: i64,
}

