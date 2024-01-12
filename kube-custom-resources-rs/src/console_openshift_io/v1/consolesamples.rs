// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/console.openshift.io/v1/consolesamples.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec contains configuration for a console sample.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "console.openshift.io", version = "v1", kind = "ConsoleSample", plural = "consolesamples")]
#[kube(schema = "disabled")]
pub struct ConsoleSampleSpec {
    /// abstract is a short introduction to the sample. 
    ///  It is required and must be no more than 100 characters in length. 
    ///  The abstract is shown on the sample card tile below the title and provider and is limited to three lines of content.
    #[serde(rename = "abstract")]
    pub r#abstract: String,
    /// description is a long form explanation of the sample. 
    ///  It is required and can have a maximum length of **4096** characters. 
    ///  It is a README.md-like content for additional information, links, pre-conditions, and other instructions. It will be rendered as Markdown so that it can contain line breaks, links, and other simple formatting.
    pub description: String,
    /// icon is an optional base64 encoded image and shown beside the sample title. 
    ///  The format must follow the data: URL format and can have a maximum size of **10 KB**. 
    ///  data:[<mediatype>][;base64],<base64 encoded image> 
    ///  For example: 
    ///  data:image;base64,             plus the base64 encoded image. 
    ///  Vector images can also be used. SVG icons must start with: 
    ///  data:image/svg+xml;base64,     plus the base64 encoded SVG image. 
    ///  All sample catalog icons will be shown on a white background (also when the dark theme is used). The web console ensures that different aspect ratios work correctly. Currently, the surface of the icon is at most 40x100px. 
    ///  For more information on the data URL format, please visit https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// provider is an optional label to honor who provides the sample. 
    ///  It is optional and must be no more than 50 characters in length. 
    ///  A provider can be a company like "Red Hat" or an organization like "CNCF" or "Knative". 
    ///  Currently, the provider is only shown on the sample card tile below the title with the prefix "Provided by "
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// source defines where to deploy the sample service from. The sample may be sourced from an external git repository or container image.
    pub source: ConsoleSampleSource,
    /// tags are optional string values that can be used to find samples in the samples catalog. 
    ///  Examples of common tags may be "Java", "Quarkus", etc. 
    ///  They will be displayed on the samples details page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// title is the display name of the sample. 
    ///  It is required and must be no more than 50 characters in length.
    pub title: String,
    /// type is an optional label to group multiple samples. 
    ///  It is optional and must be no more than 20 characters in length. 
    ///  Recommendation is a singular term like "Builder Image", "Devfile" or "Serverless Function". 
    ///  Currently, the type is shown a badge on the sample card tile in the top right corner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// source defines where to deploy the sample service from. The sample may be sourced from an external git repository or container image.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSource {
    /// containerImport allows the user import a container image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerImport")]
    pub container_import: Option<ConsoleSampleSourceContainerImport>,
    /// gitImport allows the user to import code from a git repository.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitImport")]
    pub git_import: Option<ConsoleSampleSourceGitImport>,
    /// type of the sample, currently supported: "GitImport";"ContainerImport"
    #[serde(rename = "type")]
    pub r#type: String,
}

/// containerImport allows the user import a container image.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSourceContainerImport {
    /// reference to a container image that provides a HTTP service. The service must be exposed on the default port (8080) unless otherwise configured with the port field. 
    ///  Supported formats: - <repository-name>/<image-name> - docker.io/<repository-name>/<image-name> - quay.io/<repository-name>/<image-name> - quay.io/<repository-name>/<image-name>@sha256:<image hash> - quay.io/<repository-name>/<image-name>:<tag>
    pub image: String,
    /// service contains configuration for the Service resource created for this sample.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ConsoleSampleSourceContainerImportService>,
}

/// service contains configuration for the Service resource created for this sample.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSourceContainerImportService {
    /// targetPort is the port that the service listens on for HTTP requests. This port will be used for Service and Route created for this sample. Port must be in the range 1 to 65535. Default port is 8080.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i32>,
}

/// gitImport allows the user to import code from a git repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSourceGitImport {
    /// repository contains the reference to the actual Git repository.
    pub repository: ConsoleSampleSourceGitImportRepository,
    /// service contains configuration for the Service resource created for this sample.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ConsoleSampleSourceGitImportService>,
}

/// repository contains the reference to the actual Git repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSourceGitImportRepository {
    /// contextDir is used to specify a directory within the repository to build the component. Must start with `/` and have a maximum length of 256 characters. When omitted, the default value is to build from the root of the repository.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contextDir")]
    pub context_dir: Option<String>,
    /// revision is the git revision at which to clone the git repository Can be used to clone a specific branch, tag or commit SHA. Must be at most 256 characters in length. When omitted the repository's default branch is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// url of the Git repository that contains a HTTP service. The HTTP service must be exposed on the default port (8080) unless otherwise configured with the port field. 
    ///  Only public repositories on GitHub, GitLab and Bitbucket are currently supported: 
    ///  - https://github.com/<org>/<repository> - https://gitlab.com/<org>/<repository> - https://bitbucket.org/<org>/<repository> 
    ///  The url must have a maximum length of 256 characters.
    pub url: String,
}

/// service contains configuration for the Service resource created for this sample.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConsoleSampleSourceGitImportService {
    /// targetPort is the port that the service listens on for HTTP requests. This port will be used for Service created for this sample. Port must be in the range 1 to 65535. Default port is 8080.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i32>,
}

