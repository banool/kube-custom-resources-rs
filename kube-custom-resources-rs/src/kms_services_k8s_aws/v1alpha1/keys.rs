// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/kms-controller/kms.services.k8s.aws/v1alpha1/keys.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// KeySpec defines the desired state of Key.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kms.services.k8s.aws", version = "v1alpha1", kind = "Key", plural = "keys")]
#[kube(namespaced)]
#[kube(status = "KeyStatus")]
#[kube(schema = "disabled")]
pub struct KeySpec {
    /// A flag to indicate whether to bypass the key policy lockout safety check. 
    ///  Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately. 
    ///  For more information, refer to the scenario in the Default Key Policy (https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section in the Key Management Service Developer Guide . 
    ///  Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent PutKeyPolicy request on the KMS key. 
    ///  The default value is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bypassPolicyLockoutSafetyCheck")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// Creates the KMS key in the specified custom key store (https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html) and the key material in its associated CloudHSM cluster. To create a KMS key in a custom key store, you must also specify the Origin parameter with a value of AWS_CLOUDHSM. The CloudHSM cluster that is associated with the custom key store must have at least two active HSMs, each in a different Availability Zone in the Region. 
    ///  This parameter is valid only for symmetric encryption KMS keys in a single Region. You cannot create any other type of KMS key in a custom key store. 
    ///  To find the ID of a custom key store, use the DescribeCustomKeyStores operation. 
    ///  The response includes the custom key store ID and the ID of the CloudHSM cluster. 
    ///  This operation is part of the custom key store feature (https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html) feature in KMS, which combines the convenience and extensive integration of KMS with the isolation and control of a single-tenant key store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customKeyStoreID")]
    pub custom_key_store_id: Option<String>,
    /// A description of the KMS key. 
    ///  Use a description that helps you decide whether the KMS key is appropriate for a task. The default value is an empty string (no description). 
    ///  To set or change the description after the key is created, use UpdateKeyDescription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableKeyRotation")]
    pub enable_key_rotation: Option<bool>,
    /// Specifies the type of KMS key to create. The default value, SYMMETRIC_DEFAULT, creates a KMS key with a 256-bit AES-GCM key that is used for encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key that uses SM4 encryption. For help choosing a key spec for your KMS key, see Choosing a KMS key type (https://docs.aws.amazon.com/kms/latest/developerguide/key-types.html#symm-asymm-choose) in the Key Management Service Developer Guide . 
    ///  The KeySpec determines whether the KMS key contains a symmetric key or an asymmetric key pair. It also determines the cryptographic algorithms that the KMS key supports. You can't change the KeySpec after the KMS key is created. To further restrict the algorithms that can be used with the KMS key, use a condition key in its key policy or IAM policy. For more information, see kms:EncryptionAlgorithm (https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-encryption-algorithm), kms:MacAlgorithm (https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-mac-algorithm) or kms:Signing Algorithm (https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-signing-algorithm) in the Key Management Service Developer Guide . 
    ///  Amazon Web Services services that are integrated with KMS (http://aws.amazon.com/kms/features/#AWS_Service_Integration) use symmetric encryption KMS keys to protect your data. These services do not support asymmetric KMS keys or HMAC KMS keys. 
    ///  KMS supports the following key specs for KMS keys: 
    ///  * Symmetric encryption key (default) SYMMETRIC_DEFAULT 
    ///  * HMAC keys (symmetric) HMAC_224 HMAC_256 HMAC_384 HMAC_512 
    ///  * Asymmetric RSA key pairs RSA_2048 RSA_3072 RSA_4096 
    ///  * Asymmetric NIST-recommended elliptic curve key pairs ECC_NIST_P256 (secp256r1) ECC_NIST_P384 (secp384r1) ECC_NIST_P521 (secp521r1) 
    ///  * Other asymmetric elliptic curve key pairs ECC_SECG_P256K1 (secp256k1), commonly used for cryptocurrencies. 
    ///  * SM2 key pairs (China Regions only) SM2
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySpec")]
    pub key_spec: Option<String>,
    /// Determines the cryptographic operations (https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations) for which you can use the KMS key. The default value is ENCRYPT_DECRYPT. This parameter is optional when you are creating a symmetric encryption KMS key; otherwise, it is required. You can't change the KeyUsage value after the KMS key is created. 
    ///  Select only one valid value. 
    ///  * For symmetric encryption KMS keys, omit the parameter or specify ENCRYPT_DECRYPT. 
    ///  * For HMAC KMS keys (symmetric), specify GENERATE_VERIFY_MAC. 
    ///  * For asymmetric KMS keys with RSA key material, specify ENCRYPT_DECRYPT or SIGN_VERIFY. 
    ///  * For asymmetric KMS keys with ECC key material, specify SIGN_VERIFY. 
    ///  * For asymmetric KMS keys with SM2 key material (China Regions only), specify ENCRYPT_DECRYPT or SIGN_VERIFY.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyUsage")]
    pub key_usage: Option<String>,
    /// Creates a multi-Region primary key that you can replicate into other Amazon Web Services Regions. You cannot change this value after you create the KMS key. 
    ///  For a multi-Region key, set this parameter to True. For a single-Region KMS key, omit this parameter or set it to False. The default value is False. 
    ///  This operation supports multi-Region keys, an KMS feature that lets you create multiple interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see Multi-Region keys in KMS (https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html) in the Key Management Service Developer Guide. 
    ///  This value creates a primary key, not a replica. To create a replica key, use the ReplicateKey operation. 
    ///  You can create a multi-Region version of a symmetric encryption KMS key, an HMAC KMS key, an asymmetric KMS key, or a KMS key with imported key material. However, you cannot create a multi-Region key in a custom key store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiRegion")]
    pub multi_region: Option<bool>,
    /// The source of the key material for the KMS key. You cannot change the origin after you create the KMS key. The default is AWS_KMS, which means that KMS creates the key material. 
    ///  To create a KMS key with no key material (for imported key material), set the value to EXTERNAL. For more information about importing key material into KMS, see Importing Key Material (https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html) in the Key Management Service Developer Guide. This value is valid only for symmetric encryption KMS keys. 
    ///  To create a KMS key in an KMS custom key store (https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html) and create its key material in the associated CloudHSM cluster, set this value to AWS_CLOUDHSM. You must also use the CustomKeyStoreId parameter to identify the custom key store. This value is valid only for symmetric encryption KMS keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// The key policy to attach to the KMS key. If you do not specify a key policy, KMS attaches a default key policy to the KMS key. For more information, see Default key policy (https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default) in the Key Management Service Developer Guide. 
    ///  If you provide a key policy, it must meet the following criteria: 
    ///  * If you don't set BypassPolicyLockoutSafetyCheck to True, the key policy must allow the principal that is making the CreateKey request to make a subsequent PutKeyPolicy request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, refer to the scenario in the Default Key Policy (https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam) section of the Key Management Service Developer Guide . 
    ///  * Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see Changes that I make are not always immediately visible (https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency) in the Amazon Web Services Identity and Access Management User Guide. 
    ///  A key policy document can include only the following characters: 
    ///  * Printable ASCII characters from the space character (\u0020) through the end of the ASCII character range. 
    ///  * Printable characters in the Basic Latin and Latin-1 Supplement character set (through \u00FF). 
    ///  * The tab (\u0009), line feed (\u000A), and carriage return (\u000D) special characters 
    ///  For information about key policies, see Key policies in KMS (https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html) in the Key Management Service Developer Guide. For help writing and formatting a JSON policy document, see the IAM JSON Policy Reference (https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html) in the Identity and Access Management User Guide .
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Assigns one or more tags to the KMS key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the TagResource operation. 
    ///  Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see ABAC in KMS (https://docs.aws.amazon.com/kms/latest/developerguide/abac.html) in the Key Management Service Developer Guide. 
    ///  To use this parameter, you must have kms:TagResource (https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html) permission in an IAM policy. 
    ///  Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one. 
    ///  When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see Tagging Keys (https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<KeyTags>>,
}

