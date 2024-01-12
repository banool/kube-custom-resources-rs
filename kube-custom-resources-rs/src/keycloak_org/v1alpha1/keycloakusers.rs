// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/keycloak/keycloak-operator/keycloak.org/v1alpha1/keycloakusers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// KeycloakUserSpec defines the desired state of KeycloakUser.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keycloak.org", version = "v1alpha1", kind = "KeycloakUser", plural = "keycloakusers")]
#[kube(namespaced)]
#[kube(status = "KeycloakUserStatus")]
#[kube(schema = "disabled")]
pub struct KeycloakUserSpec {
    /// Selector for looking up KeycloakRealm Custom Resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "realmSelector")]
    pub realm_selector: Option<KeycloakUserRealmSelector>,
    /// Keycloak User REST object.
    pub user: KeycloakUserUser,
}

/// Selector for looking up KeycloakRealm Custom Resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserRealmSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<KeycloakUserRealmSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserRealmSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Keycloak User REST object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserUser {
    /// A set of Attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// A set of Client Roles.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientRoles")]
    pub client_roles: Option<BTreeMap<String, String>>,
    /// A set of Credentials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<KeycloakUserUserCredentials>>,
    /// Email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// True if email has already been verified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailVerified")]
    pub email_verified: Option<bool>,
    /// User enabled flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A set of Federated Identities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "federatedIdentities")]
    pub federated_identities: Option<Vec<KeycloakUserUserFederatedIdentities>>,
    /// First Name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firstName")]
    pub first_name: Option<String>,
    /// A set of Groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// User ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last Name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastName")]
    pub last_name: Option<String>,
    /// A set of Realm Roles.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "realmRoles")]
    pub realm_roles: Option<Vec<String>>,
    /// A set of Required Actions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredActions")]
    pub required_actions: Option<Vec<String>>,
    /// User Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserUserCredentials {
    /// True if this credential object is temporary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    /// Credential Type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Credential Value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserUserFederatedIdentities {
    /// Federated Identity Provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityProvider")]
    pub identity_provider: Option<String>,
    /// Federated Identity User ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userId")]
    pub user_id: Option<String>,
    /// Federated Identity User Name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
}

/// KeycloakUserStatus defines the observed state of KeycloakUser.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakUserStatus {
    /// Human-readable message indicating details about current operator phase or error.
    pub message: String,
    /// Current phase of the operator.
    pub phase: String,
}

