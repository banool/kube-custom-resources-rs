// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/dockerdatacenterconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DockerDatacenterConfigSpec defines the desired state of DockerDatacenterConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "DockerDatacenterConfig", plural = "dockerdatacenterconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DockerDatacenterConfigSpec {
}

/// DockerDatacenterConfigStatus defines the observed state of DockerDatacenterConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DockerDatacenterConfigStatus {
}

