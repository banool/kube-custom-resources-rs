// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/nginx-kubernetes-gateway/gateway.nginx.org/v1alpha1/clientsettingspolicies.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec defines the desired state of the ClientSettingsPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "gateway.nginx.org", version = "v1alpha1", kind = "ClientSettingsPolicy", plural = "clientsettingspolicies")]
#[kube(namespaced)]
#[kube(status = "ClientSettingsPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClientSettingsPolicySpec {
    /// Body defines the client request body settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<ClientSettingsPolicyBody>,
    /// KeepAlive defines the keep-alive settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<ClientSettingsPolicyKeepAlive>,
    /// TargetRef identifies an API object to apply the policy to.
    /// Object must be in the same namespace as the policy.
    /// Support: Gateway, HTTPRoute, GRPCRoute.
    #[serde(rename = "targetRef")]
    pub target_ref: ClientSettingsPolicyTargetRef,
}

/// Body defines the client request body settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyBody {
    /// MaxSize sets the maximum allowed size of the client request body.
    /// If the size in a request exceeds the configured value,
    /// the 413 (Request Entity Too Large) error is returned to the client.
    /// Setting size to 0 disables checking of client request body size.
    /// Default: https://nginx.org/en/docs/http/ngx_http_core_module.html#client_max_body_size.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<String>,
    /// Timeout defines a timeout for reading client request body. The timeout is set only for a period between
    /// two successive read operations, not for the transmission of the whole request body.
    /// If a client does not transmit anything within this time, the request is terminated with the
    /// 408 (Request Time-out) error.
    /// Default: https://nginx.org/en/docs/http/ngx_http_core_module.html#client_body_timeout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// KeepAlive defines the keep-alive settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyKeepAlive {
    /// Requests sets the maximum number of requests that can be served through one keep-alive connection.
    /// After the maximum number of requests are made, the connection is closed. Closing connections periodically
    /// is necessary to free per-connection memory allocations. Therefore, using too high maximum number of requests
    /// is not recommended as it can lead to excessive memory usage.
    /// Default: https://nginx.org/en/docs/http/ngx_http_core_module.html#keepalive_requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<i32>,
    /// Time defines the maximum time during which requests can be processed through one keep-alive connection.
    /// After this time is reached, the connection is closed following the subsequent request processing.
    /// Default: https://nginx.org/en/docs/http/ngx_http_core_module.html#keepalive_time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Timeout defines the keep-alive timeouts for clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ClientSettingsPolicyKeepAliveTimeout>,
}

/// Timeout defines the keep-alive timeouts for clients.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyKeepAliveTimeout {
    /// Header sets the timeout in the "Keep-Alive: timeout=time" response header field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// Server sets the timeout during which a keep-alive client connection will stay open on the server side.
    /// Setting this value to 0 disables keep-alive client connections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// TargetRef identifies an API object to apply the policy to.
/// Object must be in the same namespace as the policy.
/// Support: Gateway, HTTPRoute, GRPCRoute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyTargetRef {
    /// Group is the group of the target resource.
    pub group: String,
    /// Kind is kind of the target resource.
    pub kind: String,
    /// Name is the name of the target resource.
    pub name: String,
}

/// Status defines the state of the ClientSettingsPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyStatus {
    /// Ancestors is a list of ancestor resources (usually Gateways) that are
    /// associated with the policy, and the status of the policy with respect to
    /// each ancestor. When this policy attaches to a parent, the controller that
    /// manages the parent and the ancestors MUST add an entry to this list when
    /// the controller first sees the policy and SHOULD update the entry as
    /// appropriate when the relevant ancestor is modified.
    /// 
    /// 
    /// Note that choosing the relevant ancestor is left to the Policy designers;
    /// an important part of Policy design is designing the right object level at
    /// which to namespace this status.
    /// 
    /// 
    /// Note also that implementations MUST ONLY populate ancestor status for
    /// the Ancestor resources they are responsible for. Implementations MUST
    /// use the ControllerName field to uniquely identify the entries in this list
    /// that they are responsible for.
    /// 
    /// 
    /// Note that to achieve this, the list of PolicyAncestorStatus structs
    /// MUST be treated as a map with a composite key, made up of the AncestorRef
    /// and ControllerName fields combined.
    /// 
    /// 
    /// A maximum of 16 ancestors will be represented in this list. An empty list
    /// means the Policy is not relevant for any ancestors.
    /// 
    /// 
    /// If this slice is full, implementations MUST NOT add further entries.
    /// Instead they MUST consider the policy unimplementable and signal that
    /// on any related resources such as the ancestor that would be referenced
    /// here. For example, if this list was full on BackendTLSPolicy, no
    /// additional Gateways would be able to reference the Service targeted by
    /// the BackendTLSPolicy.
    pub ancestors: Vec<ClientSettingsPolicyStatusAncestors>,
}

