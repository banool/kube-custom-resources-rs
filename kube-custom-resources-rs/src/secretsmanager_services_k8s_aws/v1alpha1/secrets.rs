// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/secretsmanager-controller/secretsmanager.services.k8s.aws/v1alpha1/secrets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SecretSpec defines the desired state of Secret.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "secretsmanager.services.k8s.aws", version = "v1alpha1", kind = "Secret", plural = "secrets")]
#[kube(namespaced)]
#[kube(status = "SecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SecretSpec {
    /// The description of the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies whether to overwrite a secret with the same name in the destination
    /// Region. By default, secrets aren't overwritten.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forceOverwriteReplicaSecret")]
    pub force_overwrite_replica_secret: Option<bool>,
    /// The ARN, key ID, or alias of the KMS key that Secrets Manager uses to encrypt
    /// the secret value in the secret. An alias is always prefixed by alias/, for
    /// example alias/aws/secretsmanager. For more information, see About aliases
    /// (https://docs.aws.amazon.com/kms/latest/developerguide/alias-about.html).
    /// 
    /// 
    /// To use a KMS key in a different account, use the key ARN or the alias ARN.
    /// 
    /// 
    /// If you don't specify this value, then Secrets Manager uses the key aws/secretsmanager.
    /// If that key doesn't yet exist, then Secrets Manager creates it for you automatically
    /// the first time it encrypts the secret value.
    /// 
    /// 
    /// If the secret is in a different Amazon Web Services account from the credentials
    /// calling the API, then you can't use aws/secretsmanager to encrypt the secret,
    /// and you must create and use a customer managed KMS key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The name of the new secret.
    /// 
    /// 
    /// The secret name can contain ASCII letters, numbers, and the following characters:
    /// /_+=.@-
    /// 
    /// 
    /// Do not end your secret name with a hyphen followed by six characters. If
    /// you do so, you risk confusion and unexpected results when searching for a
    /// secret by partial ARN. Secrets Manager automatically adds a hyphen and six
    /// random characters after the secret name at the end of the ARN.
    pub name: String,
    /// A list of Regions and KMS keys to replicate secrets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaRegions")]
    pub replica_regions: Option<Vec<SecretReplicaRegions>>,
    /// The text data to encrypt and store in this new version of the secret. We
    /// recommend you use a JSON structure of key/value pairs for your secret value.
    /// 
    /// 
    /// Either SecretString or SecretBinary must have a value, but not both.
    /// 
    /// 
    /// If you create a secret by using the Secrets Manager console then Secrets
    /// Manager puts the protected secret text in only the SecretString parameter.
    /// The Secrets Manager console stores the information as a JSON structure of
    /// key/value pairs that a Lambda rotation function can parse.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretString")]
    pub secret_string: Option<SecretSecretString>,
    /// A list of tags to attach to the secret. Each tag is a key and value pair
    /// of strings in a JSON text string, for example:
    /// 
    /// 
    /// [{"Key":"CostCenter","Value":"12345"},{"Key":"environment","Value":"production"}]
    /// 
    /// 
    /// Secrets Manager tag key names are case sensitive. A tag with the key "ABC"
    /// is a different tag from one with key "abc".
    /// 
    /// 
    /// If you check tags in permissions policies as part of your security strategy,
    /// then adding or removing a tag can change permissions. If the completion of
    /// this operation would result in you losing your permissions for this secret,
    /// then Secrets Manager blocks the operation and returns an Access Denied error.
    /// For more information, see Control access to secrets using tags (https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html#tag-secrets-abac)
    /// and Limit access to identities with tags that match secrets' tags (https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples.html#auth-and-access_tags2).
    /// 
    /// 
    /// For information about how to format a JSON parameter for the various command
    /// line tool environments, see Using JSON for Parameters (https://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json).
    /// If your command-line tool or SDK requires quotation marks around the parameter,
    /// you should use single quotes to avoid confusion with the double quotes required
    /// in the JSON text.
    /// 
    /// 
    /// For tag quotas and naming restrictions, see Service quotas for Tagging (https://docs.aws.amazon.com/general/latest/gr/arg.html#taged-reference-quotas)
    /// in the Amazon Web Services General Reference guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SecretTags>>,
}

/// A custom type that specifies a Region and the KmsKeyId for a replica secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretReplicaRegions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// The text data to encrypt and store in this new version of the secret. We
/// recommend you use a JSON structure of key/value pairs for your secret value.
/// 
/// 
/// Either SecretString or SecretBinary must have a value, but not both.
/// 
/// 
/// If you create a secret by using the Secrets Manager console then Secrets
/// Manager puts the protected secret text in only the SecretString parameter.
/// The Secrets Manager console stores the information as a JSON structure of
/// key/value pairs that a Lambda rotation function can parse.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretSecretString {
    /// Key is the key within the secret
    pub key: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A structure that contains information about a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// SecretStatus defines the observed state of Secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<SecretStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The ARN of the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of the replicas of this secret and their status:
    /// 
    /// 
    ///    * Failed, which indicates that the replica was not created.
    /// 
    /// 
    ///    * InProgress, which indicates that Secrets Manager is in the process of
    ///    creating the replica.
    /// 
    /// 
    ///    * InSync, which indicates that the replica was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationStatus")]
    pub replication_status: Option<Vec<SecretStatusReplicationStatus>>,
    /// The unique identifier associated with the version of the new secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionID")]
    pub version_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// A replication object consisting of a RegionReplicationStatus object and includes
/// a Region, KMSKeyId, status, and status message.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretStatusReplicationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAccessedDate")]
    pub last_accessed_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

