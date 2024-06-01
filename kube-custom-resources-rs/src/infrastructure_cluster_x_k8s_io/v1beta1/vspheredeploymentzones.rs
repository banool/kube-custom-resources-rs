// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1beta1/vspheredeploymentzones.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VSphereDeploymentZoneSpec defines the desired state of VSphereDeploymentZone.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "VSphereDeploymentZone", plural = "vspheredeploymentzones")]
#[kube(status = "VSphereDeploymentZoneStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereDeploymentZoneSpec {
    /// ControlPlane determines if this failure domain is suitable for use by control plane machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<bool>,
    /// FailureDomain is the name of the VSphereFailureDomain used for this VSphereDeploymentZone
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// PlacementConstraint encapsulates the placement constraints used within this deployment zone.
    #[serde(rename = "placementConstraint")]
    pub placement_constraint: VSphereDeploymentZonePlacementConstraint,
    /// Server is the address of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// PlacementConstraint encapsulates the placement constraints used within this deployment zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereDeploymentZonePlacementConstraint {
    /// Folder is the name or inventory path of the folder in which the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// ResourcePool is the name or inventory path of the resource pool in which the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
}

/// VSphereDeploymentZoneStatus contains the status for a VSphereDeploymentZone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereDeploymentZoneStatus {
    /// Conditions defines current service state of the VSphereMachine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Ready is true when the VSphereDeploymentZone resource is ready. If set to false, it will be ignored by VSphereClusters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

