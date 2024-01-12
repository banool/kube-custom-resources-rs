// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/image-reflector-controller/image.toolkit.fluxcd.io/v1beta2/imagepolicies.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ImagePolicySpec defines the parameters for calculating the ImagePolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "image.toolkit.fluxcd.io", version = "v1beta2", kind = "ImagePolicy", plural = "imagepolicies")]
#[kube(namespaced)]
#[kube(status = "ImagePolicyStatus")]
#[kube(schema = "disabled")]
pub struct ImagePolicySpec {
    /// FilterTags enables filtering for only a subset of tags based on a set of rules. If no rules are provided, all the tags from the repository will be ordered and compared.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterTags")]
    pub filter_tags: Option<ImagePolicyFilterTags>,
    /// ImageRepositoryRef points at the object specifying the image being scanned
    #[serde(rename = "imageRepositoryRef")]
    pub image_repository_ref: ImagePolicyImageRepositoryRef,
    /// Policy gives the particulars of the policy to be followed in selecting the most recent image
    pub policy: ImagePolicyPolicy,
}

/// FilterTags enables filtering for only a subset of tags based on a set of rules. If no rules are provided, all the tags from the repository will be ordered and compared.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyFilterTags {
    /// Extract allows a capture group to be extracted from the specified regular expression pattern, useful before tag evaluation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract: Option<String>,
    /// Pattern specifies a regular expression pattern used to filter for image tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// ImageRepositoryRef points at the object specifying the image being scanned
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyImageRepositoryRef {
    /// Name of the referent.
    pub name: String,
    /// Namespace of the referent, when not specified it acts as LocalObjectReference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Policy gives the particulars of the policy to be followed in selecting the most recent image
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyPolicy {
    /// Alphabetical set of rules to use for alphabetical ordering of the tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alphabetical: Option<ImagePolicyPolicyAlphabetical>,
    /// Numerical set of rules to use for numerical ordering of the tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numerical: Option<ImagePolicyPolicyNumerical>,
    /// SemVer gives a semantic version range to check against the tags available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<ImagePolicyPolicySemver>,
}

/// Alphabetical set of rules to use for alphabetical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyPolicyAlphabetical {
    /// Order specifies the sorting order of the tags. Given the letters of the alphabet as tags, ascending order would select Z, and descending order would select A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<ImagePolicyPolicyAlphabeticalOrder>,
}

/// Alphabetical set of rules to use for alphabetical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImagePolicyPolicyAlphabeticalOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// Numerical set of rules to use for numerical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyPolicyNumerical {
    /// Order specifies the sorting order of the tags. Given the integer values from 0 to 9 as tags, ascending order would select 9, and descending order would select 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<ImagePolicyPolicyNumericalOrder>,
}

/// Numerical set of rules to use for numerical ordering of the tags.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImagePolicyPolicyNumericalOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// SemVer gives a semantic version range to check against the tags available.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyPolicySemver {
    /// Range gives a semver range for the image tag; the highest version within the range that's a tag yields the latest image.
    pub range: String,
}

/// ImagePolicyStatus defines the observed state of ImagePolicy
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ImagePolicyStatusConditions>>,
    /// LatestImage gives the first in the list of images scanned by the image repository, when filtered and ordered according to the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestImage")]
    pub latest_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ObservedPreviousImage is the observed previous LatestImage. It is used to keep track of the previous and current images.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedPreviousImage")]
    pub observed_previous_image: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImagePolicyStatusConditions {
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
    pub status: ImagePolicyStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImagePolicyStatusConditionsStatus {
    True,
    False,
    Unknown,
}

