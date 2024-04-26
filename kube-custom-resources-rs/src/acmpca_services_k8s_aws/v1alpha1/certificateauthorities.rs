// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/acmpca-controller/acmpca.services.k8s.aws/v1alpha1/certificateauthorities.yaml --derive=Default --derive=PartialEq
// kopium version: 0.18.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// CertificateAuthoritySpec defines the desired state of CertificateAuthority.
/// 
/// 
/// Contains information about your private certificate authority (CA). Your
/// private CA can issue and revoke X.509 digital certificates. Digital certificates
/// verify that the entity named in the certificate Subject field owns or controls
/// the public key contained in the Subject Public Key Info field. Call the CreateCertificateAuthority
/// (https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html)
/// action to create your private CA. You must then call the GetCertificateAuthorityCertificate
/// (https://docs.aws.amazon.com/privateca/latest/APIReference/API_GetCertificateAuthorityCertificate.html)
/// action to retrieve a private CA certificate signing request (CSR). Sign the
/// CSR with your Amazon Web Services Private CA-hosted or on-premises root or
/// subordinate CA certificate. Call the ImportCertificateAuthorityCertificate
/// (https://docs.aws.amazon.com/privateca/latest/APIReference/API_ImportCertificateAuthorityCertificate.html)
/// action to import the signed certificate into Certificate Manager (ACM).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "acmpca.services.k8s.aws", version = "v1alpha1", kind = "CertificateAuthority", plural = "certificateauthorities")]
#[kube(namespaced)]
#[kube(status = "CertificateAuthorityStatus")]
#[kube(schema = "disabled")]
pub struct CertificateAuthoritySpec {
    /// Name and bit size of the private key algorithm, the name of the signing algorithm,
    /// and X.500 certificate subject information.
    #[serde(rename = "certificateAuthorityConfiguration")]
    pub certificate_authority_configuration: CertificateAuthorityCertificateAuthorityConfiguration,
    /// Specifies a cryptographic key management compliance standard used for handling
    /// CA keys.
    /// 
    /// 
    /// Default: FIPS_140_2_LEVEL_3_OR_HIGHER
    /// 
    /// 
    /// Some Amazon Web Services Regions do not support the default. When creating
    /// a CA in these Regions, you must provide FIPS_140_2_LEVEL_2_OR_HIGHER as the
    /// argument for KeyStorageSecurityStandard. Failure to do this results in an
    /// InvalidArgsException with the message, "A certificate authority cannot be
    /// created in this region with the specified security standard."
    /// 
    /// 
    /// For information about security standard support in various Regions, see Storage
    /// and security compliance of Amazon Web Services Private CA private keys (https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyStorageSecurityStandard")]
    pub key_storage_security_standard: Option<String>,
    /// Contains information to enable Online Certificate Status Protocol (OCSP)
    /// support, to enable a certificate revocation list (CRL), to enable both, or
    /// to enable neither. The default is for both certificate validation mechanisms
    /// to be disabled.
    /// 
    /// 
    /// The following requirements apply to revocation configurations.
    /// 
    /// 
    ///    * A configuration disabling CRLs or OCSP must contain only the Enabled=False
    ///    parameter, and will fail if other parameters such as CustomCname or ExpirationInDays
    ///    are included.
    /// 
    /// 
    ///    * In a CRL configuration, the S3BucketName parameter must conform to Amazon
    ///    S3 bucket naming rules (https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
    /// 
    /// 
    ///    * A configuration containing a custom Canonical Name (CNAME) parameter
    ///    for CRLs or OCSP must conform to RFC2396 (https://www.ietf.org/rfc/rfc2396.txt)
    ///    restrictions on the use of special characters in a CNAME.
    /// 
    /// 
    ///    * In a CRL or OCSP configuration, the value of a CNAME parameter must
    ///    not include a protocol prefix such as "http://" or "https://".
    /// 
    /// 
    /// For more information, see the OcspConfiguration (https://docs.aws.amazon.com/privateca/latest/APIReference/API_OcspConfiguration.html)
    /// and CrlConfiguration (https://docs.aws.amazon.com/privateca/latest/APIReference/API_CrlConfiguration.html)
    /// types.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revocationConfiguration")]
    pub revocation_configuration: Option<CertificateAuthorityRevocationConfiguration>,
    /// Key-value pairs that will be attached to the new private CA. You can associate
    /// up to 50 tags with a private CA. For information using tags with IAM to manage
    /// permissions, see Controlling Access Using IAM Tags (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CertificateAuthorityTags>>,
    /// The type of the certificate authority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Specifies whether the CA issues general-purpose certificates that typically
    /// require a revocation mechanism, or short-lived certificates that may optionally
    /// omit revocation because they expire quickly. Short-lived certificate validity
    /// is limited to seven days.
    /// 
    /// 
    /// The default value is GENERAL_PURPOSE.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usageMode")]
    pub usage_mode: Option<String>,
}

