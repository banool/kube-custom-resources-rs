// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/CleverCloud/clever-operator/api.clever-cloud.com/v1beta1/pulsars.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "api.clever-cloud.com", version = "v1beta1", kind = "Pulsar", plural = "pulsars")]
#[kube(namespaced)]
#[kube(status = "PulsarStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PulsarSpec {
    pub instance: PulsarInstance,
    pub organisation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PulsarInstance {
    pub region: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PulsarStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

