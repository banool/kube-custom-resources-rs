// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-cluster-management-io/ocm/operator.open-cluster-management.io/v1/klusterlets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec represents the desired deployment configuration of Klusterlet agent.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.open-cluster-management.io", version = "v1", kind = "Klusterlet", plural = "klusterlets")]
#[kube(status = "KlusterletStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KlusterletSpec {
    /// ClusterName is the name of the managed cluster to be created on hub.
    /// The Klusterlet agent generates a random name if it is not set, or discovers the appropriate cluster name on OpenShift.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// DeployOption contains the options of deploying a klusterlet
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deployOption")]
    pub deploy_option: Option<KlusterletDeployOption>,
    /// ExternalServerURLs represents a list of apiserver urls and ca bundles that is accessible externally
    /// If it is set empty, managed cluster has no externally accessible url that hub cluster can visit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalServerURLs")]
    pub external_server_ur_ls: Option<Vec<KlusterletExternalServerUrLs>>,
    /// HubApiServerHostAlias contains the host alias for hub api server.
    /// registration-agent and work-agent will use it to communicate with hub api server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hubApiServerHostAlias")]
    pub hub_api_server_host_alias: Option<KlusterletHubApiServerHostAlias>,
    /// ImagePullSpec represents the desired image configuration of agent, it takes effect only when
    /// singleton mode is set. quay.io/open-cluster-management.io/registration-operator:latest will
    /// be used if unspecified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSpec")]
    pub image_pull_spec: Option<String>,
    /// Namespace is the namespace to deploy the agent on the managed cluster.
    /// The namespace must have a prefix of "open-cluster-management-", and if it is not set,
    /// the namespace of "open-cluster-management-agent" is used to deploy agent.
    /// In addition, the add-ons are deployed to the namespace of "{Namespace}-addon".
    /// In the Hosted mode, this namespace still exists on the managed cluster to contain
    /// necessary resources, like service accounts, roles and rolebindings, while the agent
    /// is deployed to the namespace with the same name as klusterlet on the management cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// NodePlacement enables explicit control over the scheduling of the deployed pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodePlacement")]
    pub node_placement: Option<KlusterletNodePlacement>,
    /// PriorityClassName is the name of the PriorityClass that will be used by the
    /// deployed klusterlet agent. It will be ignored when the PriorityClass/v1 API
    /// is not available on the managed cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityClassName")]
    pub priority_class_name: Option<String>,
    /// RegistrationConfiguration contains the configuration of registration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrationConfiguration")]
    pub registration_configuration: Option<KlusterletRegistrationConfiguration>,
    /// RegistrationImagePullSpec represents the desired image configuration of registration agent.
    /// quay.io/open-cluster-management.io/registration:latest will be used if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrationImagePullSpec")]
    pub registration_image_pull_spec: Option<String>,
    /// ResourceRequirement specify QoS classes of deployments managed by klusterlet.
    /// It applies to all the containers in the deployments.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRequirement")]
    pub resource_requirement: Option<KlusterletResourceRequirement>,
    /// WorkConfiguration contains the configuration of work
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workConfiguration")]
    pub work_configuration: Option<KlusterletWorkConfiguration>,
    /// WorkImagePullSpec represents the desired image configuration of work agent.
    /// quay.io/open-cluster-management.io/work:latest will be used if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workImagePullSpec")]
    pub work_image_pull_spec: Option<String>,
}

