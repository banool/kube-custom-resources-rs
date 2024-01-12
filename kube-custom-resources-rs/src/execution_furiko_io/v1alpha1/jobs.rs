// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/furiko-io/furiko/execution.furiko.io/v1alpha1/jobs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// JobSpec defines the desired state of a Job.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "execution.furiko.io", version = "v1alpha1", kind = "Job", plural = "jobs")]
#[kube(namespaced)]
#[kube(status = "JobStatus")]
#[kube(schema = "disabled")]
pub struct JobSpec {
    /// ConfigName allows specifying the name of the JobConfig to create the Job from. The JobConfig must be in the same namespace as the Job. 
    ///  It is provided as a write-only input field for convenience, and will override the template, labels and annotations from the JobConfig's template. 
    ///  This field will never be returned from the API. To look up the parent JobConfig, use ownerReferences.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configName")]
    pub config_name: Option<String>,
    /// Specifies the time to start killing the job. When the time passes this timestamp, the controller will start attempting to kill all tasks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "killTimestamp")]
    pub kill_timestamp: Option<String>,
    /// Specifies key-values pairs of values for Options, in JSON or YAML format. 
    ///  Example specification: 
    ///  spec: optionValues: |- myStringOption: "value" myBoolOption: true mySelectOption: - option1 - option3 
    ///  Each entry in the optionValues struct should consist of the option's name, and the value could be an arbitrary type that corresponds to the option's type itself. Each option value specified will be evaluated to a string based on the JobConfig's OptionsSpec and added to Substitutions. If the key also exists in Substitutions, that one takes priority. 
    ///  Cannot be updated after creation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optionValues")]
    pub option_values: Option<String>,
    /// Specifies optional start policy for a Job, which specifies certain conditions which have to be met before a Job is started.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startPolicy")]
    pub start_policy: Option<JobStartPolicy>,
    /// Defines key-value pairs of context variables to be substituted into the TaskTemplate. Each entry should consist of the full context variable name (i.e. `ctx.name`), and the values must be a string. Substitutions defined here take highest precedence over both predefined context variables and evaluated OptionValues. 
    ///  Most users should be using OptionValues to specify custom Job Option values for running the Job instead of using Subsitutions directly. 
    ///  Cannot be updated after creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<BTreeMap<String, String>>,
    /// Template specifies how to create the Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<JobTemplate>,
    /// Specifies the maximum lifetime of a Job that is finished. If not set, it will be set to the DefaultTTLSecondsAfterFinished configuration value in the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecondsAfterFinished")]
    pub ttl_seconds_after_finished: Option<i64>,
    /// Specifies the type of Job. Can be one of: Adhoc, Scheduled 
    ///  Default: Adhoc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Specifies optional start policy for a Job, which specifies certain conditions which have to be met before a Job is started.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStartPolicy {
    /// Specifies the behaviour when there are other concurrent jobs for the JobConfig.
    #[serde(rename = "concurrencyPolicy")]
    pub concurrency_policy: String,
    /// Specifies the earliest time that the Job can be started after. Can be specified together with other fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startAfter")]
    pub start_after: Option<String>,
}

