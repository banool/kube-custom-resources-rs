// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasegroups.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// CouchbaseGroupSpec allows the specification of Couchbase group configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseGroup", plural = "couchbasegroups")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct CouchbaseGroupSpec {
    /// LDAPGroupRef is a reference to an LDAP group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ldapGroupRef")]
    pub ldap_group_ref: Option<String>,
    /// Roles is a list of roles that this group is granted.
    pub roles: Vec<CouchbaseGroupRoles>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRoles {
    /// Bucket name for bucket admin roles.  When not specified for a role that can be scoped to a specific bucket, the role will apply to all buckets in the cluster. Deprecated:  Couchbase Autonomous Operator 2.3
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Bucket level access to apply to specified role. The bucket must exist.  When not specified, the bucket field will be checked. If both are empty and the role can be scoped to a specific bucket, the role will apply to all buckets in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buckets: Option<CouchbaseGroupRolesBuckets>,
    /// Collection level access to apply to the specified role.  The collection must exist. When not specified, the role is subject to scope or bucket level access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collections: Option<CouchbaseGroupRolesCollections>,
    /// Name of role.
    pub name: CouchbaseGroupRolesName,
    /// Scope level access to apply to specified role.  The scope must exist.  When not specified, the role will apply to selected bucket or all buckets in the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<CouchbaseGroupRolesScopes>,
}

/// Bucket level access to apply to specified role. The bucket must exist.  When not specified, the bucket field will be checked. If both are empty and the role can be scoped to a specific bucket, the role will apply to all buckets in the cluster
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesBuckets {
    /// Resources is an explicit list of named bucket resources that will be considered for inclusion in this role.  If a resource reference doesn't match a resource, then no error conditions are raised due to undefined resource creation ordering and eventual consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CouchbaseGroupRolesBucketsResources>>,
    /// Selector allows resources to be implicitly considered for inclusion in this role.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CouchbaseGroupRolesBucketsSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesBucketsResources {
    /// Kind indicates the kind of resource that is being referenced.  A Role can only reference `CouchbaseBucket` kind.  This field defaults to `CouchbaseBucket` if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CouchbaseGroupRolesBucketsResourcesKind>,
    /// Name is the name of the Kubernetes resource name that is being referenced.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseGroupRolesBucketsResourcesKind {
    CouchbaseBucket,
}

/// Selector allows resources to be implicitly considered for inclusion in this role.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesBucketsSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CouchbaseGroupRolesBucketsSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesBucketsSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Collection level access to apply to the specified role.  The collection must exist. When not specified, the role is subject to scope or bucket level access.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesCollections {
    /// Resources is an explicit list of named resources that will be considered for inclusion in this collection or collections.  If a resource reference doesn't match a resource, then no error conditions are raised due to undefined resource creation ordering and eventual consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CouchbaseGroupRolesCollectionsResources>>,
    /// Selector allows resources to be implicitly considered for inclusion in this collection or collections.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CouchbaseGroupRolesCollectionsSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesCollectionsResources {
    /// Kind indicates the kind of resource that is being referenced.  A scope can only reference `CouchbaseCollection` and `CouchbaseCollectionGroup` resource kinds.  This field defaults to `CouchbaseCollection` if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CouchbaseGroupRolesCollectionsResourcesKind>,
    /// Name is the name of the Kubernetes resource name that is being referenced. Legal collection names have a maximum length of 251 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "_-%".
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseGroupRolesCollectionsResourcesKind {
    CouchbaseCollection,
    CouchbaseCollectionGroup,
}

