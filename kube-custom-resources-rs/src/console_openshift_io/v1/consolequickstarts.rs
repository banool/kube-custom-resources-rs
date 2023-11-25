// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/console.openshift.io/v1/consolequickstarts.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ConsoleQuickStartSpec is the desired quick start configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "console.openshift.io", version = "v1", kind = "ConsoleQuickStart", plural = "consolequickstarts")]
#[kube(schema = "disabled")]
pub struct ConsoleQuickStartSpec {
    /// accessReviewResources contains a list of resources that the user's access will be reviewed against in order for the user to complete the Quick Start. The Quick Start will be hidden if any of the access reviews fail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessReviewResources")]
    pub access_review_resources: Option<Vec<ConsoleQuickStartAccessReviewResources>>,
    /// conclusion sums up the Quick Start and suggests the possible next steps. (includes markdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    /// description is the description of the Quick Start. (includes markdown)
    pub description: String,
    /// displayName is the display name of the Quick Start.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// durationMinutes describes approximately how many minutes it will take to complete the Quick Start.
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i64,
    /// icon is a base64 encoded image that will be displayed beside the Quick Start display name. The icon should be an vector image for easy scaling. The size of the icon should be 40x40.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// introduction describes the purpose of the Quick Start. (includes markdown)
    pub introduction: String,
    /// nextQuickStart is a list of the following Quick Starts, suggested for the user to try.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextQuickStart")]
    pub next_quick_start: Option<Vec<String>>,
    /// prerequisites contains all prerequisites that need to be met before taking a Quick Start. (includes markdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerequisites: Option<Vec<String>>,
    /// tags is a list of strings that describe the Quick Start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// tasks is the list of steps the user has to perform to complete the Quick Start.
    pub tasks: Vec<ConsoleQuickStartTasks>,
}

/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleQuickStartAccessReviewResources {
    /// Group is the API Group of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Resource is one of the existing resource types.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Subresource is one of the existing resource types.  "" means none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    /// Version is the API Version of the Resource.  "*" means all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// ConsoleQuickStartTask is a single step in a Quick Start.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleQuickStartTasks {
    /// description describes the steps needed to complete the task. (includes markdown)
    pub description: String,
    /// review contains instructions to validate the task is complete. The user will select 'Yes' or 'No'. using a radio button, which indicates whether the step was completed successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<ConsoleQuickStartTasksReview>,
    /// summary contains information about the passed step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ConsoleQuickStartTasksSummary>,
    /// title describes the task and is displayed as a step heading.
    pub title: String,
}

/// review contains instructions to validate the task is complete. The user will select 'Yes' or 'No'. using a radio button, which indicates whether the step was completed successfully.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleQuickStartTasksReview {
    /// failedTaskHelp contains suggestions for a failed task review and is shown at the end of task. (includes markdown)
    #[serde(rename = "failedTaskHelp")]
    pub failed_task_help: String,
    /// instructions contains steps that user needs to take in order to validate his work after going through a task. (includes markdown)
    pub instructions: String,
}

/// summary contains information about the passed step.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleQuickStartTasksSummary {
    /// failed briefly describes the unsuccessfully passed task. (includes markdown)
    pub failed: String,
    /// success describes the succesfully passed task.
    pub success: String,
}

