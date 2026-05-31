use std::collections::BTreeMap;

use sdkwork_aiot_contract::AiotActorRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeviceAuthMode {
    BearerToken,
    Hmac,
    MtlsX509,
    HardwareAttestation,
    BrokerCredential,
    BridgeTrust,
}

impl DeviceAuthMode {
    pub fn standard_modes() -> Vec<Self> {
        vec![
            Self::BearerToken,
            Self::Hmac,
            Self::MtlsX509,
            Self::HardwareAttestation,
            Self::BrokerCredential,
            Self::BridgeTrust,
        ]
    }

    pub fn manifest_name(self) -> &'static str {
        match self {
            Self::BearerToken => "bearer_token",
            Self::Hmac => "hmac",
            Self::MtlsX509 => "mtls_x509",
            Self::HardwareAttestation => "hardware_attestation",
            Self::BrokerCredential => "broker_credential",
            Self::BridgeTrust => "bridge_trust",
        }
    }
}

pub type AuthLevel = DeviceAuthMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevicePrincipal {
    pub tenant_id: String,
    pub organization_id: String,
    pub product_id: String,
    pub device_id: String,
    pub auth_level: DeviceAuthMode,
    pub credential_id: Option<String>,
    pub trusted: bool,
}

impl DevicePrincipal {
    pub fn new(
        tenant_id: impl Into<String>,
        organization_id: impl Into<String>,
        product_id: impl Into<String>,
        device_id: impl Into<String>,
        auth_level: DeviceAuthMode,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            organization_id: organization_id.into(),
            product_id: product_id.into(),
            device_id: device_id.into(),
            auth_level,
            credential_id: None,
            trusted: true,
        }
    }

    pub fn with_credential(mut self, credential_id: impl Into<String>) -> Self {
        self.credential_id = Some(credential_id.into());
        self
    }

    pub fn actor_ref(&self) -> AiotActorRef {
        AiotActorRef::device(self.device_id.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAuthRequest {
    pub protocol_id: String,
    pub device_id: String,
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub product_id: Option<String>,
    pub client_id: Option<String>,
    pub credential_id: Option<String>,
    pub mode: DeviceAuthMode,
    pub evidence: BTreeMap<String, String>,
}

impl DeviceAuthRequest {
    pub fn new(protocol_id: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self {
            protocol_id: protocol_id.into(),
            device_id: device_id.into(),
            tenant_id: None,
            organization_id: None,
            product_id: None,
            client_id: None,
            credential_id: None,
            mode: DeviceAuthMode::BearerToken,
            evidence: BTreeMap::new(),
        }
    }

    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn with_organization(mut self, organization_id: impl Into<String>) -> Self {
        self.organization_id = Some(organization_id.into());
        self
    }

    pub fn with_product(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    pub fn with_client(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn with_credential(mut self, credential_id: impl Into<String>) -> Self {
        self.credential_id = Some(credential_id.into());
        self
    }

    pub fn with_mode(mut self, mode: DeviceAuthMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_evidence(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.evidence.insert(name.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAuthDecision {
    pub allowed: bool,
    pub principal: Option<DevicePrincipal>,
    pub reason_code: Option<String>,
}

impl DeviceAuthDecision {
    pub fn allow(request: DeviceAuthRequest) -> Result<Self, DeviceAuthError> {
        let tenant_id = required(request.tenant_id, "security.device_auth.missing_context")?;
        let organization_id = required(
            request.organization_id,
            "security.device_auth.missing_context",
        )?;
        let product_id = required(request.product_id, "security.device_auth.missing_context")?;

        let mut principal = DevicePrincipal::new(
            tenant_id,
            organization_id,
            product_id,
            request.device_id,
            request.mode,
        );
        if let Some(credential_id) = request.credential_id {
            principal = principal.with_credential(credential_id);
        }

        Ok(Self {
            allowed: true,
            principal: Some(principal),
            reason_code: None,
        })
    }

    pub fn deny(reason_code: impl Into<String>) -> Self {
        Self {
            allowed: false,
            principal: None,
            reason_code: Some(reason_code.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAuthError {
    pub code: String,
}

impl DeviceAuthError {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

fn required(value: Option<String>, code: &'static str) -> Result<String, DeviceAuthError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| DeviceAuthError::new(code))
}
