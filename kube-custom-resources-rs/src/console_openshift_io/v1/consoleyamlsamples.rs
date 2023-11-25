// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/console.openshift.io/v1/consoleyamlsamples.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ConsoleYAMLSampleSpec is the desired YAML sample configuration. Samples will appear with their descriptions in a samples sidebar when creating a resources in the web console.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "console.openshift.io", version = "v1", kind = "ConsoleYAMLSample", plural = "consoleyamlsamples")]
#[kube(schema = "disabled")]
pub struct ConsoleYAMLSampleSpec {
    /// description of the YAML sample.
    pub description: String,
    /// snippet indicates that the YAML sample is not the full YAML resource definition, but a fragment that can be inserted into the existing YAML document at the user's cursor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<bool>,
    /// targetResource contains apiVersion and kind of the resource YAML sample is representating.
    #[serde(rename = "targetResource")]
    pub target_resource: ConsoleYAMLSampleTargetResource,
    /// title of the YAML sample.
    pub title: String,
    /// yaml is the YAML sample to display.
    pub yaml: String,
}

/// targetResource contains apiVersion and kind of the resource YAML sample is representating.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleYAMLSampleTargetResource {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

