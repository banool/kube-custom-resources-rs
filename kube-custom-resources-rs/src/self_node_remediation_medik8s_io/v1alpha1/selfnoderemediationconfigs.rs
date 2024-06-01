// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/medik8s/self-node-remediation/self-node-remediation.medik8s.io/v1alpha1/selfnoderemediationconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// SelfNodeRemediationConfigSpec defines the desired state of SelfNodeRemediationConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "self-node-remediation.medik8s.io", version = "v1alpha1", kind = "SelfNodeRemediationConfig", plural = "selfnoderemediationconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SelfNodeRemediationConfigSpec {
    /// the frequency for api-server connectivity check
    /// Valid time units are "ms", "s", "m", "h".
    /// the frequency for api-server connectivity check
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiCheckInterval")]
    pub api_check_interval: Option<String>,
    /// Valid time units are "ms", "s", "m", "h".
    /// timeout for each api-connectivity check
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiServerTimeout")]
    pub api_server_timeout: Option<String>,
    /// CustomDsTolerations allows to add custom tolerations snr agents that are running on the ds in order to support remediation for different types of nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customDsTolerations")]
    pub custom_ds_tolerations: Option<Vec<SelfNodeRemediationConfigCustomDsTolerations>>,
    /// EndpointHealthCheckUrl is an url that self node remediation agents which run on control-plane node will try to access when they can't contact their peers.
    /// This is a part of self diagnostics which will decide whether the node should be remediated or not.
    /// It will be ignored when empty (which is the default).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointHealthCheckUrl")]
    pub endpoint_health_check_url: Option<String>,
    /// HostPort is used for internal communication between SNR agents.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostPort")]
    pub host_port: Option<i64>,
    /// IsSoftwareRebootEnabled indicates whether self node remediation agent will do software reboot,
    /// if the watchdog device can not be used or will use watchdog only,
    /// without a fallback to software reboot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isSoftwareRebootEnabled")]
    pub is_software_reboot_enabled: Option<bool>,
    /// after this threshold, the node will start contacting its peers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxApiErrorThreshold")]
    pub max_api_error_threshold: Option<i64>,
    /// Valid time units are "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerApiServerTimeout")]
    pub peer_api_server_timeout: Option<String>,
    /// Valid time units are "ms", "s", "m", "h".
    /// timeout for establishing connection to peer
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerDialTimeout")]
    pub peer_dial_timeout: Option<String>,
    /// Valid time units are "ms", "s", "m", "h".
    /// timeout for each peer request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerRequestTimeout")]
    pub peer_request_timeout: Option<String>,
    /// Valid time units are "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerUpdateInterval")]
    pub peer_update_interval: Option<String>,
    /// SafeTimeToAssumeNodeRebootedSeconds is the time after which the healthy self node remediation
    /// agents will assume the unhealthy node has been rebooted, and it is safe to recover affected workloads.
    /// This is extremely important as starting replacement Pods while they are still running on the failed
    /// node will likely lead to data corruption and violation of run-once semantics.
    /// In an effort to prevent this, the operator ignores values lower than a minimum calculated from the
    /// ApiCheckInterval, ApiServerTimeout, MaxApiErrorThreshold, PeerDialTimeout, and PeerRequestTimeout fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "safeTimeToAssumeNodeRebootedSeconds")]
    pub safe_time_to_assume_node_rebooted_seconds: Option<i64>,
    /// WatchdogFilePath is the watchdog file path that should be available on each node, e.g. /dev/watchdog
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "watchdogFilePath")]
    pub watchdog_file_path: Option<String>,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SelfNodeRemediationConfigCustomDsTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// SelfNodeRemediationConfigStatus defines the observed state of SelfNodeRemediationConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SelfNodeRemediationConfigStatus {
}

