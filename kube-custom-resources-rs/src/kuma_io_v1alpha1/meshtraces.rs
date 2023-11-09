// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshtraces.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Spec is the specification of the Kuma MeshTrace resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshTrace", plural = "meshtraces")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct MeshTraceSpec {
    /// MeshTrace configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshTraceDefault>,
    /// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshTraceTargetRef,
}

/// MeshTrace configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefault {
    /// A one element array of backend definition. Envoy allows configuring only 1 backend, so the natural way of representing that would be just one object. Unfortunately due to the reasons explained in MADR 009-tracing-policy this has to be a one element array for now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<MeshTraceDefaultBackends>>,
    /// Sampling configuration. Sampling is the process by which a decision is made on whether to process/export a span or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<MeshTraceDefaultSampling>,
    /// Custom tags configuration. You can add custom tags to traces based on headers or literal values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MeshTraceDefaultTags>>,
}

/// Only one of zipkin, datadog or openTelemetry can be used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultBackends {
    /// Datadog backend configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datadog: Option<MeshTraceDefaultBackendsDatadog>,
    /// OpenTelemetry backend configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<MeshTraceDefaultBackendsOpenTelemetry>,
    #[serde(rename = "type")]
    pub r#type: MeshTraceDefaultBackendsType,
    /// Zipkin backend configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zipkin: Option<MeshTraceDefaultBackendsZipkin>,
}

/// Datadog backend configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultBackendsDatadog {
    /// Determines if datadog service name should be split based on traffic direction and destination. For example, with `splitService: true` and a `backend` service that communicates with a couple of databases, you would get service names like `backend_INBOUND`, `backend_OUTBOUND_db1`, and `backend_OUTBOUND_db2` in Datadog. Default: false
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "splitService")]
    pub split_service: Option<bool>,
    /// Address of Datadog collector, only host and port are allowed (no paths, fragments etc.)
    pub url: String,
}

/// OpenTelemetry backend configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultBackendsOpenTelemetry {
    /// Address of OpenTelemetry collector.
    pub endpoint: String,
}

/// Only one of zipkin, datadog or openTelemetry can be used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTraceDefaultBackendsType {
    Zipkin,
    Datadog,
    OpenTelemetry,
}

/// Zipkin backend configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultBackendsZipkin {
    /// Version of the API. values: httpJson, httpProto. Default: httpJson see https://github.com/envoyproxy/envoy/blob/v1.22.0/api/envoy/config/trace/v3/zipkin.proto#L66
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<MeshTraceDefaultBackendsZipkinApiVersion>,
    /// Determines whether client and server spans will share the same span context. Default: true. https://github.com/envoyproxy/envoy/blob/v1.22.0/api/envoy/config/trace/v3/zipkin.proto#L63
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharedSpanContext")]
    pub shared_span_context: Option<bool>,
    /// Generate 128bit traces. Default: false
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "traceId128bit")]
    pub trace_id128bit: Option<bool>,
    /// Address of Zipkin collector.
    pub url: String,
}

/// Zipkin backend configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTraceDefaultBackendsZipkinApiVersion {
    #[serde(rename = "httpJson")]
    HttpJson,
    #[serde(rename = "httpProto")]
    HttpProto,
}

/// Sampling configuration. Sampling is the process by which a decision is made on whether to process/export a span or not.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultSampling {
    /// Target percentage of requests that will be force traced if the 'x-client-trace-id' header is set. Default: 100% Mirror of client_sampling in Envoy https://github.com/envoyproxy/envoy/blob/v1.22.0/api/envoy/config/filter/network/http_connection_manager/v2/http_connection_manager.proto#L127-L133 Either int or decimal represented as string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<IntOrString>,
    /// Target percentage of requests will be traced after all other sampling checks have been applied (client, force tracing, random sampling). This field functions as an upper limit on the total configured sampling rate. For instance, setting client_sampling to 100% but overall_sampling to 1% will result in only 1% of client requests with the appropriate headers to be force traced. Default: 100% Mirror of overall_sampling in Envoy https://github.com/envoyproxy/envoy/blob/v1.22.0/api/envoy/config/filter/network/http_connection_manager/v2/http_connection_manager.proto#L142-L150 Either int or decimal represented as string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overall: Option<IntOrString>,
    /// Target percentage of requests that will be randomly selected for trace generation, if not requested by the client or not forced. Default: 100% Mirror of random_sampling in Envoy https://github.com/envoyproxy/envoy/blob/v1.22.0/api/envoy/config/filter/network/http_connection_manager/v2/http_connection_manager.proto#L135-L140 Either int or decimal represented as string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random: Option<IntOrString>,
}

/// Custom tags configuration. Only one of literal or header can be used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultTags {
    /// Tag taken from a header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<MeshTraceDefaultTagsHeader>,
    /// Tag taken from literal value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    /// Name of the tag.
    pub name: String,
}

/// Tag taken from a header.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceDefaultTagsHeader {
    /// Default value to use if header is missing. If the default is missing and there is no value the tag will not be included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Name of the header.
    pub name: String,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshTraceTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshTraceTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshTraceTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

