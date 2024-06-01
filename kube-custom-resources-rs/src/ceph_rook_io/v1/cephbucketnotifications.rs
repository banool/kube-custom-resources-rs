// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephbucketnotifications.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BucketNotificationSpec represent the spec of a Bucket Notification
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBucketNotification", plural = "cephbucketnotifications")]
#[kube(namespaced)]
#[kube(status = "CephBucketNotificationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketNotificationFilterMetadataFilters {
    /// Name of the metadata or tag
    pub name: String,
    /// Value to filter on
    pub value: String,
}

/// NotificationFilterRule represent a single rule in the Notification Filter spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketNotificationFilterTagFilters {
    /// Name of the metadata or tag
    pub name: String,
    /// Value to filter on
    pub value: String,
}

/// Status represents the status of an object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBucketNotificationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