/// A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings. 
///  For information about the rules that apply to tag keys and tag values, see User-Defined Tag Restrictions (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html) in the Amazon Web Services Billing and Cost Management User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyTags {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagKey")]
    pub tag_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagValue")]
    pub tag_value: Option<String>,
}

/// KeyStatus defines the observed state of Key
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<KeyStatusAckResourceMetadata>,
    /// The twelve-digit account ID of the Amazon Web Services account that owns the KMS key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsAccountID")]
    pub aws_account_id: Option<String>,
    /// The cluster ID of the CloudHSM cluster that contains the key material for the KMS key. When you create a KMS key in a custom key store (https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html), KMS creates the key material for the KMS key in the associated CloudHSM cluster. This value is present only when the KMS key is created in a custom key store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudHsmClusterID")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<KeyStatusConditions>>,
    /// The date and time when the KMS key was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationDate")]
    pub creation_date: Option<String>,
    /// The date and time after which KMS deletes this KMS key. This value is present only when the KMS key is scheduled for deletion, that is, when its KeyState is PendingDeletion. 
    ///  When the primary key in a multi-Region key is scheduled for deletion but still has replica keys, its key state is PendingReplicaDeletion and the length of its waiting period is displayed in the PendingDeletionWindowInDays field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionDate")]
    pub deletion_date: Option<String>,
    /// Specifies whether the KMS key is enabled. When KeyState is Enabled this value is true, otherwise it is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The encryption algorithms that the KMS key supports. You cannot use the KMS key with other encryption algorithms within KMS. 
    ///  This value is present only when the KeyUsage of the KMS key is ENCRYPT_DECRYPT.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionAlgorithms")]
    pub encryption_algorithms: Option<Vec<String>>,
    /// Specifies whether the KMS key's key material expires. This value is present only when Origin is EXTERNAL, otherwise this value is omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expirationModel")]
    pub expiration_model: Option<String>,
    /// The globally unique identifier for the KMS key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    /// The manager of the KMS key. KMS keys in your Amazon Web Services account are either customer managed or Amazon Web Services managed. For more information about the difference, see KMS keys (https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#kms_keys) in the Key Management Service Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyManager")]
    pub key_manager: Option<String>,
    /// The current status of the KMS key. 
    ///  For more information about how key state affects the use of a KMS key, see Key states of KMS keys (https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html) in the Key Management Service Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyState")]
    pub key_state: Option<String>,
    /// The message authentication code (MAC) algorithm that the HMAC KMS key supports. 
    ///  This value is present only when the KeyUsage of the KMS key is GENERATE_VERIFY_MAC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "macAlgorithms")]
    pub mac_algorithms: Option<Vec<String>>,
    /// Lists the primary and replica keys in same multi-Region key. This field is present only when the value of the MultiRegion field is True. 
    ///  For more information about any listed KMS key, use the DescribeKey operation. 
    ///  * MultiRegionKeyType indicates whether the KMS key is a PRIMARY or REPLICA key. 
    ///  * PrimaryKey displays the key ARN and Region of the primary key. This field displays the current KMS key if it is the primary key. 
    ///  * ReplicaKeys displays the key ARNs and Regions of all replica keys. This field includes the current KMS key if it is a replica key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiRegionConfiguration")]
    pub multi_region_configuration: Option<KeyStatusMultiRegionConfiguration>,
    /// The waiting period before the primary key in a multi-Region key is deleted. This waiting period begins when the last of its replica keys is deleted. This value is present only when the KeyState of the KMS key is PendingReplicaDeletion. That indicates that the KMS key is the primary key in a multi-Region key, it is scheduled for deletion, and it still has existing replica keys. 
    ///  When a single-Region KMS key or a multi-Region replica key is scheduled for deletion, its deletion date is displayed in the DeletionDate field. However, when the primary key in a multi-Region key is scheduled for deletion, its waiting period doesn't begin until all of its replica keys are deleted. This value displays that waiting period. When the last replica key in the multi-Region key is deleted, the KeyState of the scheduled primary key changes from PendingReplicaDeletion to PendingDeletion and the deletion date appears in the DeletionDate field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingDeletionWindowInDays")]
    pub pending_deletion_window_in_days: Option<i64>,
    /// The signing algorithms that the KMS key supports. You cannot use the KMS key with other signing algorithms within KMS. 
    ///  This field appears only when the KeyUsage of the KMS key is SIGN_VERIFY.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingAlgorithms")]
    pub signing_algorithms: Option<Vec<String>>,
    /// The time at which the imported key material expires. When the key material expires, KMS deletes the key material and the KMS key becomes unusable. This value is present only for KMS keys whose Origin is EXTERNAL and whose ExpirationModel is KEY_MATERIAL_EXPIRES, otherwise this value is omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validTo")]
    pub valid_to: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a globally-unique identifier and is set only by the ACK service controller once the controller has orchestrated the creation of the resource OR when it has verified that an "adopted" resource (a resource where the ARN annotation was set by the Kubernetes user on the CR) exists and matches the supplied CR's Spec field values. TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Lists the primary and replica keys in same multi-Region key. This field is present only when the value of the MultiRegion field is True. 
///  For more information about any listed KMS key, use the DescribeKey operation. 
///  * MultiRegionKeyType indicates whether the KMS key is a PRIMARY or REPLICA key. 
///  * PrimaryKey displays the key ARN and Region of the primary key. This field displays the current KMS key if it is the primary key. 
///  * ReplicaKeys displays the key ARNs and Regions of all replica keys. This field includes the current KMS key if it is a replica key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatusMultiRegionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiRegionKeyType")]
    pub multi_region_key_type: Option<String>,
    /// Describes the primary or replica key in a multi-Region key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryKey")]
    pub primary_key: Option<KeyStatusMultiRegionConfigurationPrimaryKey>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaKeys")]
    pub replica_keys: Option<Vec<KeyStatusMultiRegionConfigurationReplicaKeys>>,
}

/// Describes the primary or replica key in a multi-Region key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatusMultiRegionConfigurationPrimaryKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Describes the primary or replica key in a multi-Region key.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyStatusMultiRegionConfigurationReplicaKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

