// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/zone-aware-controllers-for-k8s/zonecontrol.k8s.aws/v1/zoneawareupdates.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ZoneAwareUpdateSpec defines the desired state of ZoneAwareUpdate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "zonecontrol.k8s.aws", version = "v1", kind = "ZoneAwareUpdate", plural = "zoneawareupdates")]
#[kube(namespaced)]
#[kube(status = "ZoneAwareUpdateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ZoneAwareUpdateSpec {
    /// Dryn-run mode that can be used to test the new controller before enable it
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    /// The exponential growth rate in float string. Default value is 2.0. It's possible to disable exponential updates by setting the ExponentialFactor to 0. In this case, the number of pods updated at each step is defined only by the MaxUnavailable param.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exponentialFactor")]
    pub exponential_factor: Option<String>,
    /// Flag to ignore the PauseRolloutAlarm (default false)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreAlarm")]
    pub ignore_alarm: Option<bool>,
    /// Max number (or %) of pods that can be updated at the same time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    /// CW alarm name used to pause/skip updates. Alarm should be on the same account and region.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pauseRolloutAlarm")]
    pub pause_rollout_alarm: Option<String>,
    /// The name of the StatefulSet for which the ZoneAwareUpdate applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statefulset: Option<String>,
}

/// ZoneAwareUpdateStatus defines the observed state of ZoneAwareUpdate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ZoneAwareUpdateStatus {
    /// CurrentRevision indicates the version of the StatefulSet used to generate Pods
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRevision")]
    pub current_revision: Option<String>,
    /// DeletedReplicas is the number of replicas deleted in the last reconcile loop.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletedReplicas")]
    pub deleted_replicas: Option<i32>,
    /// OldReplicas is the number of Pods *per zone* in the CurrentRevision, when there is new UpdateRevision. It becomes zero for all zones when all pods are in the new revision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oldReplicas")]
    pub old_replicas: Option<BTreeMap<String, i32>>,
    /// PausedRollout indicates if the rollout was paused becaused the PauseRolloutAlarm is in alarm.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pausedRollout")]
    pub paused_rollout: Option<bool>,
    /// UpdateRevision indicates the new version of the StatefulSet
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateRevision")]
    pub update_revision: Option<String>,
    /// UpdateStep is used to track the rollout progress. Everytime pods are deleted/updated this is increased. It becomes zero when all pods are in the new revision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStep")]
    pub update_step: Option<i32>,
}

