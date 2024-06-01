// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluent/fluent-operator/fluentbit.fluent.io/v1alpha2/clusterinputs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// InputSpec defines the desired state of ClusterInput
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "fluentbit.fluent.io", version = "v1alpha2", kind = "ClusterInput", plural = "clusterinputs")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterInputSpec {
    /// A user friendly alias name for this input plugin. Used in metrics for distinction of each configured input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Collectd defines the Collectd input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectd: Option<ClusterInputCollectd>,
    /// CustomPlugin defines Custom Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customPlugin")]
    pub custom_plugin: Option<ClusterInputCustomPlugin>,
    /// Dummy defines Dummy Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dummy: Option<ClusterInputDummy>,
    /// FluentBitMetrics defines Fluent Bit Metrics Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fluentBitMetrics")]
    pub fluent_bit_metrics: Option<ClusterInputFluentBitMetrics>,
    /// Forward defines forward  input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward: Option<ClusterInputForward>,
    /// HTTP defines the HTTP input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<ClusterInputHttp>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<ClusterInputLogLevel>,
    /// MQTT defines the MQTT input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<ClusterInputMqtt>,
    /// Nginx defines the Nginx input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nginx: Option<ClusterInputNginx>,
    /// NodeExporterMetrics defines Node Exporter Metrics Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeExporterMetrics")]
    pub node_exporter_metrics: Option<ClusterInputNodeExporterMetrics>,
    /// OpenTelemetry defines the OpenTelemetry input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<ClusterInputOpenTelemetry>,
    /// PrometheusScrapeMetrics  defines Prometheus Scrape Metrics Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prometheusScrapeMetrics")]
    pub prometheus_scrape_metrics: Option<ClusterInputPrometheusScrapeMetrics>,
    /// StatsD defines the StatsD input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statsd: Option<ClusterInputStatsd>,
    /// Syslog defines the Syslog input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syslog: Option<ClusterInputSyslog>,
    /// Systemd defines Systemd Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub systemd: Option<ClusterInputSystemd>,
    /// Tail defines Tail Input configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<ClusterInputTail>,
    /// TCP defines the TCP input plugin configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<ClusterInputTcp>,
}

/// Collectd defines the Collectd input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputCollectd {
    /// Set the address to listen to, default: 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// Set the port to listen to, default: 25826
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Set the data specification file,default: /usr/share/collectd/types.db
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typesDB")]
    pub types_db: Option<String>,
}

/// CustomPlugin defines Custom Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputCustomPlugin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

/// Dummy defines Dummy Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputDummy {
    /// Dummy JSON record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dummy: Option<String>,
    /// Events number generated per second.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i32>,
    /// Sample events to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samples: Option<i32>,
    /// Tag name associated to all records comming from this plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// FluentBitMetrics defines Fluent Bit Metrics Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputFluentBitMetrics {
    /// The rate at which metrics are collected from the host operating system. default is 2 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeInterval")]
    pub scrape_interval: Option<String>,
    /// Scrape metrics upon start, useful to avoid waiting for 'scrape_interval' for the first round of metrics.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeOnStart")]
    pub scrape_on_start: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Forward defines forward  input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputForward {
    /// Specify maximum buffer memory size used to recieve a forward message. The value must be according to the Unit Size specification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferMaxSize")]
    pub buffer_max_size: Option<String>,
    /// Set the initial buffer size to store incoming data. This value is used too to increase buffer size as required. The value must be according to the Unit Size specification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferchunkSize")]
    pub bufferchunk_size: Option<String>,
    /// Listener network interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// Port for forward plugin instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// in_forward uses the tag value for incoming logs. If not set it uses tag from incoming log.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Adds the prefix to incoming event's tag
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagPrefix")]
    pub tag_prefix: Option<String>,
    /// Threaded mechanism allows input plugin to run in a separate thread which helps to desaturate the main pipeline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threaded: Option<String>,
    /// Specify the path to unix socket to recieve a forward message. If set, Listen and port are ignnored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixPath")]
    pub unix_path: Option<String>,
    /// Set the permission of unix socket file.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixPerm")]
    pub unix_perm: Option<String>,
}

