// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tungstenfabric/tf-operator/tf.tungsten.io/v1alpha1/vrouters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VrouterSpec is the Spec for the vrouter API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tf.tungsten.io", version = "v1alpha1", kind = "Vrouter", plural = "vrouters")]
#[kube(namespaced)]
#[kube(status = "VrouterStatus")]
#[kube(schema = "disabled")]
pub struct VrouterSpec {
    /// PodConfiguration is the common services struct.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonConfiguration")]
    pub common_configuration: Option<VrouterCommonConfiguration>,
    /// VrouterConfiguration is the Spec for the vrouter API.
    #[serde(rename = "serviceConfiguration")]
    pub service_configuration: VrouterServiceConfiguration,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterCommonConfiguration {
    /// AuthParameters auth parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authParameters")]
    pub auth_parameters: Option<VrouterCommonConfigurationAuthParameters>,
    /// OS family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<String>>,
    /// Kubernetes Cluster Configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<VrouterCommonConfigurationLogLevel>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<VrouterCommonConfigurationTolerations>>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterCommonConfigurationAuthParameters {
    /// AuthenticationMode auth mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authMode")]
    pub auth_mode: Option<VrouterCommonConfigurationAuthParametersAuthMode>,
    /// KeystoneAuthParameters keystone parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthParameters")]
    pub keystone_auth_parameters: Option<VrouterCommonConfigurationAuthParametersKeystoneAuthParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneSecretName")]
    pub keystone_secret_name: Option<String>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VrouterCommonConfigurationAuthParametersAuthMode {
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "keystone")]
    Keystone,
}