/// PolicyAncestorStatus describes the status of a route with respect to an
/// associated Ancestor.
/// 
/// 
/// Ancestors refer to objects that are either the Target of a policy or above it
/// in terms of object hierarchy. For example, if a policy targets a Service, the
/// Policy's Ancestors are, in order, the Service, the HTTPRoute, the Gateway, and
/// the GatewayClass. Almost always, in this hierarchy, the Gateway will be the most
/// useful object to place Policy status on, so we recommend that implementations
/// SHOULD use Gateway as the PolicyAncestorStatus object unless the designers
/// have a _very_ good reason otherwise.
/// 
/// 
/// In the context of policy attachment, the Ancestor is used to distinguish which
/// resource results in a distinct application of this policy. For example, if a policy
/// targets a Service, it may have a distinct result per attached Gateway.
/// 
/// 
/// Policies targeting the same resource may have different effects depending on the
/// ancestors of those resources. For example, different Gateways targeting the same
/// Service may have different capabilities, especially if they have different underlying
/// implementations.
/// 
/// 
/// For example, in BackendTLSPolicy, the Policy attaches to a Service that is
/// used as a backend in a HTTPRoute that is itself attached to a Gateway.
/// In this case, the relevant object for status is the Gateway, and that is the
/// ancestor object referred to in this status.
/// 
/// 
/// Note that a parent is also an ancestor, so for objects where the parent is the
/// relevant object for status, this struct SHOULD still be used.
/// 
/// 
/// This struct is intended to be used in a slice that's effectively a map,
/// with a composite key made up of the AncestorRef and the ControllerName.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyStatusAncestors {
    /// AncestorRef corresponds with a ParentRef in the spec that this
    /// PolicyAncestorStatus struct describes the status of.
    #[serde(rename = "ancestorRef")]
    pub ancestor_ref: ClientSettingsPolicyStatusAncestorsAncestorRef,
    /// Conditions describes the status of the Policy with respect to the given Ancestor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ControllerName is a domain/path string that indicates the name of the
    /// controller that wrote this status. This corresponds with the
    /// controllerName field on GatewayClass.
    /// 
    /// 
    /// Example: "example.net/gateway-controller".
    /// 
    /// 
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are
    /// valid Kubernetes names
    /// (https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names).
    /// 
    /// 
    /// Controllers MUST populate this field when writing status. Controllers should ensure that
    /// entries to status populated with their ControllerName are cleaned up when they are no
    /// longer necessary.
    #[serde(rename = "controllerName")]
    pub controller_name: String,
}

/// AncestorRef corresponds with a ParentRef in the spec that this
/// PolicyAncestorStatus struct describes the status of.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientSettingsPolicyStatusAncestorsAncestorRef {
    /// Group is the group of the referent.
    /// When unspecified, "gateway.networking.k8s.io" is inferred.
    /// To set the core API group (such as for a "Service" kind referent),
    /// Group must be explicitly set to "" (empty string).
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent.
    /// 
    /// 
    /// There are two kinds of parent resources with "Core" support:
    /// 
    /// 
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, ClusterIP Services only)
    /// 
    /// 
    /// Support for other resources is Implementation-Specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    /// 
    /// 
    /// Support: Core
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, this refers
    /// to the local namespace of the Route.
    /// 
    /// 
    /// Note that there are specific rules for ParentRefs which cross namespace
    /// boundaries. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example:
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable any other kind of cross-namespace reference.
    /// 
    /// 
    /// <gateway:experimental:description>
    /// ParentRefs from a Route to a Service in the same namespace are "producer"
    /// routes, which apply default routing rules to inbound connections from
    /// any namespace to the Service.
    /// 
    /// 
    /// ParentRefs from a Route to a Service in a different namespace are
    /// "consumer" routes, and these routing rules are only applied to outbound
    /// connections originating from the same namespace as the Route, for which
    /// the intended destination of the connections are a Service targeted as a
    /// ParentRef of the Route.
    /// </gateway:experimental:description>
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is the network port this Route targets. It can be interpreted
    /// differently based on the type of parent resource.
    /// 
    /// 
    /// When the parent resource is a Gateway, this targets all listeners
    /// listening on the specified port that also support this kind of Route(and
    /// select this Route). It's not recommended to set `Port` unless the
    /// networking behaviors specified in a Route must apply to a specific port
    /// as opposed to a listener(s) whose port(s) may be changed. When both Port
    /// and SectionName are specified, the name and port of the selected listener
    /// must match both specified values.
    /// 
    /// 
    /// <gateway:experimental:description>
    /// When the parent resource is a Service, this targets a specific port in the
    /// Service spec. When both Port (experimental) and SectionName are specified,
    /// the name and port of the selected port must match both specified values.
    /// </gateway:experimental:description>
    /// 
    /// 
    /// Implementations MAY choose to support other parent resources.
    /// Implementations supporting other types of parent resources MUST clearly
    /// document how/if Port is interpreted.
    /// 
    /// 
    /// For the purpose of status, an attachment is considered successful as
    /// long as the parent resource accepts it partially. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment
    /// from the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route,
    /// the Route MUST be considered detached from the Gateway.
    /// 
    /// 
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// SectionName is the name of a section within the target resource. In the
    /// following resources, SectionName is interpreted as the following:
    /// 
    /// 
    /// * Gateway: Listener name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values.
    /// * Service: Port name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values.
    /// 
    /// 
    /// Implementations MAY choose to support attaching Routes to other resources.
    /// If that is the case, they MUST clearly document how SectionName is
    /// interpreted.
    /// 
    /// 
    /// When unspecified (empty string), this will reference the entire resource.
    /// For the purpose of status, an attachment is considered successful if at
    /// least one section in the parent resource accepts it. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from
    /// the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route, the
    /// Route MUST be considered detached from the Gateway.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