/// DeployOption contains the options of deploying a klusterlet
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletDeployOption {
    /// Mode can be Default, Hosted, Singleton or SingletonHosted. It is Default mode if not specified
    /// In Default mode, all klusterlet related resources are deployed on the managed cluster.
    /// In Hosted mode, only crd and configurations are installed on the spoke/managed cluster. Controllers run in another
    /// cluster (defined as management-cluster) and connect to the mangaged cluster with the kubeconfig in secret of
    /// "external-managed-kubeconfig"(a kubeconfig of managed-cluster with cluster-admin permission).
    /// In Singleton mode, registration/work agent is started as a single deployment.
    /// In SingletonHosted mode, agent is started as a single deployment in hosted mode.
    /// Note: Do not modify the Mode field once it's applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// ServerURL represents the apiserver url and ca bundle that is accessible externally
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletExternalServerUrLs {
    /// CABundle is the ca bundle to connect to apiserver of the managed cluster.
    /// System certs are used if it is not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caBundle")]
    pub ca_bundle: Option<String>,
    /// URL is the url of apiserver endpoint of the managed cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// HubApiServerHostAlias contains the host alias for hub api server.
/// registration-agent and work-agent will use it to communicate with hub api server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletHubApiServerHostAlias {
    /// Hostname for the above IP address.
    pub hostname: String,
    /// IP address of the host file entry.
    pub ip: String,
}

/// NodePlacement enables explicit control over the scheduling of the deployed pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletNodePlacement {
    /// NodeSelector defines which Nodes the Pods are scheduled on. The default is an empty list.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// Tolerations are attached by pods to tolerate any taint that matches
    /// the triple <key,value,effect> using the matching operator <operator>.
    /// The default is an empty list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<KlusterletNodePlacementTolerations>>,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletNodePlacementTolerations {
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

/// RegistrationConfiguration contains the configuration of registration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletRegistrationConfiguration {
    /// BootstrapKubeConfigs defines the ordered list of bootstrap kubeconfigs. The order decides which bootstrap kubeconfig to use first when rebootstrap.
    /// 
    /// 
    /// When the agent loses the connection to the current hub over HubConnectionTimeoutSeconds, or the managedcluster CR
    /// is set `hubAcceptsClient=false` on the hub, the controller marks the related bootstrap kubeconfig as "failed".
    /// 
    /// 
    /// A failed bootstrapkubeconfig won't be used for the duration specified by SkipFailedBootstrapKubeConfigSeconds.
    /// But if the user updates the content of a failed bootstrapkubeconfig, the "failed" mark will be cleared.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootstrapKubeConfigs")]
    pub bootstrap_kube_configs: Option<KlusterletRegistrationConfigurationBootstrapKubeConfigs>,
    /// clientCertExpirationSeconds represents the seconds of a client certificate to expire. If it is not set or 0, the default
    /// duration seconds will be set by the hub cluster. If the value is larger than the max signing duration seconds set on
    /// the hub cluster, the max signing duration seconds will be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertExpirationSeconds")]
    pub client_cert_expiration_seconds: Option<i32>,
    /// ClusterAnnotations is annotations with the reserve prefix "agent.open-cluster-management.io" set on
    /// ManagedCluster when creating only, other actors can update it afterwards.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAnnotations")]
    pub cluster_annotations: Option<BTreeMap<String, String>>,
    /// FeatureGates represents the list of feature gates for registration
    /// If it is set empty, default feature gates will be used.
    /// If it is set, featuregate/Foo is an example of one item in FeatureGates:
    ///   1. If featuregate/Foo does not exist, registration-operator will discard it
    ///   2. If featuregate/Foo exists and is false by default. It is now possible to set featuregate/Foo=[false|true]
    ///   3. If featuregate/Foo exists and is true by default. If a cluster-admin upgrading from 1 to 2 wants to continue having featuregate/Foo=false,
    ///  	he can set featuregate/Foo=false before upgrading. Let's say the cluster-admin wants featuregate/Foo=false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGates")]
    pub feature_gates: Option<Vec<KlusterletRegistrationConfigurationFeatureGates>>,
    /// KubeAPIBurst indicates the maximum burst of the throttle while talking with apiserver of hub cluster from the spoke cluster.
    /// If it is set empty, use the default value: 100
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIBurst")]
    pub kube_api_burst: Option<i32>,
    /// KubeAPIQPS indicates the maximum QPS while talking with apiserver of hub cluster from the spoke cluster.
    /// If it is set empty, use the default value: 50
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIQPS")]
    pub kube_apiqps: Option<i32>,
}

