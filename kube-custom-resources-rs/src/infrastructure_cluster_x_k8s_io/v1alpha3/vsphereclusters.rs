// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1alpha3/vsphereclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VSphereClusterSpec defines the desired state of VSphereCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha3", kind = "VSphereCluster", plural = "vsphereclusters")]
#[kube(namespaced)]
#[kube(status = "VSphereClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereClusterSpec {
    /// CloudProviderConfiguration holds the cluster-wide configuration for the vSphere cloud provider. 
    ///  Deprecated: will be removed in v1alpha4.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudProviderConfiguration")]
    pub cloud_provider_configuration: Option<VSphereClusterCloudProviderConfiguration>,
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<VSphereClusterControlPlaneEndpoint>,
    /// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains the identity to use when reconciling the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityRef")]
    pub identity_ref: Option<VSphereClusterIdentityRef>,
    /// Insecure is a flag that controls whether to validate the vSphere server's certificate. 
    ///  Deprecated: will be removed in v1alpha4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// LoadBalancerRef may be used to enable a control plane load balancer for this cluster. When a LoadBalancerRef is provided, the VSphereCluster.Status.Ready field will not be true until the referenced resource is Status.Ready and has a non-empty Status.Address value. 
    ///  Deprecated: will be removed in v1alpha4.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerRef")]
    pub load_balancer_ref: Option<VSphereClusterLoadBalancerRef>,
    /// Server is the address of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Thumbprint is the colon-separated SHA-1 checksum of the given vCenter server's host certificate When provided, Insecure should not be set to true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

/// CloudProviderConfiguration holds the cluster-wide configuration for the vSphere cloud provider. 
///  Deprecated: will be removed in v1alpha4.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfiguration {
    /// Disk is the vSphere cloud provider's disk configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk: Option<VSphereClusterCloudProviderConfigurationDisk>,
    /// Global is the vSphere cloud provider's global configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<VSphereClusterCloudProviderConfigurationGlobal>,
    /// Labels is the vSphere cloud provider's zone and region configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<VSphereClusterCloudProviderConfigurationLabels>,
    /// Network is the vSphere cloud provider's network configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<VSphereClusterCloudProviderConfigurationNetwork>,
    /// CPIProviderConfig contains extra information used to configure the vSphere cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerConfig")]
    pub provider_config: Option<VSphereClusterCloudProviderConfigurationProviderConfig>,
    /// VCenter is a list of vCenter configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualCenter")]
    pub virtual_center: Option<BTreeMap<String, VSphereClusterCloudProviderConfigurationVirtualCenter>>,
    /// Workspace is the vSphere cloud provider's workspace configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<VSphereClusterCloudProviderConfigurationWorkspace>,
}

/// Disk is the vSphere cloud provider's disk configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationDisk {
    /// SCSIControllerType defines SCSI controller to be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scsiControllerType")]
    pub scsi_controller_type: Option<String>,
}