/// KeystoneAuthParameters keystone parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterCommonConfigurationAuthParametersKeystoneAuthParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPassword")]
    pub admin_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPort")]
    pub admin_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminTenant")]
    pub admin_tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminUsername")]
    pub admin_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authProtocol")]
    pub auth_protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectDomainName")]
    pub project_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userDomainName")]
    pub user_domain_name: Option<String>,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VrouterCommonConfigurationLogLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "none")]
    None,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterCommonConfigurationTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// VrouterConfiguration is the Spec for the vrouter API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterServiceConfiguration {
    /// vRouter
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "agentMode")]
    pub agent_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "barbicanPassword")]
    pub barbican_password: Option<String>,
    /// Openstack
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "barbicanTenantName")]
    pub barbican_tenant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "barbicanUser")]
    pub barbican_user: Option<String>,
    /// New params for vrouter configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudOrchestrator")]
    pub cloud_orchestrator: Option<String>,
    /// CniMTU - mtu for virtual tap devices
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cniMTU")]
    pub cni_mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectorPort")]
    pub collector_port: Option<String>,
    /// Config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configApiPort")]
    pub config_api_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configApiServerCaCertfile")]
    pub config_api_server_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configApiSslEnable")]
    pub config_api_ssl_enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<VrouterServiceConfigurationContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlInstance")]
    pub control_instance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSubnet")]
    pub data_subnet: Option<String>,
    /// DNS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsServerPort")]
    pub dns_server_port: Option<String>,
    /// Host
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dpdkUioDriver")]
    pub dpdk_uio_driver: Option<String>,
    /// What is it doing? VrouterEncryption   bool              `json:"vrouterEncryption,omitempty"` What is it doing? What is it doing?
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envVariablesConfig")]
    pub env_variables_config: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fabricSntHashTableSize")]
    pub fabric_snt_hash_table_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hugePages1G")]
    pub huge_pages1_g: Option<i64>,
    /// HugePages
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hugePages2M")]
    pub huge_pages2_m: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hypervisorType")]
    pub hypervisor_type: Option<String>,
    /// Introspect
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "introspectSslEnable")]
    pub introspect_ssl_enable: Option<bool>,
    /// Kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "k8sToken")]
    pub k8s_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "k8sTokenFile")]
    pub k8s_token_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthAdminPassword")]
    pub keystone_auth_admin_password: Option<String>,
    /// Keystone authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthAdminPort")]
    pub keystone_auth_admin_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthCaCertfile")]
    pub keystone_auth_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthCertfile")]
    pub keystone_auth_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthHost")]
    pub keystone_auth_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthInsecure")]
    pub keystone_auth_insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthKeyfile")]
    pub keystone_auth_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthProjectDomainName")]
    pub keystone_auth_project_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthProto")]
    pub keystone_auth_proto: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthRegionName")]
    pub keystone_auth_region_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthUrlTokens")]
    pub keystone_auth_url_tokens: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthUrlVersion")]
    pub keystone_auth_url_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthUserDomainName")]
    pub keystone_auth_user_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesApiPort")]
    pub kubernetes_api_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesApiSecurePort")]
    pub kubernetes_api_secure_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesPodSubnet")]
    pub kubernetes_pod_subnet: Option<String>,
    /// L3MH
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "l3mhCidr")]
    pub l3mh_cidr: Option<String>,
    /// Logging
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logDir")]
    pub log_dir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLocal")]
    pub log_local: Option<i64>,
    /// Metadata
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataProxySecret")]
    pub metadata_proxy_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSslCaCertfile")]
    pub metadata_ssl_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSslCertType")]
    pub metadata_ssl_cert_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSslCertfile")]
    pub metadata_ssl_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSslEnable")]
    pub metadata_ssl_enable: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataSslKeyfile")]
    pub metadata_ssl_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "physicalInterface")]
    pub physical_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityBandwidth")]
    pub priority_bandwidth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityId")]
    pub priority_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityScheduling")]
    pub priority_scheduling: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityTagging")]
    pub priority_tagging: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "qosDefHwQueue")]
    pub qos_def_hw_queue: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "qosLogicalQueues")]
    pub qos_logical_queues: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "qosQueueId")]
    pub qos_queue_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredKernelVrouterEncryption")]
    pub required_kernel_vrouter_encryption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sampleDestination")]
    pub sample_destination: Option<String>,
    /// Sandesh
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshCaCertfile")]
    pub sandesh_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshCertfile")]
    pub sandesh_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshKeyfile")]
    pub sandesh_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshServerCertfile")]
    pub sandesh_server_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshServerKeyfile")]
    pub sandesh_server_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandeshSslEnable")]
    pub sandesh_ssl_enable: Option<bool>,
    /// Server SSL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCaCertfile")]
    pub server_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertfile")]
    pub server_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverKeyfile")]
    pub server_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sloDestination")]
    pub slo_destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sriovPhysicalInterface")]
    pub sriov_physical_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sriovPhysicalNetwork")]
    pub sriov_physical_network: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sriovVf")]
    pub sriov_vf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslEnable")]
    pub ssl_enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslInsecure")]
    pub ssl_insecure: Option<bool>,
    /// Collector
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statsCollectorDestinationPath")]
    pub stats_collector_destination_path: Option<String>,
    /// XMPP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcluster: Option<String>,
    /// TSN
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tsnAgentMode")]
    pub tsn_agent_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vrouterCryptInterface")]
    pub vrouter_crypt_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vrouterDecryptInterface")]
    pub vrouter_decrypt_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vrouterDecryptKey")]
    pub vrouter_decrypt_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vrouterEncryption")]
    pub vrouter_encryption: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vrouterGateway")]
    pub vrouter_gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmmpSslEnable")]
    pub xmmp_ssl_enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmppServerCaCertfile")]
    pub xmpp_server_ca_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmppServerCertfile")]
    pub xmpp_server_certfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmppServerKeyfile")]
    pub xmpp_server_keyfile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmppServerPort")]
    pub xmpp_server_port: Option<String>,
}

/// Container defines name, image and command.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterServiceConfigurationContainers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VrouterStatus is the Status for vrouter API. TODO: after update to controllter-tool v0.4 rework AgentStatus to make it map instead of [] for performance (https://github.com/operator-framework/operator-sdk/issues/2485 https://github.com/kubernetes-sigs/controller-tools/pull/317)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeOnControllers")]
    pub active_on_controllers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<VrouterStatusAgents>>,
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "operator-sdk generate k8s" to regenerate code after modifying this file Add custom validation using kubebuilder tags: https://book.kubebuilder.io/beyond_basics/generating_crd.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<BTreeMap<String, VrouterStatusNodes>>,
}

/// AgentStatus is the Status of the agent.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterStatusAgents {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analyticsNodes")]
    pub analytics_nodes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configNodes")]
    pub config_nodes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlNodes")]
    pub control_nodes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptedParams")]
    pub encrypted_params: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// AgentServiceStatus is the status value: Starting, Ready, Updating
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "operator-sdk generate k8s" to regenerate code after modifying this file Add custom validation using kubebuilder tags: https://book.kubebuilder.io/beyond_basics/generating_crd.html
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VrouterStatusNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