/// Template specifies how to create the Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTemplate {
    /// Defines whether tasks are allowed to be force deleted or not. If the node is unresponsive, it may be possible that the task cannot be killed by normal graceful deletion. The controller may choose to force delete the task, which would ignore the final state of the task since the node is unable to return whether the task is actually still alive. 
    ///  If not set to true, there may be some cases when there may actually be two concurrently running tasks when even when ConcurrencyPolicyForbid. Setting this to true would prevent this from happening, but the Job may remain stuck indefinitely until the node recovers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forbidTaskForceDeletion")]
    pub forbid_task_force_deletion: Option<bool>,
    /// Specifies maximum number of attempts before the Job will terminate in failure. If defined, the controller will wait retryDelaySeconds before creating the next task. Once maxAttempts is reached, the Job terminates in RetryLimitExceeded. 
    ///  If parallelism is also defined, this corresponds to the maximum attempts for each parallel task. That is, if there are 5 parallel task to be run at a time, with maxAttempts of 3, the Job may create up to a maximum of 15 tasks if each has failed. 
    ///  Value must be a positive integer. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAttempts")]
    pub max_attempts: Option<i64>,
    /// Describes how to run multiple tasks in parallel for the Job. If not set, then there will be at most a single task running at any time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<JobTemplateParallelism>,
    /// Optional duration in seconds to wait between retries. If left empty or zero, it means no delay (i.e. retry immediately). 
    ///  If parallelism is also defined, the retry delay is from the time of the last failed task with the same index. That is, if there are two parallel tasks - index 0 and index 1 - which failed at t=0 and t=15, with retryDelaySeconds of 30, the controller will only create the next attempts at t=30 and t=45 respectively. 
    ///  Value must be a non-negative integer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryDelaySeconds")]
    pub retry_delay_seconds: Option<i64>,
    /// Optional duration in seconds to wait before terminating the task if it is still pending. This field is useful to prevent jobs from being stuck forever if the Job has a deadline to start running by. If not set, it will be set to the DefaultPendingTimeoutSeconds configuration value in the controller. To disable pending timeout, set this to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "taskPendingTimeoutSeconds")]
    pub task_pending_timeout_seconds: Option<i64>,
    /// Defines the template to create a single task in the Job.
    #[serde(rename = "taskTemplate")]
    pub task_template: JobTemplateTaskTemplate,
}

/// Describes how to run multiple tasks in parallel for the Job. If not set, then there will be at most a single task running at any time.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTemplateParallelism {
    /// Defines when the Job will complete when there are multiple tasks running in parallel. For example, if using the AllSuccessful strategy, the Job will only terminate once all parallel tasks have terminated successfully, or once any task has exhausted its maxAttempts limit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionStrategy")]
    pub completion_strategy: Option<String>,
    /// Specifies an exact number of tasks to be run in parallel. The index number can be retrieved via the "${task.index_num}" context variable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withCount")]
    pub with_count: Option<i64>,
    /// Specifies a list of keys corresponding to each task that will be run in parallel. The index key can be retrieved via the "${task.index_key}" context variable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withKeys")]
    pub with_keys: Option<Vec<String>>,
    /// Specifies a matrix of key-value pairs, with each key mapped to a list of possible values, such that tasks will be started for each combination of key-value pairs. The matrix values can be retrieved via context variables in the following format: "${task.index_matrix.<key>}".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withMatrix")]
    pub with_matrix: Option<BTreeMap<String, String>>,
}

/// Defines the template to create a single task in the Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTemplateTaskTemplate {
    /// Describes how to create tasks as Pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<JobTemplateTaskTemplatePod>,
}

/// Describes how to create tasks as Pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTemplateTaskTemplatePod {
    /// Standard object's metadata that will be added to Pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    /// Specification of the desired behavior of the pod. API docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.23/#podspec-v1-core 
    ///  Supports context variable substitution in the following fields for containers and initContainers: image, command, args, env.value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<BTreeMap<String, serde_json::Value>>,
}

/// JobStatus defines the observed state of a Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatus {
    /// Condition stores details about the Job's current condition.
    pub condition: JobStatusCondition,
    /// CreatedTasks describes how many tasks were created in total for this Job.
    #[serde(rename = "createdTasks")]
    pub created_tasks: i64,
    /// The current status for parallel execution of the job. May not be set if the job is not a parallel job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parallelStatus")]
    pub parallel_status: Option<JobStatusParallelStatus>,
    /// Phase stores the high-level description of a Job's state.
    pub phase: String,
    /// RunningTasks describes how many tasks are currently running for this Job.
    #[serde(rename = "runningTasks")]
    pub running_tasks: i64,
    /// StartTime specifies the time that the Job was started by the controller. If nil, it means that the Job is Queued. Cannot be changed once set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    /// State stores the high-level state of the Job's current condition. Must be one of: Queued, Waiting, Running, Finished.
    pub state: String,
    /// Tasks contains a list of tasks created by the controller. The controller updates this field when it creates a task, which helps to guard against recreating tasks after they were deleted, and also stores necessary task data for reconciliation in case tasks are deleted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<JobStatusTasks>>,
}

