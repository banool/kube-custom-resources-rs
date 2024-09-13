// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1alpha4/vsphereclustertemplates.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// VSphereClusterTemplateSpec defines the desired state of VSphereClusterTemplate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha4", kind = "VSphereClusterTemplate", plural = "vsphereclustertemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereClusterTemplateSpec {
    pub template: VSphereClusterTemplateTemplate,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplate {
    /// VSphereClusterSpec defines the desired state of VSphereCluster
    pub spec: VSphereClusterTemplateTemplateSpec,
}

/// VSphereClusterSpec defines the desired state of VSphereCluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<VSphereClusterTemplateTemplateSpecControlPlaneEndpoint>,
    /// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
    /// the identity to use when reconciling the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityRef")]
    pub identity_ref: Option<VSphereClusterTemplateTemplateSpecIdentityRef>,
    /// Server is the address of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Thumbprint is the colon-separated SHA-1 checksum of the given vCenter server's host certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
/// the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecIdentityRef {
    /// Kind of the identity. Can either be VSphereClusterIdentity or Secret
    pub kind: VSphereClusterTemplateTemplateSpecIdentityRefKind,
    /// Name of the identity.
    pub name: String,
}

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
/// the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereClusterTemplateTemplateSpecIdentityRefKind {
    VSphereClusterIdentity,
    Secret,
}