/// HTTP defines the HTTP input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputHttp {
    /// This sets the chunk size for incoming incoming JSON messages. These chunks are then stored/managed in the space available by buffer_max_size,default 512K.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferChunkSize")]
    pub buffer_chunk_size: Option<String>,
    /// Specify the maximum buffer size in KB to receive a JSON message,default 4M.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferMaxSize")]
    pub buffer_max_size: Option<String>,
    /// The address to listen on,default 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// The port for Fluent Bit to listen on,default 9880
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Add an HTTP header key/value pair on success. Multiple headers can be set. Example: X-Custom custom-answer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulHeader")]
    pub successful_header: Option<String>,
    /// It allows to set successful response code. 200, 201 and 204 are supported,default 201.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulResponseCode")]
    pub successful_response_code: Option<i32>,
    /// Specify the key name to overwrite a tag. If set, the tag will be overwritten by a value of the key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagKey")]
    pub tag_key: Option<String>,
    /// Fluent Bit provides integrated support for Transport Layer Security (TLS) and it predecessor Secure Sockets Layer (SSL) respectively.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClusterInputHttpTls>,
}

/// Fluent Bit provides integrated support for Transport Layer Security (TLS) and it predecessor Secure Sockets Layer (SSL) respectively.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputHttpTls {
    /// Absolute path to CA certificate file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caFile")]
    pub ca_file: Option<String>,
    /// Absolute path to scan for certificate files
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caPath")]
    pub ca_path: Option<String>,
    /// Absolute path to Certificate file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crtFile")]
    pub crt_file: Option<String>,
    /// Set TLS debug verbosity level. It accept the following values: 0 (No debug), 1 (Error), 2 (State change), 3 (Informational) and 4 Verbose
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<i32>,
    /// Absolute path to private Key file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyFile")]
    pub key_file: Option<String>,
    /// Optional password for tls.key_file file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyPassword")]
    pub key_password: Option<ClusterInputHttpTlsKeyPassword>,
    /// Force certificate validation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify: Option<bool>,
    /// Hostname to be used for TLS SNI extension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhost: Option<String>,
}

/// Fluent Bit provides integrated support for Transport Layer Security (TLS) and it predecessor Secure Sockets Layer (SSL) respectively.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputHttpTlsDebug {
    #[serde(rename = "0")]
    r#_0,
    #[serde(rename = "1")]
    r#_1,
    #[serde(rename = "2")]
    r#_2,
    #[serde(rename = "3")]
    r#_3,
    #[serde(rename = "4")]
    r#_4,
}

/// Optional password for tls.key_file file
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputHttpTlsKeyPassword {
    /// ValueSource defines how to find a value's key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ClusterInputHttpTlsKeyPasswordValueFrom>,
}

/// ValueSource defines how to find a value's key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputHttpTlsKeyPasswordValueFrom {
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ClusterInputHttpTlsKeyPasswordValueFromSecretKeyRef>,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputHttpTlsKeyPasswordValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// InputSpec defines the desired state of ClusterInput
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputLogLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}

/// MQTT defines the MQTT input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputMqtt {
    /// Listener network interface, default: 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// TCP port where listening for connections, default: 1883
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// Nginx defines the Nginx input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputNginx {
    /// Name of the target host or IP address to check, default: localhost
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Turn on NGINX plus mode,default: true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nginxPlus")]
    pub nginx_plus: Option<bool>,
    /// Port of the target nginx service to connect to, default: 80
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The URL of the Stub Status Handler,default: /status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusURL")]
    pub status_url: Option<String>,
}

/// NodeExporterMetrics defines Node Exporter Metrics Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputNodeExporterMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<ClusterInputNodeExporterMetricsPath>,
    /// The rate at which metrics are collected from the host operating system, default is 5 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeInterval")]
    pub scrape_interval: Option<String>,
    /// Tag name associated to all records comming from this plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputNodeExporterMetricsPath {
    /// The mount point used to collect process information and metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procfs: Option<String>,
    /// The path in the filesystem used to collect system metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysfs: Option<String>,
}