/// Selector allows resources to be implicitly considered for inclusion in this collection or collections.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesCollectionsSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CouchbaseGroupRolesCollectionsSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesCollectionsSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseGroupRolesName {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "analytics_admin")]
    AnalyticsAdmin,
    #[serde(rename = "analytics_manager")]
    AnalyticsManager,
    #[serde(rename = "analytics_reader")]
    AnalyticsReader,
    #[serde(rename = "analytics_select")]
    AnalyticsSelect,
    #[serde(rename = "backup_admin")]
    BackupAdmin,
    #[serde(rename = "bucket_admin")]
    BucketAdmin,
    #[serde(rename = "bucket_full_access")]
    BucketFullAccess,
    #[serde(rename = "cluster_admin")]
    ClusterAdmin,
    #[serde(rename = "data_backup")]
    DataBackup,
    #[serde(rename = "data_dcp_reader")]
    DataDcpReader,
    #[serde(rename = "data_monitoring")]
    DataMonitoring,
    #[serde(rename = "data_reader")]
    DataReader,
    #[serde(rename = "data_writer")]
    DataWriter,
    #[serde(rename = "eventing_admin")]
    EventingAdmin,
    #[serde(rename = "external_stats_reader")]
    ExternalStatsReader,
    #[serde(rename = "fts_admin")]
    FtsAdmin,
    #[serde(rename = "fts_searcher")]
    FtsSearcher,
    #[serde(rename = "mobile_sync_gateway")]
    MobileSyncGateway,
    #[serde(rename = "mobile_sync_gateway_application")]
    MobileSyncGatewayApplication,
    #[serde(rename = "mobile_sync_gateway_application_read_only")]
    MobileSyncGatewayApplicationReadOnly,
    #[serde(rename = "mobile_sync_gateway_architect")]
    MobileSyncGatewayArchitect,
    #[serde(rename = "mobile_sync_gateway_dev_ops")]
    MobileSyncGatewayDevOps,
    #[serde(rename = "mobile_sync_gateway_replicator")]
    MobileSyncGatewayReplicator,
    #[serde(rename = "query_delete")]
    QueryDelete,
    #[serde(rename = "query_execute_external_functions")]
    QueryExecuteExternalFunctions,
    #[serde(rename = "query_execute_functions")]
    QueryExecuteFunctions,
    #[serde(rename = "query_execute_global_external_functions")]
    QueryExecuteGlobalExternalFunctions,
    #[serde(rename = "query_execute_global_functions")]
    QueryExecuteGlobalFunctions,
    #[serde(rename = "query_external_access")]
    QueryExternalAccess,
    #[serde(rename = "query_insert")]
    QueryInsert,
    #[serde(rename = "query_manage_external_functions")]
    QueryManageExternalFunctions,
    #[serde(rename = "query_manage_functions")]
    QueryManageFunctions,
    #[serde(rename = "query_manage_global_external_functions")]
    QueryManageGlobalExternalFunctions,
    #[serde(rename = "query_manage_global_functions")]
    QueryManageGlobalFunctions,
    #[serde(rename = "query_manage_index")]
    QueryManageIndex,
    #[serde(rename = "query_select")]
    QuerySelect,
    #[serde(rename = "query_system_catalog")]
    QuerySystemCatalog,
    #[serde(rename = "query_update")]
    QueryUpdate,
    #[serde(rename = "replication_admin")]
    ReplicationAdmin,
    #[serde(rename = "replication_target")]
    ReplicationTarget,
    #[serde(rename = "ro_admin")]
    RoAdmin,
    #[serde(rename = "scope_admin")]
    ScopeAdmin,
    #[serde(rename = "security_admin")]
    SecurityAdmin,
    #[serde(rename = "security_admin_external")]
    SecurityAdminExternal,
    #[serde(rename = "security_admin_local")]
    SecurityAdminLocal,
    #[serde(rename = "views_admin")]
    ViewsAdmin,
    #[serde(rename = "views_reader")]
    ViewsReader,
}

/// Scope level access to apply to specified role.  The scope must exist.  When not specified, the role will apply to selected bucket or all buckets in the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesScopes {
    /// Resources is an explicit list of named resources that will be considered for inclusion in this scope or scopes.  If a resource reference doesn't match a resource, then no error conditions are raised due to undefined resource creation ordering and eventual consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CouchbaseGroupRolesScopesResources>>,
    /// Selector allows resources to be implicitly considered for inclusion in this scope or scopes.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CouchbaseGroupRolesScopesSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesScopesResources {
    /// Kind indicates the kind of resource that is being referenced.  A scope can only reference `CouchbaseScope` and `CouchbaseScopeGroup` resource kinds.  This field defaults to `CouchbaseScope` if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CouchbaseGroupRolesScopesResourcesKind>,
    /// Name is the name of the Kubernetes resource name that is being referenced. Legal scope names have a maximum length of 251 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "_-%".
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseGroupRolesScopesResourcesKind {
    CouchbaseScope,
    CouchbaseScopeGroup,
}

/// Selector allows resources to be implicitly considered for inclusion in this scope or scopes.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesScopesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CouchbaseGroupRolesScopesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseGroupRolesScopesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

