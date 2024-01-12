// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/external-secrets/external-secrets/external-secrets.io/v1beta1/clusterexternalsecrets.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ClusterExternalSecretSpec defines the desired state of ClusterExternalSecret.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "external-secrets.io", version = "v1beta1", kind = "ClusterExternalSecret", plural = "clusterexternalsecrets")]
#[kube(status = "ClusterExternalSecretStatus")]
#[kube(schema = "disabled")]
pub struct ClusterExternalSecretSpec {
    /// The metadata of the external secrets to be created
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalSecretMetadata")]
    pub external_secret_metadata: Option<ClusterExternalSecretExternalSecretMetadata>,
    /// The name of the external secrets to be created defaults to the name of the ClusterExternalSecret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalSecretName")]
    pub external_secret_name: Option<String>,
    /// The spec for the ExternalSecrets to be created
    #[serde(rename = "externalSecretSpec")]
    pub external_secret_spec: ClusterExternalSecretExternalSecretSpec,
    /// The labels to select by to find the Namespaces to create the ExternalSecrets in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ClusterExternalSecretNamespaceSelector>,
    /// Choose namespaces by name. This field is ORed with anything that NamespaceSelector ends up choosing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// The time in which the controller should reconcile its objects and recheck namespaces for labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshTime")]
    pub refresh_time: Option<String>,
}

/// The metadata of the external secrets to be created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// The spec for the ExternalSecrets to be created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpec {
    /// Data defines the connection between the Kubernetes Secret keys and the Provider data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ClusterExternalSecretExternalSecretSpecData>>,
    /// DataFrom is used to fetch all properties from a specific Provider data If multiple entries are specified, the Secret keys are merged in the specified order
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataFrom")]
    pub data_from: Option<Vec<ClusterExternalSecretExternalSecretSpecDataFrom>>,
    /// RefreshInterval is the amount of time before the values are read again from the SecretStore provider Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h" May be set to zero to fetch and create it once. Defaults to 1h.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretStoreRef")]
    pub secret_store_ref: Option<ClusterExternalSecretExternalSecretSpecSecretStoreRef>,
    /// ExternalSecretTarget defines the Kubernetes Secret to be created There can be only one target per ExternalSecret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ClusterExternalSecretExternalSecretSpecTarget>,
}

/// ExternalSecretData defines the connection between the Kubernetes Secret key (spec.data.<key>) and the Provider data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecData {
    /// RemoteRef points to the remote secret and defines which secret (version/property/..) to fetch.
    #[serde(rename = "remoteRef")]
    pub remote_ref: ClusterExternalSecretExternalSecretSpecDataRemoteRef,
    /// SecretKey defines the key in which the controller stores the value. This is the key in the Kind=Secret
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    /// SourceRef allows you to override the source from which the value will pulled from.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRef")]
    pub source_ref: Option<ClusterExternalSecretExternalSecretSpecDataSourceRef>,
}

/// RemoteRef points to the remote secret and defines which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataRemoteRef {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ClusterExternalSecretExternalSecretSpecDataRemoteRefConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ClusterExternalSecretExternalSecretSpecDataRemoteRefDecodingStrategy>,
    /// Key is the key used in the Provider, mandatory
    pub key: String,
    /// Policy for fetching tags/labels from provider secrets, possible options are Fetch, None. Defaults to None
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataPolicy")]
    pub metadata_policy: Option<ClusterExternalSecretExternalSecretSpecDataRemoteRefMetadataPolicy>,
    /// Used to select a specific property of the Provider value (if a map), if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Used to select a specific version of the Provider value, if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// RemoteRef points to the remote secret and defines which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataRemoteRefConversionStrategy {
    Default,
    Unicode,
}

/// RemoteRef points to the remote secret and defines which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataRemoteRefDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// RemoteRef points to the remote secret and defines which secret (version/property/..) to fetch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataRemoteRefMetadataPolicy {
    None,
    Fetch,
}

/// SourceRef allows you to override the source from which the value will pulled from.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataSourceRef {
    /// GeneratorRef points to a generator custom resource. 
    ///  Deprecated: The generatorRef is not implemented in .data[]. this will be removed with v1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatorRef")]
    pub generator_ref: Option<ClusterExternalSecretExternalSecretSpecDataSourceRefGeneratorRef>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storeRef")]
    pub store_ref: Option<ClusterExternalSecretExternalSecretSpecDataSourceRefStoreRef>,
}