/// Condition stores details about the Job's current condition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusCondition {
    /// Stores the status of the Job's finished state. If specified, it means that all tasks in the Job have terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished: Option<JobStatusConditionFinished>,
    /// Stores the status of the Job's queueing condition. If specified, it means that the Job is currently not started and is queued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queueing: Option<JobStatusConditionQueueing>,
    /// Stores the status of the Job's running state. If specified, it means that all tasks in the Job have started running. If the Job is already complete, this status may be set of not all tasks are terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<JobStatusConditionRunning>,
    /// Stores the status of the Job's waiting condition. If specified, it means that the Job currently is waiting for at least one task to be created and start running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<JobStatusConditionWaiting>,
}

/// Stores the status of the Job's finished state. If specified, it means that all tasks in the Job have terminated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusConditionFinished {
    /// The time at which the Job was first marked as finished by the controller.
    #[serde(rename = "finishTime")]
    pub finish_time: String,
    /// The time at which the latest task was created by the controller. May be nil if no tasks were ever created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestCreationTimestamp")]
    pub latest_creation_timestamp: Option<String>,
    /// The time at which the latest task had started running. May be nil if no tasks had started running.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRunningTimestamp")]
    pub latest_running_timestamp: Option<String>,
    /// Optional descriptive message explaining the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The result of it being finished.
    pub result: String,
}

/// Stores the status of the Job's queueing condition. If specified, it means that the Job is currently not started and is queued.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusConditionQueueing {
    /// Optional descriptive message explaining the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Stores the status of the Job's running state. If specified, it means that all tasks in the Job have started running. If the Job is already complete, this status may be set of not all tasks are terminated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusConditionRunning {
    /// The time at which the latest task had started running.
    #[serde(rename = "latestRunningTimestamp")]
    pub latest_running_timestamp: String,
    /// The timestamp for the latest task that was created by the controller.
    #[serde(rename = "latestTaskCreationTimestamp")]
    pub latest_task_creation_timestamp: String,
    /// Number of tasks waiting to be terminated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terminatingTasks")]
    pub terminating_tasks: Option<i64>,
}

/// Stores the status of the Job's waiting condition. If specified, it means that the Job currently is waiting for at least one task to be created and start running.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusConditionWaiting {
    /// Optional descriptive message explaining the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// The current status for parallel execution of the job. May not be set if the job is not a parallel job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusParallelStatus {
    /// If true, the job is complete and currently in the process of waiting for all remaining tasks to be terminated.
    pub complete: bool,
    /// The status for each parallel index. The size of the list should be exactly equal to the total parallelism factor, even if no tasks are created yet.
    pub indexes: Vec<JobStatusParallelStatusIndexes>,
    /// If complete, contains whether the job is successful according to the ParallelCompletionStrategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
}

/// ParallelIndexStatus stores the status for a single ParallelIndex in the Job. There should be at most one task running at a time for a single parallel index in the Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusParallelStatusIndexes {
    /// Total number of tasks created for this parallel index.
    #[serde(rename = "createdTasks")]
    pub created_tasks: i64,
    /// Hash of the index.
    pub hash: String,
    /// The parallel index.
    pub index: JobStatusParallelStatusIndexesIndex,
    /// Result of executing tasks for this parallel index if it is already terminated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Overall state of the parallel index.
    pub state: String,
}

/// The parallel index.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusParallelStatusIndexesIndex {
    /// If withKeys is used for parallelism, contains the index key of the job as a string.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexKey")]
    pub index_key: Option<String>,
    /// If withCount is used for parallelism, contains the index number of the job numbered from 0 to N-1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexNumber")]
    pub index_number: Option<i64>,
    /// If withMatrix is used for parallelism, contains key-value pairs of the job as strings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matrixValues")]
    pub matrix_values: Option<BTreeMap<String, String>>,
}

