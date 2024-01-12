// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeedge/kubeedge/devices.kubeedge.io/v1alpha2/devicemodels.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// DeviceModelSpec defines the model / template for a device.It is a blueprint which describes the device capabilities and access mechanism via property visitors.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "devices.kubeedge.io", version = "v1alpha2", kind = "DeviceModel", plural = "devicemodels")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct DeviceModelSpec {
    /// Required: List of device properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<DeviceModelProperties>>,
    /// Required for DMI: Protocol name used by the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// DeviceProperty describes an individual device property / attribute like temperature / humidity etc.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelProperties {
    /// The device property description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required: The device property name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Required: PropertyType represents the type and data validation of the property.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DeviceModelPropertiesType>,
}

/// Required: PropertyType represents the type and data validation of the property.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<DeviceModelPropertiesTypeBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<DeviceModelPropertiesTypeBytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double: Option<DeviceModelPropertiesTypeDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float: Option<DeviceModelPropertiesTypeFloat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<DeviceModelPropertiesTypeInt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<DeviceModelPropertiesTypeString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeBoolean {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeBooleanAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeBooleanAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeBytes {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeBytesAccessMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeBytesAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeDouble {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeDoubleAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// The unit of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeDoubleAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeFloat {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeFloatAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// The unit of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeFloatAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeInt {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeIntAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    /// The unit of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeIntAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceModelPropertiesTypeString {
    /// Required: Access mode of property, ReadWrite or ReadOnly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<DeviceModelPropertiesTypeStringAccessMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceModelPropertiesTypeStringAccessMode {
    ReadWrite,
    ReadOnly,
}

