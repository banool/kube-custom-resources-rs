// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasememcachedbuckets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CouchbaseMemcachedBucketSpec is the specification for a Memcached bucket resource, and allows the bucket to be customized.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseMemcachedBucket", plural = "couchbasememcachedbuckets")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CouchbaseMemcachedBucketSpec {
    /// EnableFlush defines whether a client can delete all documents in a bucket. This field defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableFlush")]
    pub enable_flush: Option<bool>,
    /// MemoryQuota is a memory limit to the size of a bucket. The memory quota is defined per Couchbase pod running the data service.  This field defaults to, and must be greater than or equal to 100Mi.  More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryQuota")]
    pub memory_quota: Option<String>,
    /// Name is the name of the bucket within Couchbase server.  By default the Operator will use the `metadata.name` field to define the bucket name.  The `metadata.name` field only supports a subset of the supported character set.  When specified, this field overrides `metadata.name`.  Legal bucket names have a maximum length of 100 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "-_%\.".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

