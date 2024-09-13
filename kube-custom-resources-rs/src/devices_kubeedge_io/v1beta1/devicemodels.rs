// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeedge/kubeedge/devices.kubeedge.io/v1beta1/devicemodels.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DeviceModelSpec defines the model for a device.It is a blueprint which describes the device capabilities and access mechanism via property visitors.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "devices.kubeedge.io", version = "v1beta1", kind = "DeviceModel", plural = "devicemodels")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeviceModelSpec {
    /// Required: List of device properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<DeviceModelProperties>>,
    /// Required: Protocol name used by the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// ModelProperty describes an individual device property / attribute like temperature / humidity etc.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelProperties {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesAccessMode>,
    /// The device property description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    /// Required: The device property name. Note: If you need to use the built-in stream data processing function, you need to define Name as saveFrame or saveVideo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Required: Type of device property, ENUM: INT,FLOAT,DOUBLE,STRING,BOOLEAN,BYTES,STREAM
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DeviceModelPropertiesType>,
    /// The unit of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// ModelProperty describes an individual device property / attribute like temperature / humidity etc.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesAccessMode {
    ReadWrite,
    ReadOnly,
}

/// ModelProperty describes an individual device property / attribute like temperature / humidity etc.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesType {
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "BYTES")]
    Bytes,
    #[serde(rename = "STREAM")]
    Stream,
}