/// Name and bit size of the private key algorithm, the name of the signing algorithm,
/// and X.500 certificate subject information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfiguration {
    /// Describes the certificate extensions to be added to the certificate signing
    /// request (CSR).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "csrExtensions")]
    pub csr_extensions: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyAlgorithm")]
    pub key_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingAlgorithm")]
    pub signing_algorithm: Option<String>,
    /// Contains information about the certificate subject. The Subject field in
    /// the certificate identifies the entity that owns or controls the public key
    /// in the certificate. The entity can be a user, computer, device, or service.
    /// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
    /// of relative distinguished names (RDNs). The RDNs are separated by commas
    /// in the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CertificateAuthorityCertificateAuthorityConfigurationSubject>,
}

/// Describes the certificate extensions to be added to the certificate signing
/// request (CSR).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensions {
    /// Defines one or more purposes for which the key contained in the certificate
    /// can be used. Default value for each option is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyUsage")]
    pub key_usage: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsKeyUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectInformationAccess")]
    pub subject_information_access: Option<Vec<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccess>>,
}

/// Defines one or more purposes for which the key contained in the certificate
/// can be used. Default value for each option is false.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsKeyUsage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crlSign")]
    pub crl_sign: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEncipherment")]
    pub data_encipherment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decipherOnly")]
    pub decipher_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "digitalSignature")]
    pub digital_signature: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encipherOnly")]
    pub encipher_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyAgreement")]
    pub key_agreement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyCertSign")]
    pub key_cert_sign: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyEncipherment")]
    pub key_encipherment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonRepudiation")]
    pub non_repudiation: Option<bool>,
}

/// Provides access information used by the authorityInfoAccess and subjectInfoAccess
/// extensions described in RFC 5280 (https://datatracker.ietf.org/doc/html/rfc5280).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccess {
    /// Describes an ASN.1 X.400 GeneralName as defined in RFC 5280 (https://datatracker.ietf.org/doc/html/rfc5280).
    /// Only one of the following naming options should be provided. Providing more
    /// than one option results in an InvalidArgsException error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessLocation")]
    pub access_location: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocation>,
    /// Describes the type and format of extension access. Only one of CustomObjectIdentifier
    /// or AccessMethodType may be provided. Providing both results in InvalidArgsException.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMethod")]
    pub access_method: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessMethod>,
}

/// Describes an ASN.1 X.400 GeneralName as defined in RFC 5280 (https://datatracker.ietf.org/doc/html/rfc5280).
/// Only one of the following naming options should be provided. Providing more
/// than one option results in an InvalidArgsException error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocation {
    /// Contains information about the certificate subject. The Subject field in
    /// the certificate identifies the entity that owns or controls the public key
    /// in the certificate. The entity can be a user, computer, device, or service.
    /// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
    /// of relative distinguished names (RDNs). The RDNs are separated by commas
    /// in the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directoryName")]
    pub directory_name: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationDirectoryName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Describes an Electronic Data Interchange (EDI) entity as described in as
    /// defined in Subject Alternative Name (https://datatracker.ietf.org/doc/html/rfc5280)
    /// in RFC 5280.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ediPartyName")]
    pub edi_party_name: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationEdiPartyName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddress")]
    pub ip_address: Option<String>,
    /// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID)
    /// and value. The OID must satisfy the regular expression shown below. For more
    /// information, see NIST's definition of Object Identifier (OID) (https://csrc.nist.gov/glossary/term/Object_Identifier).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherName")]
    pub other_name: Option<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationOtherName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registeredID")]
    pub registered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rfc822Name")]
    pub rfc822_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uniformResourceIdentifier")]
    pub uniform_resource_identifier: Option<String>,
}

