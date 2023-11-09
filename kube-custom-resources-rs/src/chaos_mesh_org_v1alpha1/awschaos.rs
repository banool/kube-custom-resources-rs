// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/awschaos.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AWSChaosSpec is the content of the specification for an AWSChaos
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "AWSChaos", plural = "awschaos")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct AWSChaosSpec {
    /// Action defines the specific aws chaos action. Supported action: ec2-stop / ec2-restart / detach-volume Default action: ec2-stop
    pub action: AWSChaosAction,
    /// AWSRegion defines the region of aws.
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    /// DeviceName indicates the name of the device. Needed in detach-volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// Duration represents the duration of the chaos action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Ec2Instance indicates the ID of the ec2 instance.
    #[serde(rename = "ec2Instance")]
    pub ec2_instance: String,
    /// Endpoint indicates the endpoint of the aws server. Just used it in test now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// SecretName defines the name of kubernetes secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// EbsVolume indicates the ID of the EBS volume. Needed in detach-volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeID")]
    pub volume_id: Option<String>,
}

/// AWSChaosSpec is the content of the specification for an AWSChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AWSChaosAction {
    #[serde(rename = "ec2-stop")]
    Ec2Stop,
    #[serde(rename = "ec2-restart")]
    Ec2Restart,
    #[serde(rename = "detach-volume")]
    DetachVolume,
}

/// AWSChaosStatus represents the status of an AWSChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AWSChaosStatus {
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AWSChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: AWSChaosStatusExperiment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AWSChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AWSChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<AWSChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<AWSChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AWSChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<AWSChaosStatusExperimentContainerRecordsEvents>>,
    pub id: String,
    /// InjectedCount is a counter to record the sum of successful injections
    #[serde(rename = "injectedCount")]
    pub injected_count: i64,
    pub phase: String,
    /// RecoveredCount is a counter to record the sum of successful recoveries
    #[serde(rename = "recoveredCount")]
    pub recovered_count: i64,
    #[serde(rename = "selectorKey")]
    pub selector_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AWSChaosStatusExperimentContainerRecordsEvents {
    /// Message is the detail message, e.g. the reason why we failed to inject the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Operation represents the operation we are doing, when we crate this event
    pub operation: String,
    /// Timestamp is time when we create this event
    pub timestamp: String,
    /// Type means the stage of this event
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AWSChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

