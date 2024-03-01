// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/transformjobs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// TransformJobSpec defines the desired state of TransformJob.
/// 
/// 
/// A batch transform job. For information about SageMaker batch transform, see
/// Use Batch Transform (https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform.html).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "TransformJob", plural = "transformjobs")]
#[kube(namespaced)]
#[kube(status = "TransformJobStatus")]
#[kube(schema = "disabled")]
pub struct TransformJobSpec {
    /// Specifies the number of records to include in a mini-batch for an HTTP inference
    /// request. A record is a single unit of input data that inference can be made
    /// on. For example, a single line in a CSV file is a record.
    /// 
    /// 
    /// To enable the batch strategy, you must set the SplitType property to Line,
    /// RecordIO, or TFRecord.
    /// 
    /// 
    /// To use only one record when making an HTTP invocation request to a container,
    /// set BatchStrategy to SingleRecord and SplitType to Line.
    /// 
    /// 
    /// To fit as many records in a mini-batch as can fit within the MaxPayloadInMB
    /// limit, set BatchStrategy to MultiRecord and SplitType to Line.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchStrategy")]
    pub batch_strategy: Option<String>,
    /// The data structure used to specify the data to be used for inference in a
    /// batch transform job and to associate the data that is relevant to the prediction
    /// results in the output. The input filter provided allows you to exclude input
    /// data that is not needed for inference in a batch transform job. The output
    /// filter provided allows you to include input data relevant to interpreting
    /// the predictions in the output from the job. For more information, see Associate
    /// Prediction Results with their Corresponding Input Records (https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataProcessing")]
    pub data_processing: Option<TransformJobDataProcessing>,
    /// The environment variables to set in the Docker container. We support up to
    /// 16 key and values entries in the map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    /// Associates a SageMaker job as a trial component with an experiment and trial.
    /// Specified when you call the following APIs:
    /// 
    /// 
    ///    * CreateProcessingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateProcessingJob.html)
    /// 
    /// 
    ///    * CreateTrainingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTrainingJob.html)
    /// 
    /// 
    ///    * CreateTransformJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTransformJob.html)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentConfig")]
    pub experiment_config: Option<TransformJobExperimentConfig>,
    /// The maximum number of parallel requests that can be sent to each instance
    /// in a transform job. If MaxConcurrentTransforms is set to 0 or left unset,
    /// Amazon SageMaker checks the optional execution-parameters to determine the
    /// settings for your chosen algorithm. If the execution-parameters endpoint
    /// is not enabled, the default value is 1. For more information on execution-parameters,
    /// see How Containers Serve Requests (https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-batch-code.html#your-algorithms-batch-code-how-containe-serves-requests).
    /// For built-in algorithms, you don't need to set a value for MaxConcurrentTransforms.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrentTransforms")]
    pub max_concurrent_transforms: Option<i64>,
    /// The maximum allowed size of the payload, in MB. A payload is the data portion
    /// of a record (without metadata). The value in MaxPayloadInMB must be greater
    /// than, or equal to, the size of a single record. To estimate the size of a
    /// record in MB, divide the size of your dataset by the number of records. To
    /// ensure that the records fit within the maximum payload size, we recommend
    /// using a slightly larger value. The default value is 6 MB.
    /// 
    /// 
    /// The value of MaxPayloadInMB cannot be greater than 100 MB. If you specify
    /// the MaxConcurrentTransforms parameter, the value of (MaxConcurrentTransforms
    /// * MaxPayloadInMB) also cannot exceed 100 MB.
    /// 
    /// 
    /// For cases where the payload might be arbitrarily large and is transmitted
    /// using HTTP chunked encoding, set the value to 0. This feature works only
    /// in supported algorithms. Currently, Amazon SageMaker built-in algorithms
    /// do not support HTTP chunked encoding.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPayloadInMB")]
    pub max_payload_in_mb: Option<i64>,
    /// Configures the timeout and maximum number of retries for processing a transform
    /// job invocation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelClientConfig")]
    pub model_client_config: Option<TransformJobModelClientConfig>,
    /// The name of the model that you want to use for the transform job. ModelName
    /// must be the name of an existing Amazon SageMaker model within an Amazon Web
    /// Services Region in an Amazon Web Services account.
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// (Optional) An array of key-value pairs. For more information, see Using Cost
    /// Allocation Tags (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what)
    /// in the Amazon Web Services Billing and Cost Management User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TransformJobTags>>,
    /// Describes the input source and the way the transform job consumes it.
    #[serde(rename = "transformInput")]
    pub transform_input: TransformJobTransformInput,
    /// The name of the transform job. The name must be unique within an Amazon Web
    /// Services Region in an Amazon Web Services account.
    #[serde(rename = "transformJobName")]
    pub transform_job_name: String,
    /// Describes the results of the transform job.
    #[serde(rename = "transformOutput")]
    pub transform_output: TransformJobTransformOutput,
    /// Describes the resources, including ML instance types and ML instance count,
    /// to use for the transform job.
    #[serde(rename = "transformResources")]
    pub transform_resources: TransformJobTransformResources,
}