/// TaskRef stores information about a Job's owned task.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusTasks {
    /// States of each container for the task. This field will be reconciled from the relevant task object, and is not guaranteed to be up-to-date. This field will persist the state of tasks beyond the lifetime of the task resources, even if they were deleted.
    #[serde(rename = "containerStates")]
    pub container_states: Vec<JobStatusTasksContainerStates>,
    /// Creation time of the task.
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    /// DeletedStatus, if set, specifies a placeholder Status of the task after it is reconciled as deleted. If the task is deleted, Status cannot be reconciled from the task any more, and instead uses information stored in DeletedStatus. In other words, this field acts as a tombstone marker, and is only used after the deletion of the task object is complete. 
    ///  While the task is in the process of being deleted (i.e. deletionTimestamp is set but object still exists), Status will still be reconciled from the actual task's status. 
    ///  If the task is already deleted and DeletedStatus is also not set, then the task's state will be marked as TaskDeletedFinalStateUnknown.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletedStatus")]
    pub deleted_status: Option<JobStatusTasksDeletedStatus>,
    /// Time that the task finished. Will always return a non-zero timestamp if task is finished.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finishTimestamp")]
    pub finish_timestamp: Option<String>,
    /// Name of the task. Assumes to share the same namespace as the Job.
    pub name: String,
    /// Node name that the task was bound to. May be empty if task was never scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// If the Job is a parallel job, then contains the parallel index of the task.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parallelIndex")]
    pub parallel_index: Option<JobStatusTasksParallelIndex>,
    /// The retry index of the task, starting from 0 up to maxAttempts - 1.
    #[serde(rename = "retryIndex")]
    pub retry_index: i64,
    /// Timestamp that the task transitioned to running. May be zero if the task was never observed as started running.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runningTimestamp")]
    pub running_timestamp: Option<String>,
    /// Status of the task. This field will be reconciled from the relevant task object, may not be always up-to-date. This field will persist the state of tasks beyond the lifetime of the task resources, even if they are deleted.
    pub status: JobStatusTasksStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusTasksContainerStates {
    /// Container ID of the container. May be empty if the container is not yet created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerID")]
    pub container_id: Option<String>,
    /// Exit status from the last termination of the container
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    /// Message regarding the container's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the container's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Signal from the last termination of the container
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
}

/// DeletedStatus, if set, specifies a placeholder Status of the task after it is reconciled as deleted. If the task is deleted, Status cannot be reconciled from the task any more, and instead uses information stored in DeletedStatus. In other words, this field acts as a tombstone marker, and is only used after the deletion of the task object is complete. 
///  While the task is in the process of being deleted (i.e. deletionTimestamp is set but object still exists), Status will still be reconciled from the actual task's status. 
///  If the task is already deleted and DeletedStatus is also not set, then the task's state will be marked as TaskDeletedFinalStateUnknown.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusTasksDeletedStatus {
    /// Descriptive message for the task's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the task's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// If the state is Terminated, the result of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// State of the task.
    pub state: String,
}

/// If the Job is a parallel job, then contains the parallel index of the task.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusTasksParallelIndex {
    /// If withKeys is used for parallelism, contains the index key of the job as a string.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexKey")]
    pub index_key: Option<String>,
    /// If withCount is used for parallelism, contains the index number of the job numbered from 0 to N-1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexNumber")]
    pub index_number: Option<i64>,
    /// If withMatrix is used for parallelism, contains key-value pairs of the job as strings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matrixValues")]
    pub matrix_values: Option<BTreeMap<String, String>>,
}

/// Status of the task. This field will be reconciled from the relevant task object, may not be always up-to-date. This field will persist the state of tasks beyond the lifetime of the task resources, even if they are deleted.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusTasksStatus {
    /// Descriptive message for the task's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the task's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// If the state is Terminated, the result of the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// State of the task.
    pub state: String,
}