/// GeneratorRef points to a generator custom resource. 
///  Deprecated: The generatorRef is not implemented in .data[]. this will be removed with v1.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataSourceRefGeneratorRef {
    /// Specify the apiVersion of the generator resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Specify the Kind of the resource, e.g. Password, ACRAccessToken etc.
    pub kind: String,
    /// Specify the name of the generator resource
    pub name: String,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataSourceRefStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore) Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the SecretStore resource
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFrom {
    /// Used to extract multiple key/value pairs from one secret Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract: Option<ClusterExternalSecretExternalSecretSpecDataFromExtract>,
    /// Used to find secrets based on tags or regular expressions Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<ClusterExternalSecretExternalSecretSpecDataFromFind>,
    /// Used to rewrite secret Keys after getting them from the secret Provider Multiple Rewrite operations can be provided. They are applied in a layered order (first to last)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<Vec<ClusterExternalSecretExternalSecretSpecDataFromRewrite>>,
    /// SourceRef points to a store or generator which contains secret values ready to use. Use this in combination with Extract or Find pull values out of a specific SecretStore. When sourceRef points to a generator Extract or Find is not supported. The generator returns a static map of values
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRef")]
    pub source_ref: Option<ClusterExternalSecretExternalSecretSpecDataFromSourceRef>,
}

/// Used to extract multiple key/value pairs from one secret Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromExtract {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ClusterExternalSecretExternalSecretSpecDataFromExtractConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ClusterExternalSecretExternalSecretSpecDataFromExtractDecodingStrategy>,
    /// Key is the key used in the Provider, mandatory
    pub key: String,
    /// Policy for fetching tags/labels from provider secrets, possible options are Fetch, None. Defaults to None
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataPolicy")]
    pub metadata_policy: Option<ClusterExternalSecretExternalSecretSpecDataFromExtractMetadataPolicy>,
    /// Used to select a specific property of the Provider value (if a map), if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Used to select a specific version of the Provider value, if supported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Used to extract multiple key/value pairs from one secret Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataFromExtractConversionStrategy {
    Default,
    Unicode,
}

/// Used to extract multiple key/value pairs from one secret Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataFromExtractDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// Used to extract multiple key/value pairs from one secret Note: Extract does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataFromExtractMetadataPolicy {
    None,
    Fetch,
}

/// Used to find secrets based on tags or regular expressions Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromFind {
    /// Used to define a conversion Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conversionStrategy")]
    pub conversion_strategy: Option<ClusterExternalSecretExternalSecretSpecDataFromFindConversionStrategy>,
    /// Used to define a decoding Strategy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decodingStrategy")]
    pub decoding_strategy: Option<ClusterExternalSecretExternalSecretSpecDataFromFindDecodingStrategy>,
    /// Finds secrets based on the name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ClusterExternalSecretExternalSecretSpecDataFromFindName>,
    /// A root path to start the find operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Find secrets based on tags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// Used to find secrets based on tags or regular expressions Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataFromFindConversionStrategy {
    Default,
    Unicode,
}

/// Used to find secrets based on tags or regular expressions Note: Find does not support sourceRef.Generator or sourceRef.GeneratorRef.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecDataFromFindDecodingStrategy {
    Auto,
    Base64,
    #[serde(rename = "Base64URL")]
    Base64Url,
    None,
}

/// Finds secrets based on the name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromFindName {
    /// Finds secrets base
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromRewrite {
    /// Used to rewrite with regular expressions. The resulting key will be the output of a regexp.ReplaceAll operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<ClusterExternalSecretExternalSecretSpecDataFromRewriteRegexp>,
    /// Used to apply string transformation on the secrets. The resulting key will be the output of the template applied by the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<ClusterExternalSecretExternalSecretSpecDataFromRewriteTransform>,
}

/// Used to rewrite with regular expressions. The resulting key will be the output of a regexp.ReplaceAll operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromRewriteRegexp {
    /// Used to define the regular expression of a re.Compiler.
    pub source: String,
    /// Used to define the target pattern of a ReplaceAll operation.
    pub target: String,
}

/// Used to apply string transformation on the secrets. The resulting key will be the output of the template applied by the operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromRewriteTransform {
    /// Used to define the template to apply on the secret name. `.value ` will specify the secret name in the template.
    pub template: String,
}