/// The data structure used to specify the data to be used for inference in a
/// batch transform job and to associate the data that is relevant to the prediction
/// results in the output. The input filter provided allows you to exclude input
/// data that is not needed for inference in a batch transform job. The output
/// filter provided allows you to include input data relevant to interpreting
/// the predictions in the output from the job. For more information, see Associate
/// Prediction Results with their Corresponding Input Records (https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobDataProcessing {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inputFilter")]
    pub input_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "joinSource")]
    pub join_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputFilter")]
    pub output_filter: Option<String>,
}

/// Associates a SageMaker job as a trial component with an experiment and trial.
/// Specified when you call the following APIs:
/// 
/// 
///    * CreateProcessingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateProcessingJob.html)
/// 
/// 
///    * CreateTrainingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTrainingJob.html)
/// 
/// 
///    * CreateTransformJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTransformJob.html)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobExperimentConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentName")]
    pub experiment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trialComponentDisplayName")]
    pub trial_component_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trialName")]
    pub trial_name: Option<String>,
}

/// Configures the timeout and maximum number of retries for processing a transform
/// job invocation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobModelClientConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "invocationsMaxRetries")]
    pub invocations_max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "invocationsTimeoutInSeconds")]
    pub invocations_timeout_in_seconds: Option<i64>,
}

/// A tag object that consists of a key and an optional value, used to manage
/// metadata for SageMaker Amazon Web Services resources.
/// 
/// 
/// You can add tags to notebook instances, training jobs, hyperparameter tuning
/// jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations,
/// and endpoints. For more information on adding tags to SageMaker resources,
/// see AddTags (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AddTags.html).
/// 
/// 
/// For more information on adding metadata to your Amazon Web Services resources
/// with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
/// For advice on best practices for managing Amazon Web Services resources with
/// tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services
/// Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Describes the input source and the way the transform job consumes it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTransformInput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionType")]
    pub compression_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentType")]
    pub content_type: Option<String>,
    /// Describes the location of the channel data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSource")]
    pub data_source: Option<TransformJobTransformInputDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "splitType")]
    pub split_type: Option<String>,
}

/// Describes the location of the channel data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTransformInputDataSource {
    /// Describes the S3 data source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataSource")]
    pub s3_data_source: Option<TransformJobTransformInputDataSourceS3DataSource>,
}

/// Describes the S3 data source.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTransformInputDataSourceS3DataSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataType")]
    pub s3_data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// Describes the results of the transform job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTransformOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "assembleWith")]
    pub assemble_with: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3OutputPath")]
    pub s3_output_path: Option<String>,
}

/// Describes the resources, including ML instance types and ML instance count,
/// to use for the transform job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobTransformResources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceCount")]
    pub instance_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeKMSKeyID")]
    pub volume_kms_key_id: Option<String>,
}

/// TransformJobStatus defines the observed state of TransformJob
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<TransformJobStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TransformJobStatusConditions>>,
    /// If the transform job failed, FailureReason describes why it failed. A transform
    /// job creates a log file, which includes error messages, and stores it as an
    /// Amazon S3 object. For more information, see Log Amazon SageMaker Events with
    /// Amazon CloudWatch (https://docs.aws.amazon.com/sagemaker/latest/dg/logging-cloudwatch.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// The status of the transform job. If the transform job failed, the reason
    /// is returned in the FailureReason field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transformJobStatus")]
    pub transform_job_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobStatusAckResourceMetadata {
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

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransformJobStatusConditions {
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

