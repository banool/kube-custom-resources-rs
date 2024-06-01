// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/config.karmada.io/v1alpha1/resourceinterpretercustomizations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec describes the configuration in detail.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "config.karmada.io", version = "v1alpha1", kind = "ResourceInterpreterCustomization", plural = "resourceinterpretercustomizations")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ResourceInterpreterCustomizationSpec {
    /// Customizations describe the interpretation rules.
    pub customizations: ResourceInterpreterCustomizationCustomizations,
    /// CustomizationTarget represents the resource type that the customization applies to.
    pub target: ResourceInterpreterCustomizationTarget,
}

/// Customizations describe the interpretation rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizations {
    /// DependencyInterpretation describes the rules for Karmada to analyze the
    /// dependent resources.
    /// Karmada provides built-in rules for several standard Kubernetes types, see:
    /// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#interpretdependency
    /// If DependencyInterpretation is set, the built-in rules will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dependencyInterpretation")]
    pub dependency_interpretation: Option<ResourceInterpreterCustomizationCustomizationsDependencyInterpretation>,
    /// HealthInterpretation describes the health assessment rules by which Karmada
    /// can assess the health state of the resource type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthInterpretation")]
    pub health_interpretation: Option<ResourceInterpreterCustomizationCustomizationsHealthInterpretation>,
    /// ReplicaResource describes the rules for Karmada to discover the resource's
    /// replica as well as resource requirements.
    /// It would be useful for those CRD resources that declare workload types like
    /// Deployment.
    /// It is usually not needed for Kubernetes native resources(Deployment, Job) as
    /// Karmada knows how to discover info from them. But if it is set, the built-in
    /// discovery rules will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaResource")]
    pub replica_resource: Option<ResourceInterpreterCustomizationCustomizationsReplicaResource>,
    /// ReplicaRevision describes the rules for Karmada to revise the resource's replica.
    /// It would be useful for those CRD resources that declare workload types like
    /// Deployment.
    /// It is usually not needed for Kubernetes native resources(Deployment, Job) as
    /// Karmada knows how to revise replicas for them. But if it is set, the built-in
    /// revision rules will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaRevision")]
    pub replica_revision: Option<ResourceInterpreterCustomizationCustomizationsReplicaRevision>,
    /// Retention describes the desired behavior that Karmada should react on
    /// the changes made by member cluster components. This avoids system
    /// running into a meaningless loop that Karmada resource controller and
    /// the member cluster component continually applying opposite values of a field.
    /// For example, the "replicas" of Deployment might be changed by the HPA
    /// controller on member cluster. In this case, Karmada should retain the "replicas"
    /// and not try to change it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<ResourceInterpreterCustomizationCustomizationsRetention>,
    /// StatusAggregation describes the rules for Karmada to aggregate status
    /// collected from member clusters to resource template.
    /// Karmada provides built-in rules for several standard Kubernetes types, see:
    /// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#aggregatestatus
    /// If StatusAggregation is set, the built-in rules will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusAggregation")]
    pub status_aggregation: Option<ResourceInterpreterCustomizationCustomizationsStatusAggregation>,
    /// StatusReflection describes the rules for Karmada to pick the resource's status.
    /// Karmada provides built-in rules for several standard Kubernetes types, see:
    /// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#interpretstatus
    /// If StatusReflection is set, the built-in rules will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusReflection")]
    pub status_reflection: Option<ResourceInterpreterCustomizationCustomizationsStatusReflection>,
}

/// DependencyInterpretation describes the rules for Karmada to analyze the
/// dependent resources.
/// Karmada provides built-in rules for several standard Kubernetes types, see:
/// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#interpretdependency
/// If DependencyInterpretation is set, the built-in rules will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsDependencyInterpretation {
    /// LuaScript holds the Lua script that is used to interpret the dependencies of
    /// a specific resource.
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function GetDependencies(desiredObj)
    ///           dependencies = {}
    ///           if desiredObj.spec.serviceAccountName ~= nil and desiredObj.spec.serviceAccountName ~= "default" then
    ///               dependency = {}
    ///               dependency.apiVersion = "v1"
    ///               dependency.kind = "ServiceAccount"
    ///               dependency.name = desiredObj.spec.serviceAccountName
    ///               dependency.namespace = desiredObj.namespace
    ///               dependencies[1] = {}
    ///               dependencies[1] = dependency
    ///           end
    ///           return dependencies
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - desiredObj: the object represents the configuration to be applied
    ///       to the member cluster.
    /// 
    /// 
    /// The returned value should be expressed by a slice of DependentObjectReference.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// HealthInterpretation describes the health assessment rules by which Karmada
/// can assess the health state of the resource type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsHealthInterpretation {
    /// LuaScript holds the Lua script that is used to assess the health state of
    /// a specific resource.
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function InterpretHealth(observedObj)
    ///           if observedObj.status.readyReplicas == observedObj.spec.replicas then
    ///               return true
    ///           end
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - observedObj: the object represents the configuration that is observed
    ///       from a specific member cluster.
    /// 
    /// 
    /// The returned boolean value indicates the health status.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// ReplicaResource describes the rules for Karmada to discover the resource's
