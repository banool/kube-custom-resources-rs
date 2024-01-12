// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta2/gitrepositories.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// GitRepositorySpec specifies the required configuration to produce an Artifact for a Git repository.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta2", kind = "GitRepository", plural = "gitrepositories")]
#[kube(namespaced)]
#[kube(status = "GitRepositoryStatus")]
#[kube(schema = "disabled")]
pub struct GitRepositorySpec {
    /// AccessFrom specifies an Access Control List for allowing cross-namespace references to this object. NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<GitRepositoryAccessFrom>,
    /// GitImplementation specifies which Git client library implementation to use. Defaults to 'go-git', valid values are ('go-git', 'libgit2'). Deprecated: gitImplementation is deprecated now that 'go-git' is the only supported implementation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitImplementation")]
    pub git_implementation: Option<GitRepositoryGitImplementation>,
    /// Ignore overrides the set of excluded patterns in the .sourceignore format (which is the same as .gitignore). If not provided, a default will be used, consult the documentation for your version to find out what those are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<String>,
    /// Include specifies a list of GitRepository resources which Artifacts should be included in the Artifact produced for this GitRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<GitRepositoryInclude>>,
    /// Interval at which to check the GitRepository for updates.
    pub interval: String,
    /// RecurseSubmodules enables the initialization of all submodules within the GitRepository as cloned from the URL, using their default settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recurseSubmodules")]
    pub recurse_submodules: Option<bool>,
    /// Reference specifies the Git reference to resolve and monitor for changes, defaults to the 'master' branch.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ref")]
    pub r#ref: Option<GitRepositoryRef>,
    /// SecretRef specifies the Secret containing authentication credentials for the GitRepository. For HTTPS repositories the Secret must contain 'username' and 'password' fields for basic auth or 'bearerToken' field for token auth. For SSH repositories the Secret must contain 'identity' and 'known_hosts' fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<GitRepositorySecretRef>,
    /// Suspend tells the controller to suspend the reconciliation of this GitRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for Git operations like cloning, defaults to 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// URL specifies the Git repository URL, it can be an HTTP/S or SSH address.
    pub url: String,
    /// Verification specifies the configuration to verify the Git commit signature(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify: Option<GitRepositoryVerify>,
}

/// AccessFrom specifies an Access Control List for allowing cross-namespace references to this object. NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies. Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<GitRepositoryAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies. An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// GitRepositorySpec specifies the required configuration to produce an Artifact for a Git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GitRepositoryGitImplementation {
    #[serde(rename = "go-git")]
    GoGit,
    #[serde(rename = "libgit2")]
    Libgit2,
}

/// GitRepositoryInclude specifies a local reference to a GitRepository which Artifact (sub-)contents must be included, and where they should be placed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryInclude {
    /// FromPath specifies the path to copy contents from, defaults to the root of the Artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromPath")]
    pub from_path: Option<String>,
    /// GitRepositoryRef specifies the GitRepository which Artifact contents must be included.
    pub repository: GitRepositoryIncludeRepository,
    /// ToPath specifies the path to copy contents to, defaults to the name of the GitRepositoryRef.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toPath")]
    pub to_path: Option<String>,
}

/// GitRepositoryRef specifies the GitRepository which Artifact contents must be included.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryIncludeRepository {
    /// Name of the referent.
    pub name: String,
}

/// Reference specifies the Git reference to resolve and monitor for changes, defaults to the 'master' branch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryRef {
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

/// SecretRef specifies the Secret containing authentication credentials for the GitRepository. For HTTPS repositories the Secret must contain 'username' and 'password' fields for basic auth or 'bearerToken' field for token auth. For SSH repositories the Secret must contain 'identity' and 'known_hosts' fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// Verification specifies the configuration to verify the Git commit signature(s).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryVerify {
    /// Mode specifies what Git object should be verified, currently ('head').
    pub mode: GitRepositoryVerifyMode,
    /// SecretRef specifies the Secret containing the public keys of trusted Git authors.
    #[serde(rename = "secretRef")]
    pub secret_ref: GitRepositoryVerifySecretRef,
}

