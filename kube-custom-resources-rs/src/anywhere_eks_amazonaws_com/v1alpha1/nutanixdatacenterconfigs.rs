// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/nutanixdatacenterconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// NutanixDatacenterConfigSpec defines the desired state of NutanixDatacenterConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "NutanixDatacenterConfig", plural = "nutanixdatacenterconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NutanixDatacenterConfigSpec {
    /// AdditionalTrustBundle is the optional PEM-encoded certificate bundle for users that configured their Prism Central with certificates from non-publicly trusted CAs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalTrustBundle")]
    pub additional_trust_bundle: Option<String>,
    /// CredentialRef is the reference to the secret name that contains the credentials for the Nutanix Prism Central. The namespace for the secret is assumed to be a constant i.e. eksa-system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialRef")]
    pub credential_ref: Option<NutanixDatacenterConfigCredentialRef>,
    /// Endpoint is the Endpoint of Nutanix Prism Central
    pub endpoint: String,
    /// Insecure is the optional flag to skip TLS verification. Nutanix Prism Central installation by default ships with a self-signed certificate that will fail TLS verification because the certificate is not issued by a public CA and does not have the IP SANs with the Prism Central endpoint. To accommodate the scenario where the user has not changed the default Certificate that ships with Prism Central, we allow the user to skip TLS verification. This is not recommended for production use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Port is the Port of Nutanix Prism Central
    pub port: i64,
}

/// CredentialRef is the reference to the secret name that contains the credentials for the Nutanix Prism Central. The namespace for the secret is assumed to be a constant i.e. eksa-system.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NutanixDatacenterConfigCredentialRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NutanixDatacenterConfigStatus defines the observed state of NutanixDatacenterConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NutanixDatacenterConfigStatus {
}