/// replica as well as resource requirements.
/// It would be useful for those CRD resources that declare workload types like
/// Deployment.
/// It is usually not needed for Kubernetes native resources(Deployment, Job) as
/// Karmada knows how to discover info from them. But if it is set, the built-in
/// discovery rules will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsReplicaResource {
    /// LuaScript holds the Lua script that is used to discover the resource's
    /// replica as well as resource requirements
    /// 
    /// 
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function GetReplicas(desiredObj)
    ///           replica = desiredObj.spec.replicas
    ///           requirement = {}
    ///           requirement.nodeClaim = {}
    ///           requirement.nodeClaim.nodeSelector = desiredObj.spec.template.spec.nodeSelector
    ///           requirement.nodeClaim.tolerations = desiredObj.spec.template.spec.tolerations
    ///           requirement.resourceRequest = desiredObj.spec.template.spec.containers[1].resources.limits
    ///           return replica, requirement
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - desiredObj: the object represents the configuration to be applied
    ///       to the member cluster.
    /// 
    /// 
    /// The function expects two return values:
    ///   - replica: the declared replica number
    ///   - requirement: the resource required by each replica expressed with a
    ///       ResourceBindingSpec.ReplicaRequirements.
    /// The returned values will be set into a ResourceBinding or ClusterResourceBinding.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// ReplicaRevision describes the rules for Karmada to revise the resource's replica.
/// It would be useful for those CRD resources that declare workload types like
/// Deployment.
/// It is usually not needed for Kubernetes native resources(Deployment, Job) as
/// Karmada knows how to revise replicas for them. But if it is set, the built-in
/// revision rules will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsReplicaRevision {
    /// LuaScript holds the Lua script that is used to revise replicas in the desired specification.
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function ReviseReplica(desiredObj, desiredReplica)
    ///           desiredObj.spec.replicas = desiredReplica
    ///           return desiredObj
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - desiredObj: the object represents the configuration to be applied
    ///       to the member cluster.
    ///   - desiredReplica: the replica number should be applied with.
    /// 
    /// 
    /// The returned object should be a revised configuration which will be
    /// applied to member cluster eventually.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// Retention describes the desired behavior that Karmada should react on
/// the changes made by member cluster components. This avoids system
/// running into a meaningless loop that Karmada resource controller and
/// the member cluster component continually applying opposite values of a field.
/// For example, the "replicas" of Deployment might be changed by the HPA
/// controller on member cluster. In this case, Karmada should retain the "replicas"
/// and not try to change it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsRetention {
    /// LuaScript holds the Lua script that is used to retain runtime values
    /// to the desired specification.
    /// 
    /// 
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function Retain(desiredObj, observedObj)
    ///           desiredObj.spec.fieldFoo = observedObj.spec.fieldFoo
    ///           return desiredObj
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - desiredObj: the object represents the configuration to be applied
    ///       to the member cluster.
    ///   - observedObj: the object represents the configuration that is observed
    ///       from a specific member cluster.
    /// 
    /// 
    /// The returned object should be a retained configuration which will be
    /// applied to member cluster eventually.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// StatusAggregation describes the rules for Karmada to aggregate status
/// collected from member clusters to resource template.
/// Karmada provides built-in rules for several standard Kubernetes types, see:
/// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#aggregatestatus
/// If StatusAggregation is set, the built-in rules will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsStatusAggregation {
    /// LuaScript holds the Lua script that is used to aggregate decentralized statuses
    /// to the desired specification.
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function AggregateStatus(desiredObj, statusItems)
    ///           for i = 1, #statusItems do
    ///               desiredObj.status.readyReplicas = desiredObj.status.readyReplicas + items[i].readyReplicas
    ///           end
    ///           return desiredObj
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - desiredObj: the object represents a resource template.
    ///   - statusItems: the slice of status expressed with AggregatedStatusItem.
    /// 
    /// 
    /// The returned object should be a whole object with status aggregated.
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// StatusReflection describes the rules for Karmada to pick the resource's status.
/// Karmada provides built-in rules for several standard Kubernetes types, see:
/// https://karmada.io/docs/userguide/globalview/customizing-resource-interpreter/#interpretstatus
/// If StatusReflection is set, the built-in rules will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationCustomizationsStatusReflection {
    /// LuaScript holds the Lua script that is used to get the status from the observed specification.
    /// The script should implement a function as follows:
    /// 
    /// 
    /// ```
    ///   luaScript: >
    ///       function ReflectStatus(observedObj)
    ///           status = {}
    ///           status.readyReplicas = observedObj.status.observedObj
    ///           return status
    ///       end
    /// ```
    /// 
    /// 
    /// The content of the LuaScript needs to be a whole function including both
    /// declaration and implementation.
    /// 
    /// 
    /// The parameters will be supplied by the system:
    ///   - observedObj: the object represents the configuration that is observed
    ///       from a specific member cluster.
    /// 
    /// 
    /// The returned status could be the whole status or part of it and will
    /// be set into both Work and ResourceBinding(ClusterResourceBinding).
    #[serde(rename = "luaScript")]
    pub lua_script: String,
}

/// CustomizationTarget represents the resource type that the customization applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceInterpreterCustomizationTarget {
    /// APIVersion represents the API version of the target resource.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind represents the Kind of target resources.
    pub kind: String,
}