/// OpenTelemetry defines the OpenTelemetry input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputOpenTelemetry {
    /// This sets the chunk size for incoming incoming JSON messages. These chunks are then stored/managed in the space available by buffer_max_size(default 512K).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferChunkSize")]
    pub buffer_chunk_size: Option<String>,
    /// Specify the maximum buffer size in KB to receive a JSON message(default 4M).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferMaxSize")]
    pub buffer_max_size: Option<String>,
    /// The address to listen on,default 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// The port for Fluent Bit to listen on.default 4318.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Route trace data as a log message(default false).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawTraces")]
    pub raw_traces: Option<bool>,
    /// It allows to set successful response code. 200, 201 and 204 are supported(default 201).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulResponseCode")]
    pub successful_response_code: Option<i32>,
    /// Specify the key name to overwrite a tag. If set, the tag will be overwritten by a value of the key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagKey")]
    pub tag_key: Option<String>,
}

/// PrometheusScrapeMetrics  defines Prometheus Scrape Metrics Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputPrometheusScrapeMetrics {
    /// The host of the prometheus metric endpoint that you want to scrape
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The metrics URI endpoint, that must start with a forward slash, deflaut: /metrics
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricsPath")]
    pub metrics_path: Option<String>,
    /// The port of the promethes metric endpoint that you want to scrape
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The interval to scrape metrics, default: 10s
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scrapeInterval")]
    pub scrape_interval: Option<String>,
    /// Tag name associated to all records comming from this plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// StatsD defines the StatsD input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputStatsd {
    /// Listener network interface, default: 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// UDP port where listening for connections, default: 8125
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// Syslog defines the Syslog input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputSyslog {
    /// By default the buffer to store the incoming Syslog messages, do not allocate the maximum memory allowed, instead it allocate memory when is required. The rounds of allocations are set by Buffer_Chunk_Size. If not set, Buffer_Chunk_Size is equal to 32000 bytes (32KB).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferChunkSize")]
    pub buffer_chunk_size: Option<String>,
    /// Specify the maximum buffer size to receive a Syslog message. If not set, the default size will be the value of Buffer_Chunk_Size.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferMaxSize")]
    pub buffer_max_size: Option<String>,
    /// If Mode is set to tcp or udp, specify the network interface to bind, default: 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// Defines transport protocol mode: unix_udp (UDP over Unix socket), unix_tcp (TCP over Unix socket), tcp or udp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ClusterInputSyslogMode>,
    /// Specify an alternative parser for the message. If Mode is set to tcp or udp then the default parser is syslog-rfc5424 otherwise syslog-rfc3164-local is used. If your syslog messages have fractional seconds set this Parser value to syslog-rfc5424 instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parser: Option<String>,
    /// If Mode is set to unix_tcp or unix_udp, set the absolute path to the Unix socket file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// If Mode is set to tcp or udp, specify the TCP port to listen for incoming connections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Specify the maximum socket receive buffer size. If not set, the default value is OS-dependant, but generally too low to accept thousands of syslog messages per second without loss on udp or unix_udp sockets. Note that on Linux the value is capped by sysctl net.core.rmem_max.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "receiveBufferSize")]
    pub receive_buffer_size: Option<String>,
    /// Specify the key where the source address will be injected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceAddressKey")]
    pub source_address_key: Option<String>,
    /// If Mode is set to unix_tcp or unix_udp, set the permission of the Unix socket file, default: 0644
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixPerm")]
    pub unix_perm: Option<i32>,
}

