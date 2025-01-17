//! Types that are expected to be serialised in applications using [crate::Webauthn]

use serde::{Deserialize, Serialize};

use webauthn_rs_core::interface::{AttestationCaList, AuthenticationState, RegistrationState};
use webauthn_rs_core::proto::{COSEAlgorithm, Credential, CredentialID, ParsedAttestationData};

/// An in progress registration session for a [SecurityKey].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityKeyRegistration {
    pub(crate) rs: RegistrationState,
    pub(crate) ca_list: Option<AttestationCaList>,
}

/// An in progress authentication session for a [SecurityKey].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityKeyAuthentication {
    pub(crate) ast: AuthenticationState,
}

/// A Security Key for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityKey {
    pub(crate) cred: Credential,
}

impl SecurityKey {
    /// Retrieve a reference to this Security Key's credential ID.
    pub fn cred_id(&self) -> &CredentialID {
        &self.cred.cred_id
    }

    /// Retrieve the type of cryptographic algorithm used by this key
    pub fn cred_algorithm(&self) -> &COSEAlgorithm {
        &self.cred.cred.type_
    }

    /// Retrieve a reference to the attestation used during this [`Credential`]'s
    /// registration. This can tell you information about the manufacterer and
    /// what type of credential it is.
    pub fn attestation(&self) -> &ParsedAttestationData {
        &self.cred.attestation.data
    }

    /// Post authentication, update this credentials counter.
    pub fn update_credential_counter(&mut self, counter: u32) {
        if counter > self.cred.counter {
            self.cred.counter = counter
        }
    }
}

impl From<Credential> for SecurityKey {
    /// Convert a generic webauthn credential into a security key
    fn from(cred: Credential) -> Self {
        SecurityKey { cred }
    }
}

// PasswordlessKey

/// An in progress registration session for a [PasswordlessKey].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordlessKeyRegistration {
    pub(crate) rs: RegistrationState,
    pub(crate) ca_list: Option<AttestationCaList>,
}

/// An in progress registration session for a [PasswordlessKey].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordlessKeyAuthentication {
    pub(crate) ast: AuthenticationState,
}

/// A passwordless key for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordlessKey {
    pub(crate) cred: Credential,
}

impl PasswordlessKey {
    /// Retrieve a reference to this Passwordless Key's credential ID.
    pub fn cred_id(&self) -> &CredentialID {
        &self.cred.cred_id
    }

    /// Retrieve the type of cryptographic algorithm used by this key
    pub fn cred_algorithm(&self) -> &COSEAlgorithm {
        &self.cred.cred.type_
    }

    /// Retrieve a reference to the attestation used during this [`Credential`]'s
    /// registration. This can tell you information about the manufacterer and
    /// what type of credential it is.
    pub fn attestation(&self) -> &ParsedAttestationData {
        &self.cred.attestation.data
    }

    /// Post authentication, update this credentials counter.
    pub fn update_credential_counter(&mut self, counter: u32) {
        if counter > self.cred.counter {
            self.cred.counter = counter
        }
    }
}
