// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/metal3-io/baremetal-operator/metal3.io/v1alpha1/preprovisioningimages.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// PreprovisioningImageSpec defines the desired state of PreprovisioningImage
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "metal3.io", version = "v1alpha1", kind = "PreprovisioningImage", plural = "preprovisioningimages")]
#[kube(namespaced)]
#[kube(status = "PreprovisioningImageStatus")]
#[kube(schema = "disabled")]
pub struct PreprovisioningImageSpec {
    /// acceptFormats is a list of acceptable image formats.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acceptFormats")]
    pub accept_formats: Option<Vec<String>>,
    /// architecture is the processor architecture for which to build the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// networkDataName is the name of a Secret in the local namespace that contains network data to build in to the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkDataName")]
    pub network_data_name: Option<String>,
}

/// PreprovisioningImageStatus defines the observed state of PreprovisioningImage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PreprovisioningImageStatus {
    /// architecture is the processor architecture for which the image is built
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// conditions describe the state of the built image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<PreprovisioningImageStatusConditions>>,
    /// extraKernelParams is a string with extra parameters to pass to the kernel when booting the image over network. Only makes sense for initrd images.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraKernelParams")]
    pub extra_kernel_params: Option<String>,
    /// format is the type of image that is available at the download url: either iso or initrd.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<PreprovisioningImageStatusFormat>,
    /// imageUrl is the URL from which the built image can be downloaded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageUrl")]
    pub image_url: Option<String>,
    /// kernelUrl is the URL from which the kernel of the image can be downloaded. Only makes sense for initrd images.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kernelUrl")]
    pub kernel_url: Option<String>,
    /// networkData is a reference to the version of the Secret containing the network data used to build the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkData")]
    pub network_data: Option<PreprovisioningImageStatusNetworkData>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PreprovisioningImageStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: PreprovisioningImageStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreprovisioningImageStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// PreprovisioningImageStatus defines the observed state of PreprovisioningImage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreprovisioningImageStatusFormat {
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "initrd")]
    Initrd,
}

/// networkData is a reference to the version of the Secret containing the network data used to build the image.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PreprovisioningImageStatusNetworkData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