/// BootstrapKubeConfigs defines the ordered list of bootstrap kubeconfigs. The order decides which bootstrap kubeconfig to use first when rebootstrap.
/// 
/// 
/// When the agent loses the connection to the current hub over HubConnectionTimeoutSeconds, or the managedcluster CR
/// is set `hubAcceptsClient=false` on the hub, the controller marks the related bootstrap kubeconfig as "failed".
/// 
/// 
/// A failed bootstrapkubeconfig won't be used for the duration specified by SkipFailedBootstrapKubeConfigSeconds.
/// But if the user updates the content of a failed bootstrapkubeconfig, the "failed" mark will be cleared.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletRegistrationConfigurationBootstrapKubeConfigs {
    /// LocalSecretsConfig include a list of secrets that contains the kubeconfigs for ordered bootstrap kubeconifigs.
    /// The secrets must be in the same namespace where the agent controller runs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localSecretsConfig")]
    pub local_secrets_config: Option<KlusterletRegistrationConfigurationBootstrapKubeConfigsLocalSecretsConfig>,
    /// Type specifies the type of priority bootstrap kubeconfigs.
    /// By default, it is set to None, representing no priority bootstrap kubeconfigs are set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<KlusterletRegistrationConfigurationBootstrapKubeConfigsType>,
}

/// LocalSecretsConfig include a list of secrets that contains the kubeconfigs for ordered bootstrap kubeconifigs.
/// The secrets must be in the same namespace where the agent controller runs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletRegistrationConfigurationBootstrapKubeConfigsLocalSecretsConfig {
    /// HubConnectionTimeoutSeconds is used to set the timeout of connecting to the hub cluster.
    /// When agent loses the connection to the hub over the timeout seconds, the agent do a rebootstrap.
    /// By default is 10 mins.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hubConnectionTimeoutSeconds")]
    pub hub_connection_timeout_seconds: Option<i32>,
    /// KubeConfigSecrets is a list of secret names. The secrets are in the same namespace where the agent controller runs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeConfigSecrets")]
    pub kube_config_secrets: Option<Vec<KlusterletRegistrationConfigurationBootstrapKubeConfigsLocalSecretsConfigKubeConfigSecrets>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletRegistrationConfigurationBootstrapKubeConfigsLocalSecretsConfigKubeConfigSecrets {
    /// Name is the name of the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// BootstrapKubeConfigs defines the ordered list of bootstrap kubeconfigs. The order decides which bootstrap kubeconfig to use first when rebootstrap.
/// 
/// 
/// When the agent loses the connection to the current hub over HubConnectionTimeoutSeconds, or the managedcluster CR
/// is set `hubAcceptsClient=false` on the hub, the controller marks the related bootstrap kubeconfig as "failed".
/// 
/// 
/// A failed bootstrapkubeconfig won't be used for the duration specified by SkipFailedBootstrapKubeConfigSeconds.
/// But if the user updates the content of a failed bootstrapkubeconfig, the "failed" mark will be cleared.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KlusterletRegistrationConfigurationBootstrapKubeConfigsType {
    None,
    LocalSecrets,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletRegistrationConfigurationFeatureGates {
    /// Feature is the key of feature gate. e.g. featuregate/Foo.
    pub feature: String,
    /// Mode is either Enable, Disable, "" where "" is Disable by default.
    /// In Enable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=true".
    /// In Disable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=false".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<KlusterletRegistrationConfigurationFeatureGatesMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KlusterletRegistrationConfigurationFeatureGatesMode {
    Enable,
    Disable,
}

/// ResourceRequirement specify QoS classes of deployments managed by klusterlet.
/// It applies to all the containers in the deployments.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletResourceRequirement {
    /// ResourceRequirements defines resource requests and limits when Type is ResourceQosClassResourceRequirement
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRequirements")]
    pub resource_requirements: Option<KlusterletResourceRequirementResourceRequirements>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<KlusterletResourceRequirementType>,
}