/// Contains information about the certificate subject. The Subject field in
/// the certificate identifies the entity that owns or controls the public key
/// in the certificate. The entity can be a user, computer, device, or service.
/// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
/// of relative distinguished names (RDNs). The RDNs are separated by commas
/// in the certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationDirectoryName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customAttributes")]
    pub custom_attributes: Option<Vec<CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationDirectoryNameCustomAttributes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "distinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "givenName")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "organizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Defines the X.500 relative distinguished name (RDN).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationDirectoryNameCustomAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectIdentifier")]
    pub object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Describes an Electronic Data Interchange (EDI) entity as described in as
/// defined in Subject Alternative Name (https://datatracker.ietf.org/doc/html/rfc5280)
/// in RFC 5280.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationEdiPartyName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameAssigner")]
    pub name_assigner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partyName")]
    pub party_name: Option<String>,
}

/// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID)
/// and value. The OID must satisfy the regular expression shown below. For more
/// information, see NIST's definition of Object Identifier (OID) (https://csrc.nist.gov/glossary/term/Object_Identifier).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessLocationOtherName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typeID")]
    pub type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Describes the type and format of extension access. Only one of CustomObjectIdentifier
/// or AccessMethodType may be provided. Providing both results in InvalidArgsException.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationCsrExtensionsSubjectInformationAccessAccessMethod {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMethodType")]
    pub access_method_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customObjectIdentifier")]
    pub custom_object_identifier: Option<String>,
}

/// Contains information about the certificate subject. The Subject field in
/// the certificate identifies the entity that owns or controls the public key
/// in the certificate. The entity can be a user, computer, device, or service.
/// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
/// of relative distinguished names (RDNs). The RDNs are separated by commas
/// in the certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationSubject {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customAttributes")]
    pub custom_attributes: Option<Vec<CertificateAuthorityCertificateAuthorityConfigurationSubjectCustomAttributes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "distinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "givenName")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "organizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Defines the X.500 relative distinguished name (RDN).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationSubjectCustomAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectIdentifier")]
    pub object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Contains information to enable Online Certificate Status Protocol (OCSP)
