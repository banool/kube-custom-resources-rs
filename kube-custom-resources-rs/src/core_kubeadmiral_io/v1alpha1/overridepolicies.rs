// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/overridepolicies.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "core.kubeadmiral.io", version = "v1alpha1", kind = "OverridePolicy", plural = "overridepolicies")]
#[kube(namespaced)]
#[kube(status = "OverridePolicyStatus")]
#[kube(schema = "disabled")]
pub struct OverridePolicySpec {
    /// OverrideRules specify the override rules. Each rule specifies the overriders and the clusters these overriders should be applied to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "overrideRules")]
    pub override_rules: Option<Vec<OverridePolicyOverrideRules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRules {
    /// Overriders specify the overriders to be applied in the target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overriders: Option<OverridePolicyOverrideRulesOverriders>,
    /// TargetClusters selects the clusters in which the overriders in this rule should be applied. If multiple types of selectors are specified, the overall result is the intersection of all of them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetClusters")]
    pub target_clusters: Option<OverridePolicyOverrideRulesTargetClusters>,
}

/// Overriders specify the overriders to be applied in the target clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverriders {
    /// Annotation specifies overriders that apply to the resource annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<OverridePolicyOverrideRulesOverridersAnnotations>>,
    /// Args specifies overriders that apply to the container arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<OverridePolicyOverrideRulesOverridersArgs>>,
    /// Command specifies overriders that apply to the container commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<OverridePolicyOverrideRulesOverridersCommand>>,
    /// Image specifies the overriders that apply to the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<OverridePolicyOverrideRulesOverridersImage>>,
    /// JsonPatch specifies overriders in a syntax similar to RFC6902 JSON Patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jsonpatch: Option<Vec<OverridePolicyOverrideRulesOverridersJsonpatch>>,
    /// Label specifies overriders that apply to the resource labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<OverridePolicyOverrideRulesOverridersLabels>>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersAnnotations {
    /// Operator specifies the operation. If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<OverridePolicyOverrideRulesOverridersAnnotationsOperator>,
    /// Value is the value(s) that will be applied to annotations/labels of resource. If Operator is 'addIfAbsent', items in Value (empty is not allowed) will be added in annotations/labels. - For 'addIfAbsent' Operator, the keys in Value cannot conflict with annotations/labels. If Operator is 'overwrite', items in Value which match in annotations/labels will be replaced. If Operator is 'delete', items in Value which match in annotations/labels will be deleted.
    pub value: BTreeMap<String, String>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersAnnotationsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersArgs {
    /// ContainerName targets the specified container or init container in the pod template.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator specifies the operation. If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<OverridePolicyOverrideRulesOverridersArgsOperator>,
    /// Value is the value(s) that will be applied to command/args of ContainerName. If Operator is 'append', items in Value (empty is not allowed) will be appended to command/args. If Operator is 'overwrite', current command/args of ContainerName will be completely replaced by Value. If Operator is 'delete', items in Value that match in command/args will be deleted.
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersArgsOperator {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersCommand {
    /// ContainerName targets the specified container or init container in the pod template.
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Operator specifies the operation. If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<OverridePolicyOverrideRulesOverridersCommandOperator>,
    /// Value is the value(s) that will be applied to command/args of ContainerName. If Operator is 'append', items in Value (empty is not allowed) will be appended to command/args. If Operator is 'overwrite', current command/args of ContainerName will be completely replaced by Value. If Operator is 'delete', items in Value that match in command/args will be deleted.
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersCommandOperator {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersImage {
    /// ContainerNames are ignored when ImagePath is set. If empty, the image override rule applies to all containers. Otherwise, this override targets the specified container(s) or init container(s) in the pod template.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerNames")]
    pub container_names: Option<Vec<String>>,
    /// ImagePath indicates the image path to target. For Example: /spec/template/spec/containers/0/image 
    ///  If empty, the system will automatically resolve the image path if the resource type is Pod, CronJob, Deployment, StatefulSet, DaemonSet or Job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePath")]
    pub image_path: Option<String>,
    /// Operations are the specific operations to be performed on ContainerNames or ImagePath.
    pub operations: Vec<OverridePolicyOverrideRulesOverridersImageOperations>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersImageOperations {
    /// ImageComponent is the part of the image to override.
    #[serde(rename = "imageComponent")]
    pub image_component: OverridePolicyOverrideRulesOverridersImageOperationsImageComponent,
    /// Operator specifies the operation. If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<OverridePolicyOverrideRulesOverridersImageOperationsOperator>,
    /// Value is the value required by the operation. For 'addIfAbsent' Operator, the old value of ImageComponent should be empty, and the Value shouldn't be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersImageOperationsImageComponent {
    Registry,
    Repository,
    Tag,
    Digest,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersImageOperationsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersJsonpatch {
    /// Operator specifies the operation. If omitted, defaults to "replace".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Path is a JSON pointer (RFC 6901) specifying the location within the resource document where the operation is performed. Each key in the path should be prefixed with "/", while "~" and "/" should be escaped as "~0" and "~1" respectively. For example, to add a label "kubeadmiral.io/label", the path should be "/metadata/labels/kubeadmiral.io~1label".
    pub path: String,
    /// Value is the value(s) required by the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, serde_json::Value>>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesOverridersLabels {
    /// Operator specifies the operation. If omitted, defaults to "overwrite".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<OverridePolicyOverrideRulesOverridersLabelsOperator>,
    /// Value is the value(s) that will be applied to annotations/labels of resource. If Operator is 'addIfAbsent', items in Value (empty is not allowed) will be added in annotations/labels. - For 'addIfAbsent' Operator, the keys in Value cannot conflict with annotations/labels. If Operator is 'overwrite', items in Value which match in annotations/labels will be replaced. If Operator is 'delete', items in Value which match in annotations/labels will be deleted.
    pub value: BTreeMap<String, String>,
}

/// StringMapOverrider represents the rules dedicated to handling resource labels/annotations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesOverridersLabelsOperator {
    #[serde(rename = "addIfAbsent")]
    AddIfAbsent,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "delete")]
    Delete,
}

/// TargetClusters selects the clusters in which the overriders in this rule should be applied. If multiple types of selectors are specified, the overall result is the intersection of all of them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClusters {
    /// ClusterAffinity selects FederatedClusters by matching their labels and fields against expressions. If multiple terms are specified, their results are ORed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinity")]
    pub cluster_affinity: Option<Vec<OverridePolicyOverrideRulesTargetClustersClusterAffinity>>,
    /// ClusterSelector selects FederatedClusters by their labels. Empty labels selects all FederatedClusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSelector")]
    pub cluster_selector: Option<BTreeMap<String, String>>,
    /// Clusters selects FederatedClusters by their names. Empty Clusters selects all FederatedClusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClustersClusterAffinity {
    /// A list of cluster selector requirements by cluster labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressions>>,
    /// A list of cluster selector requirements by cluster fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFields>>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressions {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressionsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFields {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFieldsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OverridePolicyOverrideRulesTargetClustersClusterAffinityMatchFieldsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refCount")]
    pub ref_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typedRefCount")]
    pub typed_ref_count: Option<Vec<OverridePolicyStatusTypedRefCount>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OverridePolicyStatusTypedRefCount {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub resource: String,
}