/// Global is the vSphere cloud provider's global configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationGlobal {
    /// APIBindPort configures the vSphere cloud controller manager API port. Defaults to 43001.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiBindPort")]
    pub api_bind_port: Option<String>,
    /// APIDisable disables the vSphere cloud controller manager API. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiDisable")]
    pub api_disable: Option<bool>,
    /// CAFile Specifies the path to a CA certificate in PEM format. If not configured, the system's CA certificates will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caFile")]
    pub ca_file: Option<String>,
    /// Datacenters is a CSV string of the datacenters in which VMs are located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<String>,
    /// Insecure is a flag that disables TLS peer verification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Password is the password used to access a vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Port is the port on which the vSphere endpoint is listening. Defaults to 443.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// RoundTripperCount specifies the SOAP round tripper count (retries = RoundTripper - 1)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roundTripperCount")]
    pub round_tripper_count: Option<i32>,
    /// SecretName is the name of the Kubernetes secret in which the vSphere credentials are located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// SecretNamespace is the namespace for SecretName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretNamespace")]
    pub secret_namespace: Option<String>,
    /// SecretsDirectory is a directory in which secrets may be found. This may used in the event that: 1. It is not desirable to use the K8s API to watch changes to secrets 2. The cloud controller manager is not running in a K8s environment, such as DC/OS. For example, the container storage interface (CSI) is container orcehstrator (CO) agnostic, and should support non-K8s COs. Defaults to /etc/cloud/credentials.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretsDirectory")]
    pub secrets_directory: Option<String>,
    /// ServiceAccount is the Kubernetes service account used to launch the cloud controller manager. Defaults to cloud-controller-manager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// Thumbprint is the cryptographic thumbprint of the vSphere endpoint's certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    /// Username is the username used to access a vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Labels is the vSphere cloud provider's zone and region configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationLabels {
    /// Region is the region in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Zone is the zone in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// Network is the vSphere cloud provider's network configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationNetwork {
    /// Name is the name of the network to which VMs are connected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CPIProviderConfig contains extra information used to configure the vSphere cloud provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationProviderConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<VSphereClusterCloudProviderConfigurationProviderConfigCloud>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<VSphereClusterCloudProviderConfigurationProviderConfigStorage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationProviderConfigCloud {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerImage")]
    pub controller_image: Option<String>,
    /// ExtraArgs passes through extra arguments to the cloud provider. The arguments here are passed to the cloud provider daemonset specification
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraArgs")]
    pub extra_args: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationProviderConfigStorage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attacherImage")]
    pub attacher_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerImage")]
    pub controller_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "livenessProbeImage")]
    pub liveness_probe_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSyncerImage")]
    pub metadata_syncer_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDriverImage")]
    pub node_driver_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionerImage")]
    pub provisioner_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrarImage")]
    pub registrar_image: Option<String>,
}

/// VCenter is a list of vCenter configurations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationVirtualCenter {
    /// Datacenters is a CSV string of the datacenters in which VMs are located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<String>,
    /// Password is the password used to access a vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Port is the port on which the vSphere endpoint is listening. Defaults to 443.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// RoundTripperCount specifies the SOAP round tripper count (retries = RoundTripper - 1)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roundTripperCount")]
    pub round_tripper_count: Option<i32>,
    /// Thumbprint is the cryptographic thumbprint of the vSphere endpoint's certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    /// Username is the username used to access a vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Workspace is the vSphere cloud provider's workspace configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterCloudProviderConfigurationWorkspace {
    /// Datacenter is the datacenter in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// Datastore is the datastore in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,
    /// Folder is the folder in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// ResourcePool is the resource pool in which VMs are created/located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
    /// Server is the IP address or FQDN of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VSphereClusterIdentityRef {
    /// Kind of the identity. Can either be VSphereClusterIdentity or Secret
    pub kind: VSphereClusterIdentityRefKind,
    /// Name of the identity.
    pub name: String,
}

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereClusterIdentityRefKind {
    VSphereClusterIdentity,
    Secret,
}

/// LoadBalancerRef may be used to enable a control plane load balancer for this cluster. When a LoadBalancerRef is provided, the VSphereCluster.Status.Ready field will not be true until the referenced resource is Status.Ready and has a non-empty Status.Address value. 
///  Deprecated: will be removed in v1alpha4.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterLoadBalancerRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// VSphereClusterStatus defines the observed state of VSphereClusterSpec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterStatus {
    /// Conditions defines current service state of the VSphereCluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureDomains is a list of failure domain objects synced from the infrastructure provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomains")]
    pub failure_domains: Option<BTreeMap<String, VSphereClusterStatusFailureDomains>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

/// FailureDomains is a list of failure domain objects synced from the infrastructure provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterStatusFailureDomains {
    /// Attributes is a free form map of attributes an infrastructure provider might use or require.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// ControlPlane determines if this failure domain is suitable for use by control plane machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<bool>,
}