/// support, to enable a certificate revocation list (CRL), to enable both, or
/// to enable neither. The default is for both certificate validation mechanisms
/// to be disabled.
/// 
/// 
/// The following requirements apply to revocation configurations.
/// 
/// 
///    * A configuration disabling CRLs or OCSP must contain only the Enabled=False
///    parameter, and will fail if other parameters such as CustomCname or ExpirationInDays
///    are included.
/// 
/// 
///    * In a CRL configuration, the S3BucketName parameter must conform to Amazon
///    S3 bucket naming rules (https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html).
/// 
/// 
///    * A configuration containing a custom Canonical Name (CNAME) parameter
///    for CRLs or OCSP must conform to RFC2396 (https://www.ietf.org/rfc/rfc2396.txt)
///    restrictions on the use of special characters in a CNAME.
/// 
/// 
///    * In a CRL or OCSP configuration, the value of a CNAME parameter must
///    not include a protocol prefix such as "http://" or "https://".
/// 
/// 
/// For more information, see the OcspConfiguration (https://docs.aws.amazon.com/privateca/latest/APIReference/API_OcspConfiguration.html)
/// and CrlConfiguration (https://docs.aws.amazon.com/privateca/latest/APIReference/API_CrlConfiguration.html)
/// types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityRevocationConfiguration {
    /// Contains configuration information for a certificate revocation list (CRL).
    /// Your private certificate authority (CA) creates base CRLs. Delta CRLs are
    /// not supported. You can enable CRLs for your new or an existing private CA
    /// by setting the Enabled parameter to true. Your private CA writes CRLs to
    /// an S3 bucket that you specify in the S3BucketName parameter. You can hide
    /// the name of your bucket by specifying a value for the CustomCname parameter.
    /// Your private CA copies the CNAME or the S3 bucket name to the CRL Distribution
    /// Points extension of each certificate it issues. Your S3 bucket policy must
    /// give write permission to Amazon Web Services Private CA.
    /// 
    /// 
    /// Amazon Web Services Private CA assets that are stored in Amazon S3 can be
    /// protected with encryption. For more information, see Encrypting Your CRLs
    /// (https://docs.aws.amazon.com/privateca/latest/userguide/PcaCreateCa.html#crl-encryption).
    /// 
    /// 
    /// Your private CA uses the value in the ExpirationInDays parameter to calculate
    /// the nextUpdate field in the CRL. The CRL is refreshed prior to a certificate's
    /// expiration date or when a certificate is revoked. When a certificate is revoked,
    /// it appears in the CRL until the certificate expires, and then in one additional
    /// CRL after expiration, and it always appears in the audit report.
    /// 
    /// 
    /// A CRL is typically updated approximately 30 minutes after a certificate is
    /// revoked. If for any reason a CRL update fails, Amazon Web Services Private
    /// CA makes further attempts every 15 minutes.
    /// 
    /// 
    /// CRLs contain the following fields:
    /// 
    /// 
    ///    * Version: The current version number defined in RFC 5280 is V2. The integer
    ///    value is 0x1.
    /// 
    /// 
    ///    * Signature Algorithm: The name of the algorithm used to sign the CRL.
    /// 
    /// 
    ///    * Issuer: The X.500 distinguished name of your private CA that issued
    ///    the CRL.
    /// 
    /// 
    ///    * Last Update: The issue date and time of this CRL.
    /// 
    /// 
    ///    * Next Update: The day and time by which the next CRL will be issued.
    /// 
    /// 
    ///    * Revoked Certificates: List of revoked certificates. Each list item contains
    ///    the following information. Serial Number: The serial number, in hexadecimal
    ///    format, of the revoked certificate. Revocation Date: Date and time the
    ///    certificate was revoked. CRL Entry Extensions: Optional extensions for
    ///    the CRL entry. X509v3 CRL Reason Code: Reason the certificate was revoked.
    /// 
    /// 
    ///    * CRL Extensions: Optional extensions for the CRL. X509v3 Authority Key
    ///    Identifier: Identifies the public key associated with the private key
    ///    used to sign the certificate. X509v3 CRL Number:: Decimal sequence number
    ///    for the CRL.
    /// 
    /// 
    ///    * Signature Algorithm: Algorithm used by your private CA to sign the CRL.
    /// 
    /// 
    ///    * Signature Value: Signature computed over the CRL.
    /// 
    /// 
    /// Certificate revocation lists created by Amazon Web Services Private CA are
    /// DER-encoded. You can use the following OpenSSL command to list a CRL.
    /// 
    /// 
    /// openssl crl -inform DER -text -in crl_path -noout
    /// 
    /// 
    /// For more information, see Planning a certificate revocation list (CRL) (https://docs.aws.amazon.com/privateca/latest/userguide/crl-planning.html)
    /// in the Amazon Web Services Private Certificate Authority User Guide
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crlConfiguration")]
    pub crl_configuration: Option<CertificateAuthorityRevocationConfigurationCrlConfiguration>,
    /// Contains information to enable and configure Online Certificate Status Protocol
    /// (OCSP) for validating certificate revocation status.
    /// 
    /// 
    /// When you revoke a certificate, OCSP responses may take up to 60 minutes to
    /// reflect the new status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ocspConfiguration")]
    pub ocsp_configuration: Option<CertificateAuthorityRevocationConfigurationOcspConfiguration>,
}

