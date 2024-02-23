// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubev2v/forklift/forklift.konveyor.io/v1beta1/migrations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// MigrationSpec defines the desired state of Migration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "forklift.konveyor.io", version = "v1beta1", kind = "Migration", plural = "migrations")]
#[kube(namespaced)]
#[kube(status = "MigrationStatus")]
#[kube(schema = "disabled")]
pub struct MigrationSpec {
    /// List of VMs which will have their imports canceled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Vec<MigrationCancel>>,
    /// Date and time to finalize a warm migration. If present, this will override the value set on the Plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cutover: Option<String>,
    /// Reference to the associated Plan.
    pub plan: MigrationPlan,
}

/// Source reference. Either the ID or Name must be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationCancel {
    /// The object ID. vsphere: The managed object ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An object Name. vsphere: A qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The VM Namespace Only relevant for an openshift source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Type used to qualify the name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Reference to the associated Plan.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationPlan {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// MigrationStatus defines the observed state of Migration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatus {
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// List of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MigrationStatusConditions>>,
    /// The most recent generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// VM status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vms: Option<Vec<MigrationStatusVms>>,
}

/// Condition
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusConditions {
    /// The condition category.
    pub category: String,
    /// The condition is durable - never un-staged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    /// A list of items referenced in the `Message`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
    /// When the last status transition occurred.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// The human readable description of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition or transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The condition status [true,false].
    pub status: String,
    /// The condition type.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// VM Status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVms {
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// List of conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MigrationStatusVmsConditions>>,
    /// Errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MigrationStatusVmsError>,
    /// The firmware type detected from the OVF file produced by virt-v2v.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    /// Enable hooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<MigrationStatusVmsHooks>>,
    /// The object ID. vsphere: The managed object ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An object Name. vsphere: A qualified name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The VM Namespace Only relevant for an openshift source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Phase
    pub phase: String,
    /// Migration pipeline.
    pub pipeline: Vec<MigrationStatusVmsPipeline>,
    /// Source VM power state before migration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restorePowerState")]
    pub restore_power_state: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// Type used to qualify the name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Warm migration status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warm: Option<MigrationStatusVmsWarm>,
}

/// Condition
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsConditions {
    /// The condition category.
    pub category: String,
    /// The condition is durable - never un-staged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    /// A list of items referenced in the `Message`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
    /// When the last status transition occurred.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// The human readable description of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition or transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The condition status [true,false].
    pub status: String,
    /// The condition type.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Errors
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Plan hook.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsHooks {
    /// Hook reference.
    pub hook: MigrationStatusVmsHooksHook,
    /// Pipeline step.
    pub step: String,
}

/// Hook reference.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsHooksHook {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Pipeline step.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipeline {
    /// Annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MigrationStatusVmsPipelineError>,
    /// Name.
    pub name: String,
    /// Phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Progress.
    pub progress: MigrationStatusVmsPipelineProgress,
    /// Reason
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// Nested tasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<MigrationStatusVmsPipelineTasks>>,
}

/// Error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipelineError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Progress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipelineProgress {
    /// Completed units.
    pub completed: i64,
    /// Total units.
    pub total: i64,
}

/// Migration task.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipelineTasks {
    /// Annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Completed timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MigrationStatusVmsPipelineTasksError>,
    /// Name.
    pub name: String,
    /// Phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Progress.
    pub progress: MigrationStatusVmsPipelineTasksProgress,
    /// Reason
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Started timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
}

/// Error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipelineTasksError {
    pub phase: String,
    pub reasons: Vec<String>,
}

/// Progress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsPipelineTasksProgress {
    /// Completed units.
    pub completed: i64,
    /// Total units.
    pub total: i64,
}

/// Warm migration status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsWarm {
    #[serde(rename = "consecutiveFailures")]
    pub consecutive_failures: i64,
    pub failures: i64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextPrecopyAt")]
    pub next_precopy_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precopies: Option<Vec<MigrationStatusVmsWarmPrecopies>>,
    pub successes: i64,
}

/// Precopy durations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatusVmsWarmPrecopies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

