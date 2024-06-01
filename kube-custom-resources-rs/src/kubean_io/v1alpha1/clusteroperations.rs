// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubean-io/kubean/kubean.io/v1alpha1/clusteroperations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Spec defines the desired state of a member cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kubean.io", version = "v1alpha1", kind = "ClusterOperation", plural = "clusteroperations")]
#[kube(status = "ClusterOperationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterOperationSpec {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSource")]
    pub action_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSourceRef")]
    pub action_source_ref: Option<ClusterOperationActionSourceRef>,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDeadlineSeconds")]
    pub active_deadline_seconds: Option<i64>,
    /// Cluster the name of Cluster.kubean.io.
    pub cluster: String,
    /// EntrypointSHRef will be filled by operator when it renders entrypoint.sh.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entrypointSHRef")]
    pub entrypoint_sh_ref: Option<ClusterOperationEntrypointShRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraArgs")]
    pub extra_args: Option<String>,
    /// HostsConfRef will be filled by operator when it performs backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostsConfRef")]
    pub hosts_conf_ref: Option<ClusterOperationHostsConfRef>,
    pub image: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postHook")]
    pub post_hook: Option<Vec<ClusterOperationPostHook>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preHook")]
    pub pre_hook: Option<Vec<ClusterOperationPreHook>>,
    /// ResourceRequirements describes the compute resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ClusterOperationResources>,
    /// SSHAuthRef will be filled by operator when it performs backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshAuthRef")]
    pub ssh_auth_ref: Option<ClusterOperationSshAuthRef>,
    /// VarsConfRef will be filled by operator when it performs backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "varsConfRef")]
    pub vars_conf_ref: Option<ClusterOperationVarsConfRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationActionSourceRef {
    pub name: String,
    pub namespace: String,
}

/// EntrypointSHRef will be filled by operator when it renders entrypoint.sh.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationEntrypointShRef {
    pub name: String,
    pub namespace: String,
}

/// HostsConfRef will be filled by operator when it performs backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationHostsConfRef {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationPostHook {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSource")]
    pub action_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSourceRef")]
    pub action_source_ref: Option<ClusterOperationPostHookActionSourceRef>,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraArgs")]
    pub extra_args: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationPostHookActionSourceRef {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationPreHook {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSource")]
    pub action_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionSourceRef")]
    pub action_source_ref: Option<ClusterOperationPreHookActionSourceRef>,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraArgs")]
    pub extra_args: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationPreHookActionSourceRef {
    pub name: String,
    pub namespace: String,
}

/// ResourceRequirements describes the compute resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ClusterOperationResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// SSHAuthRef will be filled by operator when it performs backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationSshAuthRef {
    pub name: String,
    pub namespace: String,
}

/// VarsConfRef will be filled by operator when it performs backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationVarsConfRef {
    pub name: String,
    pub namespace: String,
}

/// Status contains information about the current status of a cluster operation job updated periodically by cluster controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Digest is used to avoid the change of clusterOps by others. it will be filled by operator. Do Not change this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<String>,
    /// HasModified indicates the spec has been modified by others after created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hasModified")]
    pub has_modified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobRef")]
    pub job_ref: Option<ClusterOperationStatusJobRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperationStatusJobRef {
    pub name: String,
    pub namespace: String,
}

