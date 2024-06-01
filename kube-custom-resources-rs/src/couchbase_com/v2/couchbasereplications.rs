// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasereplications.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// The explicit mappings to use for replication which are optional. For Scopes and Collection replication support we can specify a set of implicit and explicit mappings to use. If none is specified then it is assumed to be existing bucket level replication. https://docs.couchbase.com/server/current/learn/clusters-and-availability/xdcr-with-scopes-and-collections.html#explicit-mapping
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMapping {
    /// The list of explicit replications to carry out including any nested implicit replications: specifying a scope implicitly replicates all collections within it. There should be no duplicates, including more-specific duplicates, e.g. if you specify replication of a scope then you can only deny replication of collections within it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowRules")]
    pub allow_rules: Option<Vec<CouchbaseReplicationExplicitMappingAllowRules>>,
    /// The list of explicit replications to prevent including any nested implicit denials: specifying a scope implicitly denies all collections within it. There should be no duplicates, including more-specific duplicates, e.g. if you specify denial of replication of a scope then you can only specify replication of collections within it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "denyRules")]
    pub deny_rules: Option<Vec<CouchbaseReplicationExplicitMappingDenyRules>>,
}

/// CouchbaseAllowReplicationMapping is to cover Scope and Collection explicit replication. If a scope is defined then it implicitly allows all collections unless a more specific CouchbaseDenyReplicationMapping rule is present to block it. Once a rule is defined at scope level it should not be redefined at collection level. https://docs.couchbase.com/server/current/learn/clusters-and-availability/xdcr-with-scopes-and-collections.html
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMappingAllowRules {
    /// The source keyspace: where to replicate from. Source and target must match whether they have a collection or not, i.e. you cannot replicate from a scope to a collection.
    #[serde(rename = "sourceKeyspace")]
    pub source_keyspace: CouchbaseReplicationExplicitMappingAllowRulesSourceKeyspace,
    /// The target keyspace: where to replicate to. Source and target must match whether they have a collection or not, i.e. you cannot replicate from a scope to a collection.
    #[serde(rename = "targetKeyspace")]
    pub target_keyspace: CouchbaseReplicationExplicitMappingAllowRulesTargetKeyspace,
}

/// The source keyspace: where to replicate from. Source and target must match whether they have a collection or not, i.e. you cannot replicate from a scope to a collection.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMappingAllowRulesSourceKeyspace {
    /// The optional collection within the scope. May be empty to just work at scope level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    /// The scope to use.
    pub scope: String,
}

/// The target keyspace: where to replicate to. Source and target must match whether they have a collection or not, i.e. you cannot replicate from a scope to a collection.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMappingAllowRulesTargetKeyspace {
    /// The optional collection within the scope. May be empty to just work at scope level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    /// The scope to use.
    pub scope: String,
}

/// Provide rules to block implicit replication at scope or collection level. You may want to implicitly map all scopes or collections except a specific one (or set) so this is a better way to express that by creating rules just for those to deny.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMappingDenyRules {
    /// The source keyspace: where to block replication from.
    #[serde(rename = "sourceKeyspace")]
    pub source_keyspace: CouchbaseReplicationExplicitMappingDenyRulesSourceKeyspace,
}

/// The source keyspace: where to block replication from.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseReplicationExplicitMappingDenyRulesSourceKeyspace {
    /// The optional collection within the scope. May be empty to just work at scope level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    /// The scope to use.
    pub scope: String,
}

/// CouchbaseReplicationSpec allows configuration of an XDCR replication.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseReplication", plural = "couchbasereplications")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CouchbaseReplicationSpec {
    /// Bucket is the source bucket to replicate from.  This refers to the Couchbase bucket name, not the resource name of the bucket.  A bucket with this name must be defined on this cluster.  Legal bucket names have a maximum length of 100 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "-_%\.".
    pub bucket: String,
    /// CompressionType is the type of compression to apply to the replication. When None, no compression will be applied to documents as they are transferred between clusters.  When Auto, Couchbase server will automatically compress documents as they are transferred to reduce bandwidth requirements. This field must be one of "None" or "Auto", defaulting to "Auto".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionType")]
    pub compression_type: Option<CouchbaseReplicationCompressionType>,
    /// FilterExpression allows certain documents to be filtered out of the replication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterExpression")]
    pub filter_expression: Option<String>,
    /// Paused allows a replication to be stopped and restarted without having to restart the replication from the beginning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// RemoteBucket is the remote bucket name to synchronize to.  This refers to the Couchbase bucket name, not the resource name of the bucket.  Legal bucket names have a maximum length of 100 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "-_%\.".
    #[serde(rename = "remoteBucket")]
    pub remote_bucket: String,
}

/// CouchbaseReplicationSpec allows configuration of an XDCR replication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseReplicationCompressionType {
    None,
    Auto,
}

