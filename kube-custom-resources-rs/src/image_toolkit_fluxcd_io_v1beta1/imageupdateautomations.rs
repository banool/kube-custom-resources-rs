// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/fluxcd/image-automation-controller/image.toolkit.fluxcd.io/v1beta1/imageupdateautomations.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ImageUpdateAutomationSpec defines the desired state of ImageUpdateAutomation
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "image.toolkit.fluxcd.io", version = "v1beta1", kind = "ImageUpdateAutomation", plural = "imageupdateautomations")]
#[kube(namespaced)]
#[kube(status = "ImageUpdateAutomationStatus")]
#[kube(schema = "disabled")]
pub struct ImageUpdateAutomationSpec {
    /// GitSpec contains all the git-specific definitions. This is technically optional, but in practice mandatory until there are other kinds of source allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<ImageUpdateAutomationGit>,
    /// Interval gives an lower bound for how often the automation run should be attempted.
    pub interval: String,
    /// SourceRef refers to the resource giving access details to a git repository.
    #[serde(rename = "sourceRef")]
    pub source_ref: ImageUpdateAutomationSourceRef,
    /// Suspend tells the controller to not run this automation, until it is unset (or set to false). Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Update gives the specification for how to update the files in the repository. This can be left empty, to use the default value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<ImageUpdateAutomationUpdate>,
}

/// GitSpec contains all the git-specific definitions. This is technically optional, but in practice mandatory until there are other kinds of source allowed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGit {
    /// Checkout gives the parameters for cloning the git repository, ready to make changes. If not present, the `spec.ref` field from the referenced `GitRepository` or its default will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout: Option<ImageUpdateAutomationGitCheckout>,
    /// Commit specifies how to commit to the git repository.
    pub commit: ImageUpdateAutomationGitCommit,
    /// Push specifies how and where to push commits made by the automation. If missing, commits are pushed (back) to `.spec.checkout.branch` or its default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<ImageUpdateAutomationGitPush>,
}

/// Checkout gives the parameters for cloning the git repository, ready to make changes. If not present, the `spec.ref` field from the referenced `GitRepository` or its default will be used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCheckout {
    /// Reference gives a branch, tag or commit to clone from the Git repository.
    #[serde(rename = "ref")]
    pub r#ref: ImageUpdateAutomationGitCheckoutRef,
}

/// Reference gives a branch, tag or commit to clone from the Git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCheckoutRef {
    /// Branch to check out, defaults to 'master' if no other field is defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Commit SHA to check out, takes precedence over all reference fields. 
    ///  This can be combined with Branch to shallow clone the branch, in which the commit is expected to exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// Name of the reference to check out; takes precedence over Branch, Tag and SemVer. 
    ///  It must be a valid Git reference: https://git-scm.com/docs/git-check-ref-format#_description Examples: "refs/heads/main", "refs/tags/v0.1.0", "refs/pull/420/head", "refs/merge-requests/1/head"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// SemVer tag expression to check out, takes precedence over Tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
    /// Tag to check out, takes precedence over Branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Commit specifies how to commit to the git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCommit {
    /// Author gives the email and optionally the name to use as the author of commits.
    pub author: ImageUpdateAutomationGitCommitAuthor,
    /// MessageTemplate provides a template for the commit message, into which will be interpolated the details of the change made.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageTemplate")]
    pub message_template: Option<String>,
    /// SigningKey provides the option to sign commits with a GPG key
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingKey")]
    pub signing_key: Option<ImageUpdateAutomationGitCommitSigningKey>,
}

/// Author gives the email and optionally the name to use as the author of commits.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCommitAuthor {
    /// Email gives the email to provide when making a commit.
    pub email: String,
    /// Name gives the name to provide when making a commit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SigningKey provides the option to sign commits with a GPG key
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCommitSigningKey {
    /// SecretRef holds the name to a secret that contains a 'git.asc' key corresponding to the ASCII Armored file containing the GPG signing keypair as the value. It must be in the same namespace as the ImageUpdateAutomation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ImageUpdateAutomationGitCommitSigningKeySecretRef>,
}

/// SecretRef holds the name to a secret that contains a 'git.asc' key corresponding to the ASCII Armored file containing the GPG signing keypair as the value. It must be in the same namespace as the ImageUpdateAutomation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitCommitSigningKeySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// Push specifies how and where to push commits made by the automation. If missing, commits are pushed (back) to `.spec.checkout.branch` or its default.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationGitPush {
    /// Branch specifies that commits should be pushed to the branch named. The branch is created using `.spec.checkout.branch` as the starting point, if it doesn't already exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Options specifies the push options that are sent to the Git server when performing a push operation. For details, see: https://git-scm.com/docs/git-push#Documentation/git-push.txt---push-optionltoptiongt
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// Refspec specifies the Git Refspec to use for a push operation. If both Branch and Refspec are provided, then the commit is pushed to the branch and also using the specified refspec. For more details about Git Refspecs, see: https://git-scm.com/book/en/v2/Git-Internals-The-Refspec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refspec: Option<String>,
}

/// SourceRef refers to the resource giving access details to a git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationSourceRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent.
    pub kind: ImageUpdateAutomationSourceRefKind,
    /// Name of the referent.
    pub name: String,
    /// Namespace of the referent, defaults to the namespace of the Kubernetes resource object that contains the reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SourceRef refers to the resource giving access details to a git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImageUpdateAutomationSourceRefKind {
    GitRepository,
}

/// Update gives the specification for how to update the files in the repository. This can be left empty, to use the default value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationUpdate {
    /// Path to the directory containing the manifests to be updated. Defaults to 'None', which translates to the root path of the GitRepositoryRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Strategy names the strategy to be used.
    pub strategy: ImageUpdateAutomationUpdateStrategy,
}

/// Update gives the specification for how to update the files in the repository. This can be left empty, to use the default value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImageUpdateAutomationUpdateStrategy {
    Setters,
}

/// ImageUpdateAutomationStatus defines the observed state of ImageUpdateAutomation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ImageUpdateAutomationStatusConditions>>,
    /// LastAutomationRunTime records the last time the controller ran this automation through to completion (even if no updates were made).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAutomationRunTime")]
    pub last_automation_run_time: Option<String>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// LastPushCommit records the SHA1 of the last commit made by the controller, for this automation object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPushCommit")]
    pub last_push_commit: Option<String>,
    /// LastPushTime records the time of the last pushed change.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPushTime")]
    pub last_push_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImageUpdateAutomationStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: ImageUpdateAutomationStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImageUpdateAutomationStatusConditionsStatus {
    True,
    False,
    Unknown,
}

