// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/schemahero/schemahero/schemas.schemahero.io/v1alpha4/migrations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// MigrationSpec defines the desired state of Migration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "schemas.schemahero.io", version = "v1alpha4", kind = "Migration", plural = "migrations")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MigrationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseName")]
    pub database_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "editedDDL")]
    pub edited_ddl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedDDL")]
    pub generated_ddl: Option<String>,
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[serde(rename = "tableNamespace")]
    pub table_namespace: String,
}

/// MigrationStatus defines the observed state of Migration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MigrationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "approvedAt")]
    pub approved_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executedAt")]
    pub executed_at: Option<i64>,
    /// InvalidatedAt is the unix nano timestamp when this plan was determined to be invalid or outdated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "invalidatedAt")]
    pub invalidated_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<MigrationStatusPhase>,
    /// PlannedAt is the unix nano timestamp when the plan was generated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plannedAt")]
    pub planned_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rejectedAt")]
    pub rejected_at: Option<i64>,
}

/// MigrationStatus defines the observed state of Migration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MigrationStatusPhase {
    #[serde(rename = "PLANNED")]
    Planned,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "EXECUTED")]
    Executed,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "REJECTED")]
    Rejected,
}

