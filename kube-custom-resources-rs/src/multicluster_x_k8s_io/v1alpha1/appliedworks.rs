// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/work-api/multicluster.x-k8s.io/v1alpha1/appliedworks.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec represents the desired configuration of AppliedWork.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "multicluster.x-k8s.io", version = "v1alpha1", kind = "AppliedWork", plural = "appliedworks")]
#[kube(status = "AppliedWorkStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AppliedWorkSpec {
    /// WorkName represents the name of the related work on the hub.
    #[serde(rename = "workName")]
    pub work_name: String,
    /// WorkNamespace represents the namespace of the related work on the hub.
    #[serde(rename = "workNamespace")]
    pub work_namespace: String,
}

/// Status represents the current status of AppliedWork.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppliedWorkStatus {
    /// AppliedResources represents a list of resources defined within the Work that are applied. Only resources with valid GroupVersionResource, namespace, and name are suitable. An item in this slice is deleted when there is no mapped manifest in Work.Spec or by finalizer. The resource relating to the item will also be removed from managed cluster. The deleted resource may still be present until the finalizers for that resource are finished. However, the resource will not be undeleted, so it can be removed from this list and eventual consistency is preserved.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedResources")]
    pub applied_resources: Option<Vec<AppliedWorkStatusAppliedResources>>,
}

/// AppliedResourceMeta represents the group, version, resource, name and namespace of a resource. Since these resources have been created, they must have valid group, version, resource, namespace, and name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppliedWorkStatusAppliedResources {
    /// Group is the group of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the resource, the resource is cluster scoped if the value is empty
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Ordinal represents an index in manifests list, so the condition can still be linked to a manifest even thougth manifest cannot be parsed successfully.
    pub ordinal: i64,
    /// Resource is the resource type of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// UID is set on successful deletion of the Kubernetes resource by controller. The resource might be still visible on the managed cluster after this field is set. It is not directly settable by a client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Version is the version of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

