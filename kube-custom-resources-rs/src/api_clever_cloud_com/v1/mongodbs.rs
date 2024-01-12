// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/CleverCloud/clever-operator/api.clever-cloud.com/v1/mongodbs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "api.clever-cloud.com", version = "v1", kind = "MongoDb", plural = "mongodbs")]
#[kube(namespaced)]
#[kube(status = "MongoDbStatus")]
#[kube(schema = "disabled")]
pub struct MongoDbSpec {
    pub instance: MongoDbInstance,
    pub options: MongoDbOptions,
    pub organisation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MongoDbInstance {
    pub plan: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MongoDbOptions {
    pub encryption: bool,
    pub version: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MongoDbOptionsVersion {
    #[serde(rename = "403")]
    r#_403,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MongoDbStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