/// Syslog defines the Syslog input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSyslogMode {
    #[serde(rename = "unix_udp")]
    UnixUdp,
    #[serde(rename = "unix_tcp")]
    UnixTcp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputSystemd {
    /// Specify the database file to keep track of monitored files and offsets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<String>,
    /// Set a default synchronization (I/O) method. values: Extra, Full, Normal, Off. This flag affects how the internal SQLite engine do synchronization to disk, for more details about each option please refer to this section. note: this option was introduced on Fluent Bit v1.4.6.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSync")]
    pub db_sync: Option<ClusterInputSystemdDbSync>,
    /// When Fluent Bit starts, the Journal might have a high number of logs in the queue. In order to avoid delays and reduce memory usage, this option allows to specify the maximum number of log entries that can be processed per round. Once the limit is reached, Fluent Bit will continue processing the remaining log entries once Journald performs the notification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxEntries")]
    pub max_entries: Option<i64>,
    /// Set a maximum number of fields (keys) allowed per record.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxFields")]
    pub max_fields: Option<i64>,
    /// Optional path to the Systemd journal directory, if not set, the plugin will use default paths to read local-only logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Specifies if the input plugin should be paused (stop ingesting new data) when the storage.max_chunks_up value is reached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pauseOnChunksOverlimit")]
    pub pause_on_chunks_overlimit: Option<ClusterInputSystemdPauseOnChunksOverlimit>,
    /// Start reading new entries. Skip entries already stored in Journald.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readFromTail")]
    pub read_from_tail: Option<ClusterInputSystemdReadFromTail>,
    /// Specify the buffering mechanism to use. It can be memory or filesystem
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<ClusterInputSystemdStorageType>,
    /// Remove the leading underscore of the Journald field (key). For example the Journald field _PID becomes the key PID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stripUnderscores")]
    pub strip_underscores: Option<ClusterInputSystemdStripUnderscores>,
    /// Allows to perform a query over logs that contains a specific Journald key/value pairs, e.g: _SYSTEMD_UNIT=UNIT. The Systemd_Filter option can be specified multiple times in the input section to apply multiple filters as required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemdFilter")]
    pub systemd_filter: Option<Vec<String>>,
    /// Define the filter type when Systemd_Filter is specified multiple times. Allowed values are And and Or. With And a record is matched only when all of the Systemd_Filter have a match. With Or a record is matched when any of the Systemd_Filter has a match.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemdFilterType")]
    pub systemd_filter_type: Option<ClusterInputSystemdSystemdFilterType>,
    /// The tag is used to route messages but on Systemd plugin there is an extra functionality: if the tag includes a star/wildcard, it will be expanded with the Systemd Unit file (e.g: host.* => host.UNIT_NAME).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdDbSync {
    Extra,
    Full,
    Normal,
    Off,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdPauseOnChunksOverlimit {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdReadFromTail {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdStorageType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "memory")]
    Memory,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdStripUnderscores {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Systemd defines Systemd Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputSystemdSystemdFilterType {
    And,
    Or,
}

/// Tail defines Tail Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputTail {
    /// Set the initial buffer size to read files data. This value is used too to increase buffer size. The value must be according to the Unit Size specification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferChunkSize")]
    pub buffer_chunk_size: Option<String>,
    /// Set the limit of the buffer size per monitored file. When a buffer needs to be increased (e.g: very long lines), this value is used to restrict how much the memory buffer can grow. If reading a file exceed this limit, the file is removed from the monitored file list The value must be according to the Unit Size specification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferMaxSize")]
    pub buffer_max_size: Option<String>,
    /// Specify the database file to keep track of monitored files and offsets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<String>,
    /// Set a default synchronization (I/O) method. Values: Extra, Full, Normal, Off.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbSync")]
    pub db_sync: Option<ClusterInputTailDbSync>,
    /// DisableInotifyWatcher will disable inotify and use the file stat watcher instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableInotifyWatcher")]
    pub disable_inotify_watcher: Option<bool>,
    /// If enabled, the plugin will recombine split Docker log lines before passing them to any parser as configured above. This mode cannot be used at the same time as Multiline.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dockerMode")]
    pub docker_mode: Option<bool>,
    /// Wait period time in seconds to flush queued unfinished split lines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dockerModeFlushSeconds")]
    pub docker_mode_flush_seconds: Option<i64>,
    /// Specify an optional parser for the first line of the docker multiline mode. The parser name to be specified must be registered in the parsers.conf file.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dockerModeParser")]
    pub docker_mode_parser: Option<String>,
    /// Set one or multiple shell patterns separated by commas to exclude files matching a certain criteria, e.g: exclude_path=*.gz,*.zip
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludePath")]
    pub exclude_path: Option<String>,
    /// Ignores records which are older than this time in seconds. Supports m,h,d (minutes, hours, days) syntax. Default behavior is to read all records from specified files. Only available when a Parser is specificied and it can parse the time of a record.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoredOlder")]
    pub ignored_older: Option<String>,
    /// When a message is unstructured (no parser applied), it's appended as a string under the key name log. This option allows to define an alternative name for that key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Set a limit of memory that Tail plugin can use when appending data to the Engine. If the limit is reach, it will be paused; when the data is flushed it resumes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memBufLimit")]
    pub mem_buf_limit: Option<String>,
    /// If enabled, the plugin will try to discover multiline messages and use the proper parsers to compose the outgoing messages. Note that when this option is enabled the Parser option is not used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    /// Wait period time in seconds to process queued multiline messages
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multilineFlushSeconds")]
    pub multiline_flush_seconds: Option<i64>,
    /// This will help to reassembly multiline messages originally split by Docker or CRI Specify one or Multiline Parser definition to apply to the content.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multilineParser")]
    pub multiline_parser: Option<String>,
    /// Specify the name of a parser to interpret the entry as a structured message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parser: Option<String>,
    /// Name of the parser that matchs the beginning of a multiline message. Note that the regular expression defined in the parser must include a group name (named capture)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parserFirstline")]
    pub parser_firstline: Option<String>,
    /// Optional-extra parser to interpret and structure multiline entries. This option can be used to define multiple parsers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parserN")]
    pub parser_n: Option<Vec<String>>,
    /// Pattern specifying a specific log files or multiple ones through the use of common wildcards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// If enabled, it appends the name of the monitored file as part of the record. The value assigned becomes the key in the map.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pathKey")]
    pub path_key: Option<String>,
    /// Specifies if the input plugin should be paused (stop ingesting new data) when the storage.max_chunks_up value is reached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pauseOnChunksOverlimit")]
    pub pause_on_chunks_overlimit: Option<ClusterInputTailPauseOnChunksOverlimit>,
    /// For new discovered files on start (without a database offset/position), read the content from the head of the file, not tail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readFromHead")]
    pub read_from_head: Option<bool>,
    /// The interval of refreshing the list of watched files in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshIntervalSeconds")]
    pub refresh_interval_seconds: Option<i64>,
    /// Specify the number of extra time in seconds to monitor a file once is rotated in case some pending data is flushed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rotateWaitSeconds")]
    pub rotate_wait_seconds: Option<i64>,
    /// When a monitored file reach it buffer capacity due to a very long line (Buffer_Max_Size), the default behavior is to stop monitoring that file. Skip_Long_Lines alter that behavior and instruct Fluent Bit to skip long lines and continue processing other lines that fits into the buffer size.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipLongLines")]
    pub skip_long_lines: Option<bool>,
    /// Specify the buffering mechanism to use. It can be memory or filesystem
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<ClusterInputTailStorageType>,
    /// Set a tag (with regex-extract fields) that will be placed on lines read. E.g. kube.<namespace_name>.<pod_name>.<container_name>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Set a regex to exctract fields from the file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagRegex")]
    pub tag_regex: Option<String>,
}

/// Tail defines Tail Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputTailDbSync {
    Extra,
    Full,
    Normal,
    Off,
}

/// Tail defines Tail Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputTailPauseOnChunksOverlimit {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Tail defines Tail Input configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterInputTailStorageType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "memory")]
    Memory,
}

/// TCP defines the TCP input plugin configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInputTcp {
    /// Specify the maximum buffer size in KB to receive a JSON message. If not set, the default size will be the value of Chunk_Size.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bufferSize")]
    pub buffer_size: Option<String>,
    /// By default the buffer to store the incoming JSON messages, do not allocate the maximum memory allowed, instead it allocate memory when is required. The rounds of allocations are set by Chunk_Size in KB. If not set, Chunk_Size is equal to 32 (32KB).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chunkSize")]
    pub chunk_size: Option<String>,
    /// Specify the expected payload format. It support the options json and none. When using json, it expects JSON maps, when is set to none, it will split every record using the defined Separator (option below).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Listener network interface,default 0.0.0.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    /// TCP port where listening for connections,default 5170
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// When the expected Format is set to none, Fluent Bit needs a separator string to split the records. By default it uses the breakline character (LF or 0x10).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

