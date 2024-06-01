// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes/autoscaler/autoscaling.k8s.io/v1beta2/verticalpodautoscalers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Specification of the behavior of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "autoscaling.k8s.io", version = "v1beta2", kind = "VerticalPodAutoscaler", plural = "verticalpodautoscalers")]
#[kube(namespaced)]
#[kube(status = "VerticalPodAutoscalerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VerticalPodAutoscalerSpec {
    /// Controls how the autoscaler computes recommended resources. The resource policy may be used to set constraints on the recommendations for individual containers. If not specified, the autoscaler computes recommended resources for all containers in the pod, without additional constraints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePolicy")]
    pub resource_policy: Option<VerticalPodAutoscalerResourcePolicy>,
    /// TargetRef points to the controller managing the set of pods for the autoscaler to control - e.g. Deployment, StatefulSet. VerticalPodAutoscaler can be targeted at controller implementing scale subresource (the pod set is retrieved from the controller's ScaleStatus) or some well known controllers (e.g. for DaemonSet the pod set is read from the controller's spec). If VerticalPodAutoscaler cannot use specified target it will report ConfigUnsupported condition. Note that VerticalPodAutoscaler does not require full implementation of scale subresource - it will not use it to modify the replica count. The only thing retrieved is a label selector matching pods grouped by the target resource.
    #[serde(rename = "targetRef")]
    pub target_ref: VerticalPodAutoscalerTargetRef,
    /// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatePolicy")]
    pub update_policy: Option<VerticalPodAutoscalerUpdatePolicy>,
}

/// Controls how the autoscaler computes recommended resources. The resource policy may be used to set constraints on the recommendations for individual containers. If not specified, the autoscaler computes recommended resources for all containers in the pod, without additional constraints.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerResourcePolicy {
    /// Per-container resource policies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPolicies")]
    pub container_policies: Option<Vec<VerticalPodAutoscalerResourcePolicyContainerPolicies>>,
}

/// ContainerResourcePolicy controls how autoscaler computes the recommended resources for a specific container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerResourcePolicyContainerPolicies {
    /// Name of the container or DefaultContainerResourcePolicy, in which case the policy is used by the containers that don't have their own policy specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the maximum amount of resources that will be recommended for the container. The default is no maximum.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAllowed")]
    pub max_allowed: Option<BTreeMap<String, IntOrString>>,
    /// Specifies the minimal amount of resources that will be recommended for the container. The default is no minimum.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minAllowed")]
    pub min_allowed: Option<BTreeMap<String, IntOrString>>,
    /// Whether autoscaler is enabled for the container. The default is "Auto".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<VerticalPodAutoscalerResourcePolicyContainerPoliciesMode>,
}

/// ContainerResourcePolicy controls how autoscaler computes the recommended resources for a specific container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VerticalPodAutoscalerResourcePolicyContainerPoliciesMode {
    Auto,
    Off,
}

/// TargetRef points to the controller managing the set of pods for the autoscaler to control - e.g. Deployment, StatefulSet. VerticalPodAutoscaler can be targeted at controller implementing scale subresource (the pod set is retrieved from the controller's ScaleStatus) or some well known controllers (e.g. for DaemonSet the pod set is read from the controller's spec). If VerticalPodAutoscaler cannot use specified target it will report ConfigUnsupported condition. Note that VerticalPodAutoscaler does not require full implementation of scale subresource - it will not use it to modify the replica count. The only thing retrieved is a label selector matching pods grouped by the target resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerTargetRef {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}

/// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerUpdatePolicy {
    /// Controls when autoscaler applies changes to the pod resources. The default is 'Auto'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateMode")]
    pub update_mode: Option<VerticalPodAutoscalerUpdatePolicyUpdateMode>,
}

/// Describes the rules on how changes are applied to the pods. If not specified, all fields in the `PodUpdatePolicy` are set to their default values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VerticalPodAutoscalerUpdatePolicyUpdateMode {
    Off,
    Initial,
    Recreate,
    Auto,
}

/// Current information about the autoscaler.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerStatus {
    /// Conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The most recently computed amount of resources recommended by the autoscaler for the controlled pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<VerticalPodAutoscalerStatusRecommendation>,
}

/// The most recently computed amount of resources recommended by the autoscaler for the controlled pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerStatusRecommendation {
    /// Resources recommended by the autoscaler for each container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecommendations")]
    pub container_recommendations: Option<Vec<VerticalPodAutoscalerStatusRecommendationContainerRecommendations>>,
}

/// RecommendedContainerResources is the recommendation of resources computed by autoscaler for a specific container. Respects the container resource policy if present in the spec. In particular the recommendation is not produced for containers with `ContainerScalingMode` set to 'Off'.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerticalPodAutoscalerStatusRecommendationContainerRecommendations {
    /// Name of the container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Minimum recommended amount of resources. Observes ContainerResourcePolicy. This amount is not guaranteed to be sufficient for the application to operate in a stable way, however running with less resources is likely to have significant impact on performance/availability.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lowerBound")]
    pub lower_bound: Option<BTreeMap<String, IntOrString>>,
    /// Recommended amount of resources. Observes ContainerResourcePolicy.
    pub target: BTreeMap<String, IntOrString>,
    /// The most recent recommended resources target computed by the autoscaler for the controlled pods, based only on actual resource usage, not taking into account the ContainerResourcePolicy. May differ from the Recommendation if the actual resource usage causes the target to violate the ContainerResourcePolicy (lower than MinAllowed or higher that MaxAllowed). Used only as status indication, will not affect actual resource assignment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uncappedTarget")]
    pub uncapped_target: Option<BTreeMap<String, IntOrString>>,
    /// Maximum recommended amount of resources. Observes ContainerResourcePolicy. Any resources allocated beyond this value are likely wasted. This value may be larger than the maximum amount of application is actually capable of consuming.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upperBound")]
    pub upper_bound: Option<BTreeMap<String, IntOrString>>,
}

