// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/kubernetes-sigs/security-profiles-operator/security-profiles-operator.x-k8s.io/v1alpha1/profilebindings.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ProfileBindingSpec defines the desired state of ProfileBinding.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "security-profiles-operator.x-k8s.io", version = "v1alpha1", kind = "ProfileBinding", plural = "profilebindings")]
#[kube(namespaced)]
#[kube(status = "ProfileBindingStatus")]
#[kube(schema = "disabled")]
pub struct ProfileBindingSpec {
    /// Image name within pod containers to match to the profile.
    pub image: String,
    /// ProfileRef references a SeccompProfile or other profile type in the current namespace.
    #[serde(rename = "profileRef")]
    pub profile_ref: ProfileBindingProfileRef,
}

/// ProfileRef references a SeccompProfile or other profile type in the current namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileBindingProfileRef {
    /// Kind of object to be bound.
    pub kind: ProfileBindingProfileRefKind,
    /// Name of the profile within the current namespace to which to bind the selected pods.
    pub name: String,
}

/// ProfileRef references a SeccompProfile or other profile type in the current namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProfileBindingProfileRefKind {
    SeccompProfile,
    SelinuxProfile,
}

/// ProfileBindingStatus contains status of the Profilebinding.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileBindingStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeWorkloads")]
    pub active_workloads: Option<Vec<String>>,
}

