// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/autoscaling.karmada.io/v1alpha1/cronfederatedhpas.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec is the specification of the CronFederatedHPA.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "autoscaling.karmada.io", version = "v1alpha1", kind = "CronFederatedHPA", plural = "cronfederatedhpas")]
#[kube(namespaced)]
#[kube(status = "CronFederatedHPAStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CronFederatedHPASpec {
    /// Rules contains a collection of schedules that declares when and how
    /// the referencing target resource should be scaled.
    pub rules: Vec<CronFederatedHPARules>,
    /// ScaleTargetRef points to the target resource to scale.
    /// Target resource could be any resource that implementing the scale
    /// subresource like Deployment, or FederatedHPA.
    #[serde(rename = "scaleTargetRef")]
    pub scale_target_ref: CronFederatedHPAScaleTargetRef,
}

/// CronFederatedHPARule declares a schedule as well as scale actions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPARules {
    /// FailedHistoryLimit represents the count of failed execution items for
    /// each rule.
    /// The value must be a positive integer. It defaults to 3.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedHistoryLimit")]
    pub failed_history_limit: Option<i32>,
    /// Name of the rule.
    /// Each rule in a CronFederatedHPA must have a unique name.
    /// 
    /// 
    /// Note: the name will be used as an identifier to record its execution
    /// history. Changing the name will be considered as deleting the old rule
    /// and adding a new rule, that means the original execution history will be
    /// discarded.
    pub name: String,
    /// Schedule is the cron expression that represents a periodical time.
    /// The syntax follows https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#schedule-syntax.
    pub schedule: String,
    /// SuccessfulHistoryLimit represents the count of successful execution items
    /// for each rule.
    /// The value must be a positive integer. It defaults to 3.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulHistoryLimit")]
    pub successful_history_limit: Option<i32>,
    /// Suspend tells the controller to suspend subsequent executions.
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// TargetMaxReplicas is the target MaxReplicas to be set for FederatedHPA.
    /// Only needed when referencing resource is FederatedHPA.
    /// TargetMinReplicas and TargetMaxReplicas can be specified together or
    /// either one can be specified alone.
    /// nil means the MaxReplicas(.spec.maxReplicas) of the referencing FederatedHPA
    /// will not be updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetMaxReplicas")]
    pub target_max_replicas: Option<i32>,
    /// TargetMinReplicas is the target MinReplicas to be set for FederatedHPA.
    /// Only needed when referencing resource is FederatedHPA.
    /// TargetMinReplicas and TargetMaxReplicas can be specified together or
    /// either one can be specified alone.
    /// nil means the MinReplicas(.spec.minReplicas) of the referencing FederatedHPA
    /// will not be updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetMinReplicas")]
    pub target_min_replicas: Option<i32>,
    /// TargetReplicas is the target replicas to be scaled for resources
    /// referencing by ScaleTargetRef of this CronFederatedHPA.
    /// Only needed when referencing resource is not FederatedHPA.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetReplicas")]
    pub target_replicas: Option<i32>,
    /// TimeZone for the giving schedule.
    /// If not specified, this will default to the time zone of the
    /// karmada-controller-manager process.
    /// Invalid TimeZone will be rejected when applying by karmada-webhook.
    /// see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones for the
    /// all timezones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// ScaleTargetRef points to the target resource to scale.
/// Target resource could be any resource that implementing the scale
/// subresource like Deployment, or FederatedHPA.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPAScaleTargetRef {
    /// apiVersion is the API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// Status is the current status of the CronFederatedHPA.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPAStatus {
    /// ExecutionHistories record the execution histories of CronFederatedHPARule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionHistories")]
    pub execution_histories: Option<Vec<CronFederatedHPAStatusExecutionHistories>>,
}

/// ExecutionHistory records the execution history of specific CronFederatedHPARule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPAStatusExecutionHistories {
    /// FailedExecutions records failed executions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedExecutions")]
    pub failed_executions: Option<Vec<CronFederatedHPAStatusExecutionHistoriesFailedExecutions>>,
    /// NextExecutionTime is the next time to execute.
    /// Nil means the rule has been suspended.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextExecutionTime")]
    pub next_execution_time: Option<String>,
    /// RuleName is the name of the CronFederatedHPARule.
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// SuccessfulExecutions records successful executions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulExecutions")]
    pub successful_executions: Option<Vec<CronFederatedHPAStatusExecutionHistoriesSuccessfulExecutions>>,
}

/// FailedExecution records a failed execution.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPAStatusExecutionHistoriesFailedExecutions {
    /// ExecutionTime is the actual execution time of CronFederatedHPARule.
    /// Tasks may not always be executed at ScheduleTime. ExecutionTime is used
    /// to evaluate the efficiency of the controller's execution.
    #[serde(rename = "executionTime")]
    pub execution_time: String,
    /// Message is the human-readable message indicating details about the failure.
    pub message: String,
    /// ScheduleTime is the expected execution time declared in CronFederatedHPARule.
    #[serde(rename = "scheduleTime")]
    pub schedule_time: String,
}

/// SuccessfulExecution records a successful execution.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronFederatedHPAStatusExecutionHistoriesSuccessfulExecutions {
    /// AppliedMaxReplicas is the MaxReplicas have been applied.
    /// It is required if .spec.rules[*].targetMaxReplicas is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedMaxReplicas")]
    pub applied_max_replicas: Option<i32>,
    /// AppliedMinReplicas is the MinReplicas have been applied.
    /// It is required if .spec.rules[*].targetMinReplicas is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedMinReplicas")]
    pub applied_min_replicas: Option<i32>,
    /// AppliedReplicas is the replicas have been applied.
    /// It is required if .spec.rules[*].targetReplicas is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedReplicas")]
    pub applied_replicas: Option<i32>,
    /// ExecutionTime is the actual execution time of CronFederatedHPARule.
    /// Tasks may not always be executed at ScheduleTime. ExecutionTime is used
    /// to evaluate the efficiency of the controller's execution.
    #[serde(rename = "executionTime")]
    pub execution_time: String,
    /// ScheduleTime is the expected execution time declared in CronFederatedHPARule.
    #[serde(rename = "scheduleTime")]
    pub schedule_time: String,
}