/// SourceRef points to a store or generator which contains secret values ready to use. Use this in combination with Extract or Find pull values out of a specific SecretStore. When sourceRef points to a generator Extract or Find is not supported. The generator returns a static map of values
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromSourceRef {
    /// GeneratorRef points to a generator custom resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatorRef")]
    pub generator_ref: Option<ClusterExternalSecretExternalSecretSpecDataFromSourceRefGeneratorRef>,
    /// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storeRef")]
    pub store_ref: Option<ClusterExternalSecretExternalSecretSpecDataFromSourceRefStoreRef>,
}

/// GeneratorRef points to a generator custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromSourceRefGeneratorRef {
    /// Specify the apiVersion of the generator resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Specify the Kind of the resource, e.g. Password, ACRAccessToken etc.
    pub kind: String,
    /// Specify the name of the generator resource
    pub name: String,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecDataFromSourceRefStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore) Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the SecretStore resource
    pub name: String,
}

/// SecretStoreRef defines which SecretStore to fetch the ExternalSecret data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecSecretStoreRef {
    /// Kind of the SecretStore resource (SecretStore or ClusterSecretStore) Defaults to `SecretStore`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the SecretStore resource
    pub name: String,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTarget {
    /// CreationPolicy defines rules on how to create the resulting Secret Defaults to 'Owner'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationPolicy")]
    pub creation_policy: Option<ClusterExternalSecretExternalSecretSpecTargetCreationPolicy>,
    /// DeletionPolicy defines rules on how to delete the resulting Secret Defaults to 'Retain'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionPolicy")]
    pub deletion_policy: Option<ClusterExternalSecretExternalSecretSpecTargetDeletionPolicy>,
    /// Immutable defines if the final secret will be immutable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// Name defines the name of the Secret resource to be managed This field is immutable Defaults to the .metadata.name of the ExternalSecret resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Template defines a blueprint for the created Secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ClusterExternalSecretExternalSecretSpecTargetTemplate>,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetCreationPolicy {
    Owner,
    Orphan,
    Merge,
    None,
}

/// ExternalSecretTarget defines the Kubernetes Secret to be created There can be only one target per ExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetDeletionPolicy {
    Delete,
    Merge,
    Retain,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, String>>,
    /// EngineVersion specifies the template engine version that should be used to compile/execute the template specified in .data and .templateFrom[].
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateEngineVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mergePolicy")]
    pub merge_policy: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateMergePolicy>,
    /// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateFrom")]
    pub template_from: Option<Vec<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetTemplateEngineVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// Template defines a blueprint for the created Secret resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetTemplateMergePolicy {
    Replace,
    Merge,
}

/// ExternalSecretTemplateMetadata defines metadata fields for the Secret blueprint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromTarget>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMap {
    pub items: Vec<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMapItems>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMapItems {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMapItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromConfigMapItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecret {
    pub items: Vec<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecretItems>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecretItems {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateAs")]
    pub template_as: Option<ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecretItemsTemplateAs>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromSecretItemsTemplateAs {
    Values,
    KeysAndValues,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterExternalSecretExternalSecretSpecTargetTemplateTemplateFromTarget {
    Data,
    Annotations,
    Labels,
}

/// The labels to select by to find the Namespaces to create the ExternalSecrets in.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterExternalSecretNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ClusterExternalSecretStatus defines the observed state of ClusterExternalSecret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ClusterExternalSecretStatusConditions>>,
    /// ExternalSecretName is the name of the ExternalSecrets created by the ClusterExternalSecret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalSecretName")]
    pub external_secret_name: Option<String>,
    /// Failed namespaces are the namespaces that failed to apply an ExternalSecret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedNamespaces")]
    pub failed_namespaces: Option<Vec<ClusterExternalSecretStatusFailedNamespaces>>,
    /// ProvisionedNamespaces are the namespaces where the ClusterExternalSecret has secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedNamespaces")]
    pub provisioned_namespaces: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// ClusterExternalSecretNamespaceFailure represents a failed namespace deployment and it's reason.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalSecretStatusFailedNamespaces {
    /// Namespace is the namespace that failed when trying to apply an ExternalSecret
    pub namespace: String,
    /// Reason is why the ExternalSecret failed to apply to the namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

