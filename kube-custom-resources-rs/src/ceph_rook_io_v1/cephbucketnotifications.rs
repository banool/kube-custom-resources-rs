// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/rook/rook/ceph.rook.io/v1/cephbucketnotifications.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// BucketNotificationSpec represent the spec of a Bucket Notification
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBucketNotification", plural = "cephbucketnotifications")]
#[kube(namespaced)]
#[kube(status = "CephBucketNotificationStatus")]
#[kube(schema = "disabled")]
pub struct CephBucketNotificationSpec {
    /// List of events that should trigger the notification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Spec of notification filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<CephBucketNotificationFilter>,
    /// The name of the topic associated with this notification
    pub topic: String,
}

/// Spec of notification filter
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationFilter {
    /// Filters based on the object's key
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyFilters")]
    pub key_filters: Option<Vec<CephBucketNotificationFilterKeyFilters>>,
    /// Filters based on the object's metadata
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataFilters")]
    pub metadata_filters: Option<Vec<CephBucketNotificationFilterMetadataFilters>>,
    /// Filters based on the object's tags
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagFilters")]
    pub tag_filters: Option<Vec<CephBucketNotificationFilterTagFilters>>,
}

/// NotificationKeyFilterRule represent a single key rule in the Notification Filter spec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationFilterKeyFilters {
    /// Name of the filter - prefix/suffix/regex
    pub name: CephBucketNotificationFilterKeyFiltersName,
    /// Value to filter on
    pub value: String,
}

/// NotificationKeyFilterRule represent a single key rule in the Notification Filter spec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBucketNotificationFilterKeyFiltersName {
    #[serde(rename = "prefix")]
    Prefix,
    #[serde(rename = "suffix")]
    Suffix,
    #[serde(rename = "regex")]
    Regex,
}

/// NotificationFilterRule represent a single rule in the Notification Filter spec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationFilterMetadataFilters {
    /// Name of the metadata or tag
    pub name: String,
    /// Value to filter on
    pub value: String,
}

/// NotificationFilterRule represent a single rule in the Notification Filter spec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationFilterTagFilters {
    /// Name of the metadata or tag
    pub name: String,
    /// Value to filter on
    pub value: String,
}

/// Status represents the status of an object
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CephBucketNotificationStatusConditions>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// Condition represents a status condition on any Rook-Ceph Custom Resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CephBucketNotificationStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHeartbeatTime")]
    pub last_heartbeat_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ConditionReason is a reason for a condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