/// ResourceRequirements defines resource requests and limits when Type is ResourceQosClassResourceRequirement
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletResourceRequirementResourceRequirements {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<KlusterletResourceRequirementResourceRequirementsClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletResourceRequirementResourceRequirementsClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// ResourceRequirement specify QoS classes of deployments managed by klusterlet.
/// It applies to all the containers in the deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KlusterletResourceRequirementType {
    Default,
    BestEffort,
    ResourceRequirement,
}

/// WorkConfiguration contains the configuration of work
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletWorkConfiguration {
    /// AppliedManifestWorkEvictionGracePeriod is the eviction grace period the work agent will wait before
    /// evicting the AppliedManifestWorks, whose corresponding ManifestWorks are missing on the hub cluster, from
    /// the managed cluster. If not present, the default value of the work agent will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedManifestWorkEvictionGracePeriod")]
    pub applied_manifest_work_eviction_grace_period: Option<String>,
    /// FeatureGates represents the list of feature gates for work
    /// If it is set empty, default feature gates will be used.
    /// If it is set, featuregate/Foo is an example of one item in FeatureGates:
    ///   1. If featuregate/Foo does not exist, registration-operator will discard it
    ///   2. If featuregate/Foo exists and is false by default. It is now possible to set featuregate/Foo=[false|true]
    ///   3. If featuregate/Foo exists and is true by default. If a cluster-admin upgrading from 1 to 2 wants to continue having featuregate/Foo=false,
    ///  	he can set featuregate/Foo=false before upgrading. Let's say the cluster-admin wants featuregate/Foo=false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGates")]
    pub feature_gates: Option<Vec<KlusterletWorkConfigurationFeatureGates>>,
    /// KubeAPIBurst indicates the maximum burst of the throttle while talking with apiserver of hub cluster from the spoke cluster.
    /// If it is set empty, use the default value: 100
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIBurst")]
    pub kube_api_burst: Option<i32>,
    /// KubeAPIQPS indicates the maximum QPS while talking with apiserver of hub cluster from the spoke cluster.
    /// If it is set empty, use the default value: 50
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIQPS")]
    pub kube_apiqps: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletWorkConfigurationFeatureGates {
    /// Feature is the key of feature gate. e.g. featuregate/Foo.
    pub feature: String,
    /// Mode is either Enable, Disable, "" where "" is Disable by default.
    /// In Enable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=true".
    /// In Disable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=false".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<KlusterletWorkConfigurationFeatureGatesMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KlusterletWorkConfigurationFeatureGatesMode {
    Enable,
    Disable,
}

/// Status represents the current status of Klusterlet agent.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletStatus {
    /// Conditions contain the different condition statuses for this Klusterlet.
    /// Valid condition types are:
    /// Applied: Components have been applied in the managed cluster.
    /// Available: Components in the managed cluster are available and ready to serve.
    /// Progressing: Components in the managed cluster are in a transitioning state.
    /// Degraded: Components in the managed cluster do not match the desired configuration and only provide
    /// degraded service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Generations are used to determine when an item needs to be reconciled or has changed in a way that needs a reaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<KlusterletStatusGenerations>>,
    /// ObservedGeneration is the last generation change you've dealt with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// RelatedResources are used to track the resources that are related to this Klusterlet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relatedResources")]
    pub related_resources: Option<Vec<KlusterletStatusRelatedResources>>,
}

/// GenerationStatus keeps track of the generation for a given resource so that decisions about forced updates can be made.
/// The definition matches the GenerationStatus defined in github.com/openshift/api/v1
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletStatusGenerations {
    /// group is the group of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// lastGeneration is the last generation of the resource that controller applies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastGeneration")]
    pub last_generation: Option<i64>,
    /// name is the name of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the resource that you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// version is the version of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// RelatedResourceMeta represents the resource that is managed by an operator
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KlusterletStatusRelatedResources {
    /// group is the group of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// name is the name of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the thing you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// version is the version of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

