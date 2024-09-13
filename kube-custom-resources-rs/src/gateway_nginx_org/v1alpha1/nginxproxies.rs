// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/nginx-kubernetes-gateway/gateway.nginx.org/v1alpha1/nginxproxies.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the desired state of the NginxProxy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "gateway.nginx.org", version = "v1alpha1", kind = "NginxProxy", plural = "nginxproxies")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NginxProxySpec {
    /// DisableHTTP2 defines if http2 should be disabled for all servers.
    /// Default is false, meaning http2 will be enabled for all servers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableHTTP2")]
    pub disable_http2: Option<bool>,
    /// IPFamily specifies the IP family to be used by the NGINX.
    /// Default is "dual", meaning the server will use both IPv4 and IPv6.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipFamily")]
    pub ip_family: Option<NginxProxyIpFamily>,
    /// RewriteClientIP defines configuration for rewriting the client IP to the original client's IP.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rewriteClientIP")]
    pub rewrite_client_ip: Option<NginxProxyRewriteClientIp>,
    /// Telemetry specifies the OpenTelemetry configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<NginxProxyTelemetry>,
}

/// Spec defines the desired state of the NginxProxy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NginxProxyIpFamily {
    #[serde(rename = "dual")]
    Dual,
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

/// RewriteClientIP defines configuration for rewriting the client IP to the original client's IP.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxProxyRewriteClientIp {
    /// Mode defines how NGINX will rewrite the client's IP address.
    /// There are two possible modes:
    /// - ProxyProtocol: NGINX will rewrite the client's IP using the PROXY protocol header.
    /// - XForwardedFor: NGINX will rewrite the client's IP using the X-Forwarded-For header.
    /// Sets NGINX directive real_ip_header: https://nginx.org/en/docs/http/ngx_http_realip_module.html#real_ip_header
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<NginxProxyRewriteClientIpMode>,
    /// SetIPRecursively configures whether recursive search is used when selecting the client's address from
    /// the X-Forwarded-For header. It is used in conjunction with TrustedAddresses.
    /// If enabled, NGINX will recurse on the values in X-Forwarded-Header from the end of array
    /// to start of array and select the first untrusted IP.
    /// For example, if X-Forwarded-For is [11.11.11.11, 22.22.22.22, 55.55.55.1],
    /// and TrustedAddresses is set to 55.55.55.1/32, NGINX will rewrite the client IP to 22.22.22.22.
    /// If disabled, NGINX will select the IP at the end of the array.
    /// In the previous example, 55.55.55.1 would be selected.
    /// Sets NGINX directive real_ip_recursive: https://nginx.org/en/docs/http/ngx_http_realip_module.html#real_ip_recursive
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setIPRecursively")]
    pub set_ip_recursively: Option<bool>,
    /// TrustedAddresses specifies the addresses that are trusted to send correct client IP information.
    /// If a request comes from a trusted address, NGINX will rewrite the client IP information,
    /// and forward it to the backend in the X-Forwarded-For* and X-Real-IP headers.
    /// If the request does not come from a trusted address, NGINX will not rewrite the client IP information.
    /// TrustedAddresses only supports CIDR blocks: 192.33.21.1/24, fe80::1/64.
    /// To trust all addresses (not recommended for production), set to 0.0.0.0/0.
    /// If no addresses are provided, NGINX will not rewrite the client IP information.
    /// Sets NGINX directive set_real_ip_from: https://nginx.org/en/docs/http/ngx_http_realip_module.html#set_real_ip_from
    /// This field is required if mode is set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustedAddresses")]
    pub trusted_addresses: Option<Vec<NginxProxyRewriteClientIpTrustedAddresses>>,
}

/// RewriteClientIP defines configuration for rewriting the client IP to the original client's IP.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NginxProxyRewriteClientIpMode {
    ProxyProtocol,
    XForwardedFor,
}

/// Address is a struct that specifies address type and value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxProxyRewriteClientIpTrustedAddresses {
    /// Type specifies the type of address.
    /// Default is "cidr" which specifies that the address is a CIDR block.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<NginxProxyRewriteClientIpTrustedAddressesType>,
    /// Value specifies the address value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Address is a struct that specifies address type and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NginxProxyRewriteClientIpTrustedAddressesType {
    #[serde(rename = "cidr")]
    Cidr,
}

/// Telemetry specifies the OpenTelemetry configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxProxyTelemetry {
    /// Exporter specifies OpenTelemetry export parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exporter: Option<NginxProxyTelemetryExporter>,
    /// ServiceName is the "service.name" attribute of the OpenTelemetry resource.
    /// Default is 'ngf:<gateway-namespace>:<gateway-name>'. If a value is provided by the user,
    /// then the default becomes a prefix to that value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
    /// SpanAttributes are custom key/value attributes that are added to each span.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spanAttributes")]
    pub span_attributes: Option<Vec<NginxProxyTelemetrySpanAttributes>>,
}

/// Exporter specifies OpenTelemetry export parameters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxProxyTelemetryExporter {
    /// BatchCount is the number of pending batches per worker, spans exceeding the limit are dropped.
    /// Default: https://nginx.org/en/docs/ngx_otel_module.html#otel_exporter
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchCount")]
    pub batch_count: Option<i32>,
    /// BatchSize is the maximum number of spans to be sent in one batch per worker.
    /// Default: https://nginx.org/en/docs/ngx_otel_module.html#otel_exporter
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchSize")]
    pub batch_size: Option<i32>,
    /// Endpoint is the address of OTLP/gRPC endpoint that will accept telemetry data.
    /// Format: alphanumeric hostname with optional http scheme and optional port.
    pub endpoint: String,
    /// Interval is the maximum interval between two exports.
    /// Default: https://nginx.org/en/docs/ngx_otel_module.html#otel_exporter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

/// SpanAttribute is a key value pair to be added to a tracing span.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NginxProxyTelemetrySpanAttributes {
    /// Key is the key for a span attribute.
    /// Format: must have all '"' escaped and must not contain any '$' or end with an unescaped '\'
    pub key: String,
    /// Value is the value for a span attribute.
    /// Format: must have all '"' escaped and must not contain any '$' or end with an unescaped '\'
    pub value: String,
}

