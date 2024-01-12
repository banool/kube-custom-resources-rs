// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/schemahero/schemahero/schemas.schemahero.io/v1alpha4/tables.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// TableSpec defines the desired state of Table
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "schemas.schemahero.io", version = "v1alpha4", kind = "Table", plural = "tables")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct TableSpec {
    pub database: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seedData")]
    pub seed_data: Option<TableSeedData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cassandra: Option<TableSchemaCassandra>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cockroachdb: Option<TableSchemaCockroachdb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mysql: Option<TableSchemaMysql>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres: Option<TableSchemaPostgres>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rqlite: Option<TableSchemaRqlite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sqlite: Option<TableSchemaSqlite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timescaledb: Option<TableSchemaTimescaledb>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCassandra {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusteringOrder")]
    pub clustering_order: Option<TableSchemaCassandraClusteringOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaCassandraColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableSchemaCassandraProperties>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCassandraClusteringOrder {
    pub column: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDescending")]
    pub is_descending: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCassandraColumns {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isStatic")]
    pub is_static: Option<bool>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCassandraProperties {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bloomFilterFPChance")]
    pub bloom_filter_fp_chance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crcCheckChance")]
    pub crc_check_chance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dcLocalReadRepairChance")]
    pub dc_local_read_repair_chance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultTTL")]
    pub default_ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcGraceSeconds")]
    pub gc_grace_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxIndexInterval")]
    pub max_index_interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memtableFlushPeriodMs")]
    pub memtable_flush_period_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minIndexInterval")]
    pub min_index_interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readRepairChance")]
    pub read_repair_chance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "speculativeRetry")]
    pub speculative_retry: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaCockroachdbColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaCockroachdbForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaCockroachdbIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "json:triggers")]
    pub json_triggers: Option<Vec<TableSchemaCockroachdbJsonTriggers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaCockroachdbColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaCockroachdbColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaCockroachdbForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaCockroachdbJsonTriggers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintTrigger")]
    pub constraint_trigger: Option<bool>,
    pub events: Vec<String>,
    #[serde(rename = "executeProcedure")]
    pub execute_procedure: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachRun")]
    pub for_each_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachStatement")]
    pub for_each_statement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysql {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaMysqlColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultCharset")]
    pub default_charset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaMysqlForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaMysqlIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaMysqlColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaMysqlColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaMysqlForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaMysqlIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgres {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaPostgresColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaPostgresForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaPostgresIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "json:triggers")]
    pub json_triggers: Option<Vec<TableSchemaPostgresJsonTriggers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaPostgresColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaPostgresColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaPostgresForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaPostgresJsonTriggers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintTrigger")]
    pub constraint_trigger: Option<bool>,
    pub events: Vec<String>,
    #[serde(rename = "executeProcedure")]
    pub execute_procedure: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachRun")]
    pub for_each_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachStatement")]
    pub for_each_statement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqlite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaRqliteColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaRqliteForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaRqliteIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaRqliteColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaRqliteColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaRqliteForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaRqliteIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqlite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaSqliteColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaSqliteForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaSqliteIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaSqliteColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaSqliteColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaSqliteForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaSqliteIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableSchemaTimescaledbColumns>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "foreignKeys")]
    pub foreign_keys: Option<Vec<TableSchemaTimescaledbForeignKeys>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hypertable: Option<TableSchemaTimescaledbHypertable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<TableSchemaTimescaledbIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<TableSchemaTimescaledbTriggers>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbColumns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TableSchemaTimescaledbColumnsAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TableSchemaTimescaledbColumnsConstraints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbColumnsAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoIncrement")]
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbColumnsConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNull")]
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbForeignKeys {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onDelete")]
    pub on_delete: Option<String>,
    pub references: TableSchemaTimescaledbForeignKeysReferences,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbForeignKeysReferences {
    pub columns: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbHypertable {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatedSchemaName")]
    pub associated_schema_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatedTablePrefix")]
    pub associated_table_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chunkTimeInterval")]
    pub chunk_time_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<TableSchemaTimescaledbHypertableCompression>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDefaultIndexes")]
    pub create_default_indexes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataNodes")]
    pub data_nodes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ifNotExists")]
    pub if_not_exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "migrateData")]
    pub migrate_data: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberPartitions")]
    pub number_partitions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partitioningColumn")]
    pub partitioning_column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partitioningFunc")]
    pub partitioning_func: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationFactor")]
    pub replication_factor: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<TableSchemaTimescaledbHypertableRetention>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeColumnName")]
    pub time_column_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timePartitioningFunc")]
    pub time_partitioning_func: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbHypertableCompression {
    pub interval: String,
    #[serde(rename = "segmentBy")]
    pub segment_by: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbHypertableRetention {
    pub interval: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbIndexes {
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isUnique")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSchemaTimescaledbTriggers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintTrigger")]
    pub constraint_trigger: Option<bool>,
    pub events: Vec<String>,
    #[serde(rename = "executeProcedure")]
    pub execute_procedure: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachRun")]
    pub for_each_run: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forEachStatement")]
    pub for_each_statement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSeedData {
    pub rows: Vec<TableSeedDataRows>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSeedDataRows {
    pub columns: Vec<TableSeedDataRowsColumns>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSeedDataRowsColumns {
    pub column: String,
    pub value: TableSeedDataRowsColumnsValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSeedDataRowsColumnsValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub str: Option<String>,
}

/// TableStatus defines the observed state of Table
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatus {
    /// We store the SHA of the table spec from the last time we executed a plan to make startup less noisy by skipping re-planning objects that have been planned we cannot use the resourceVersion or generation fields because updating them would cause the object to be modified again
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPlannedTableSpecSHA")]
    pub last_planned_table_spec_sha: Option<String>,
}