/// Contains configuration information for a certificate revocation list (CRL).
/// Your private certificate authority (CA) creates base CRLs. Delta CRLs are
/// not supported. You can enable CRLs for your new or an existing private CA
/// by setting the Enabled parameter to true. Your private CA writes CRLs to
/// an S3 bucket that you specify in the S3BucketName parameter. You can hide
/// the name of your bucket by specifying a value for the CustomCname parameter.
/// Your private CA copies the CNAME or the S3 bucket name to the CRL Distribution
/// Points extension of each certificate it issues. Your S3 bucket policy must
/// give write permission to Amazon Web Services Private CA.
/// 
/// 
/// Amazon Web Services Private CA assets that are stored in Amazon S3 can be
/// protected with encryption. For more information, see Encrypting Your CRLs
/// (https://docs.aws.amazon.com/privateca/latest/userguide/PcaCreateCa.html#crl-encryption).
/// 
/// 
/// Your private CA uses the value in the ExpirationInDays parameter to calculate
/// the nextUpdate field in the CRL. The CRL is refreshed prior to a certificate's
/// expiration date or when a certificate is revoked. When a certificate is revoked,
/// it appears in the CRL until the certificate expires, and then in one additional
/// CRL after expiration, and it always appears in the audit report.
/// 
/// 
/// A CRL is typically updated approximately 30 minutes after a certificate is
/// revoked. If for any reason a CRL update fails, Amazon Web Services Private
/// CA makes further attempts every 15 minutes.
/// 
/// 
/// CRLs contain the following fields:
/// 
/// 
///    * Version: The current version number defined in RFC 5280 is V2. The integer
///    value is 0x1.
/// 
/// 
///    * Signature Algorithm: The name of the algorithm used to sign the CRL.
/// 
/// 
///    * Issuer: The X.500 distinguished name of your private CA that issued
///    the CRL.
/// 
/// 
///    * Last Update: The issue date and time of this CRL.
/// 
/// 
///    * Next Update: The day and time by which the next CRL will be issued.
/// 
/// 
///    * Revoked Certificates: List of revoked certificates. Each list item contains
///    the following information. Serial Number: The serial number, in hexadecimal
///    format, of the revoked certificate. Revocation Date: Date and time the
///    certificate was revoked. CRL Entry Extensions: Optional extensions for
///    the CRL entry. X509v3 CRL Reason Code: Reason the certificate was revoked.
/// 
/// 
///    * CRL Extensions: Optional extensions for the CRL. X509v3 Authority Key
///    Identifier: Identifies the public key associated with the private key
///    used to sign the certificate. X509v3 CRL Number:: Decimal sequence number
///    for the CRL.
/// 
/// 
///    * Signature Algorithm: Algorithm used by your private CA to sign the CRL.
/// 
/// 
///    * Signature Value: Signature computed over the CRL.
/// 
/// 
/// Certificate revocation lists created by Amazon Web Services Private CA are
/// DER-encoded. You can use the following OpenSSL command to list a CRL.
/// 
/// 
/// openssl crl -inform DER -text -in crl_path -noout
/// 
/// 
/// For more information, see Planning a certificate revocation list (CRL) (https://docs.aws.amazon.com/privateca/latest/userguide/crl-planning.html)
/// in the Amazon Web Services Private Certificate Authority User Guide
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityRevocationConfigurationCrlConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customCNAME")]
    pub custom_cname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expirationInDays")]
    pub expiration_in_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3BucketName")]
    pub s3_bucket_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3ObjectACL")]
    pub s3_object_acl: Option<String>,
}

/// Contains information to enable and configure Online Certificate Status Protocol
/// (OCSP) for validating certificate revocation status.
/// 
/// 
/// When you revoke a certificate, OCSP responses may take up to 60 minutes to
/// reflect the new status.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityRevocationConfigurationOcspConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ocspCustomCNAME")]
    pub ocsp_custom_cname: Option<String>,
}

/// Tags are labels that you can use to identify and organize your private CAs.
/// Each tag consists of a key and an optional value. You can associate up to
/// 50 tags with a private CA. To add one or more tags to a private CA, call
/// the TagCertificateAuthority (https://docs.aws.amazon.com/privateca/latest/APIReference/API_TagCertificateAuthority.html)
/// action. To remove a tag, call the UntagCertificateAuthority (https://docs.aws.amazon.com/privateca/latest/APIReference/API_UntagCertificateAuthority.html)
/// action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// CertificateAuthorityStatus defines the observed state of CertificateAuthority
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<CertificateAuthorityStatusAckResourceMetadata>,
    /// The base64 PEM-encoded certificate signing request (CSR) for your private
    /// CA certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateSigningRequest")]
    pub certificate_signing_request: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Date and time at which your private CA was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    pub created_at: Option<String>,
    /// Reason the request to create your private CA failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Date and time at which your private CA was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastStateChangeAt")]
    pub last_state_change_at: Option<String>,
    /// Date and time after which your private CA certificate is not valid.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notAfter")]
    pub not_after: Option<String>,
    /// Date and time before which your private CA certificate is not valid.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBefore")]
    pub not_before: Option<String>,
    /// The Amazon Web Services account ID that owns the certificate authority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerAccount")]
    pub owner_account: Option<String>,
    /// The period during which a deleted CA can be restored. For more information,
    /// see the PermanentDeletionTimeInDays parameter of the DeleteCertificateAuthorityRequest
    /// (https://docs.aws.amazon.com/privateca/latest/APIReference/API_DeleteCertificateAuthorityRequest.html)
    /// action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restorableUntil")]
    pub restorable_until: Option<String>,
    /// Serial number of your private CA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Status of your private CA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateAuthorityStatusAckResourceMetadata {
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

