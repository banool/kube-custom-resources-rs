// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/userprofiles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// UserProfileSpec defines the desired state of UserProfile.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "UserProfile", plural = "userprofiles")]
#[kube(namespaced)]
#[kube(status = "UserProfileStatus")]
#[kube(schema = "disabled")]
pub struct UserProfileSpec {
    /// The ID of the associated Domain.
    #[serde(rename = "domainID")]
    pub domain_id: String,
    /// A specifier for the type of value specified in SingleSignOnUserValue. Currently, the only supported value is "UserName". If the Domain's AuthMode is IAM Identity Center, this field is required. If the Domain's AuthMode is not IAM Identity Center, this field cannot be specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "singleSignOnUserIdentifier")]
    pub single_sign_on_user_identifier: Option<String>,
    /// The username of the associated Amazon Web Services Single Sign-On User for this UserProfile. If the Domain's AuthMode is IAM Identity Center, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not IAM Identity Center, this field cannot be specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "singleSignOnUserValue")]
    pub single_sign_on_user_value: Option<String>,
    /// Each tag consists of a key and an optional value. Tag keys must be unique per resource. 
    ///  Tags that you specify for the User Profile are also added to all Apps that the User Profile launches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UserProfileTags>>,
    /// A name for the UserProfile. This value is not case sensitive.
    #[serde(rename = "userProfileName")]
    pub user_profile_name: String,
    /// A collection of settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userSettings")]
    pub user_settings: Option<UserProfileUserSettings>,
}

/// A tag object that consists of a key and an optional value, used to manage metadata for SageMaker Amazon Web Services resources. 
///  You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints. For more information on adding tags to SageMaker resources, see AddTags. 
///  For more information on adding metadata to your Amazon Web Services resources with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html). For advice on best practices for managing Amazon Web Services resources with tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A collection of settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionRole")]
    pub execution_role: Option<String>,
    /// The JupyterServer app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<UserProfileUserSettingsJupyterServerAppSettings>,
    /// The KernelGateway app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<UserProfileUserSettingsKernelGatewayAppSettings>,
    /// A collection of settings that configure user interaction with the RStudioServerPro app. RStudioServerProAppSettings cannot be updated. The RStudioServerPro app must be deleted and a new one created to make any changes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rStudioServerProAppSettings")]
    pub r_studio_server_pro_app_settings: Option<UserProfileUserSettingsRStudioServerProAppSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// Specifies options for sharing SageMaker Studio notebooks. These settings are specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when the CreateUserProfile API is called. When SharingSettings is not specified, notebook sharing isn't allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharingSettings")]
    pub sharing_settings: Option<UserProfileUserSettingsSharingSettings>,
    /// The TensorBoard app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tensorBoardAppSettings")]
    pub tensor_board_app_settings: Option<UserProfileUserSettingsTensorBoardAppSettings>,
}

/// The JupyterServer app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsJupyterServerAppSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<UserProfileUserSettingsJupyterServerAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsJupyterServerAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
}

/// The KernelGateway app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsKernelGatewayAppSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customImages")]
    pub custom_images: Option<Vec<UserProfileUserSettingsKernelGatewayAppSettingsCustomImages>>,
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<UserProfileUserSettingsKernelGatewayAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// A custom SageMaker image. For more information, see Bring your own SageMaker image (https://docs.aws.amazon.com/sagemaker/latest/dg/studio-byoi.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsKernelGatewayAppSettingsCustomImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appImageConfigName")]
    pub app_image_config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageName")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageVersionNumber")]
    pub image_version_number: Option<i64>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsKernelGatewayAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
}

/// A collection of settings that configure user interaction with the RStudioServerPro app. RStudioServerProAppSettings cannot be updated. The RStudioServerPro app must be deleted and a new one created to make any changes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsRStudioServerProAppSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessStatus")]
    pub access_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userGroup")]
    pub user_group: Option<String>,
}

/// Specifies options for sharing SageMaker Studio notebooks. These settings are specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when the CreateUserProfile API is called. When SharingSettings is not specified, notebook sharing isn't allowed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsSharingSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notebookOutputOption")]
    pub notebook_output_option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3KMSKeyID")]
    pub s3_kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3OutputPath")]
    pub s3_output_path: Option<String>,
}

/// The TensorBoard app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsTensorBoardAppSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<UserProfileUserSettingsTensorBoardAppSettingsDefaultResourceSpec>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileUserSettingsTensorBoardAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
}

/// UserProfileStatus defines the observed state of UserProfile
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<UserProfileStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<UserProfileStatusConditions>>,
    /// The status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserProfileStatusAckResourceMetadata {
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
pub struct UserProfileStatusConditions {
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