/// Verification specifies the configuration to verify the Git commit signature(s).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GitRepositoryVerifyMode {
    #[serde(rename = "head")]
    Head,
}

/// SecretRef specifies the Secret containing the public keys of trusted Git authors.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryVerifySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// GitRepositoryStatus records the observed state of a Git repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatus {
    /// Artifact represents the last successful GitRepository reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<GitRepositoryStatusArtifact>,
    /// Conditions holds the conditions for the GitRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GitRepositoryStatusConditions>>,
    /// ContentConfigChecksum is a checksum of all the configurations related to the content of the source artifact: - .spec.ignore - .spec.recurseSubmodules - .spec.included and the checksum of the included artifacts observed in .status.observedGeneration version of the object. This can be used to determine if the content of the included repository has changed. It has the format of `<algo>:<checksum>`, for example: `sha256:<checksum>`. 
    ///  Deprecated: Replaced with explicit fields for observed artifact content config in the status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentConfigChecksum")]
    pub content_config_checksum: Option<String>,
    /// IncludedArtifacts contains a list of the last successfully included Artifacts as instructed by GitRepositorySpec.Include.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includedArtifacts")]
    pub included_artifacts: Option<Vec<GitRepositoryStatusIncludedArtifacts>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation of the GitRepository object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ObservedIgnore is the observed exclusion patterns used for constructing the source artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedIgnore")]
    pub observed_ignore: Option<String>,
    /// ObservedInclude is the observed list of GitRepository resources used to to produce the current Artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedInclude")]
    pub observed_include: Option<Vec<GitRepositoryStatusObservedInclude>>,
    /// ObservedRecurseSubmodules is the observed resource submodules configuration used to produce the current Artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedRecurseSubmodules")]
    pub observed_recurse_submodules: Option<bool>,
    /// URL is the dynamic fetch link for the latest Artifact. It is provided on a "best effort" basis, and using the precise GitRepositoryStatus.Artifact data is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the last successful GitRepository reconciliation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatusArtifact {
    /// Digest is the digest of the file in the form of '<algorithm>:<checksum>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of the Artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Metadata holds upstream information such as OCI annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Path is the relative file path of the Artifact. It can be used to locate the file in the root of the Artifact storage on the local file system of the controller managing the Source.
    pub path: String,
    /// Revision is a human-readable identifier traceable in the origin source system. It can be a Git commit SHA, Git tag, a Helm chart version, etc.
    pub revision: String,
    /// Size is the number of bytes in the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// URL is the HTTP address of the Artifact as exposed by the controller managing the Source. It can be used to retrieve the Artifact for consumption, e.g. by another controller applying the Artifact contents.
    pub url: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatusConditions {
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
    pub status: GitRepositoryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GitRepositoryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// Artifact represents the output of a Source reconciliation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatusIncludedArtifacts {
    /// Digest is the digest of the file in the form of '<algorithm>:<checksum>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of the Artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Metadata holds upstream information such as OCI annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Path is the relative file path of the Artifact. It can be used to locate the file in the root of the Artifact storage on the local file system of the controller managing the Source.
    pub path: String,
    /// Revision is a human-readable identifier traceable in the origin source system. It can be a Git commit SHA, Git tag, a Helm chart version, etc.
    pub revision: String,
    /// Size is the number of bytes in the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// URL is the HTTP address of the Artifact as exposed by the controller managing the Source. It can be used to retrieve the Artifact for consumption, e.g. by another controller applying the Artifact contents.
    pub url: String,
}

/// GitRepositoryInclude specifies a local reference to a GitRepository which Artifact (sub-)contents must be included, and where they should be placed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatusObservedInclude {
    /// FromPath specifies the path to copy contents from, defaults to the root of the Artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromPath")]
    pub from_path: Option<String>,
    /// GitRepositoryRef specifies the GitRepository which Artifact contents must be included.
    pub repository: GitRepositoryStatusObservedIncludeRepository,
    /// ToPath specifies the path to copy contents to, defaults to the name of the GitRepositoryRef.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toPath")]
    pub to_path: Option<String>,
}

/// GitRepositoryRef specifies the GitRepository which Artifact contents must be included.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitRepositoryStatusObservedIncludeRepository {
    /// Name of the referent.
    pub name: String,
}

