use sdkwork_aiot_security::{
    DeviceAuthDecision, DeviceAuthMode, DeviceAuthRequest, DevicePrincipal,
};

#[test]
fn device_principal_is_scoped_to_tenant_and_device_not_user_session() {
    let principal = DevicePrincipal::new("t1", "o1", "prod1", "dev1", DeviceAuthMode::Hmac);

    assert_eq!(principal.tenant_id, "t1");
    assert_eq!(principal.organization_id, "o1");
    assert_eq!(principal.product_id, "prod1");
    assert_eq!(principal.device_id, "dev1");
    assert_eq!(principal.auth_level, DeviceAuthMode::Hmac);
}

#[test]
fn device_auth_request_is_protocol_scoped_not_user_session_scoped() {
    let request = DeviceAuthRequest::new("xiaozhi.websocket", "device-001")
        .with_tenant("t1")
        .with_organization("o1")
        .with_product("prod1")
        .with_client("client-abc")
        .with_credential("credential-001")
        .with_mode(DeviceAuthMode::BearerToken)
        .with_evidence("Authorization", "Bearer device-token");

    assert_eq!(request.protocol_id, "xiaozhi.websocket");
    assert_eq!(request.device_id, "device-001");
    assert_eq!(request.tenant_id.as_deref(), Some("t1"));
    assert_eq!(request.organization_id.as_deref(), Some("o1"));
    assert_eq!(request.product_id.as_deref(), Some("prod1"));
    assert_eq!(request.client_id.as_deref(), Some("client-abc"));
    assert_eq!(request.credential_id.as_deref(), Some("credential-001"));
    assert_eq!(request.mode, DeviceAuthMode::BearerToken);
    assert_eq!(
        request.evidence.get("Authorization").map(String::as_str),
        Some("Bearer device-token")
    );
}

#[test]
fn device_auth_decision_builds_principal_without_iam_user_identity() {
    let request = DeviceAuthRequest::new("mqtt.v5", "device-001")
        .with_tenant("t1")
        .with_organization("o1")
        .with_product("prod1")
        .with_credential("credential-001")
        .with_mode(DeviceAuthMode::BrokerCredential);

    let decision = DeviceAuthDecision::allow(request).expect("decision");
    let principal = decision.principal.expect("principal");

    assert!(decision.allowed);
    assert_eq!(principal.tenant_id, "t1");
    assert_eq!(principal.organization_id, "o1");
    assert_eq!(principal.product_id, "prod1");
    assert_eq!(principal.device_id, "device-001");
    assert_eq!(principal.auth_level, DeviceAuthMode::BrokerCredential);
    assert_eq!(principal.credential_id.as_deref(), Some("credential-001"));
    assert_eq!(principal.actor_ref().actor_type, "device");
    assert_eq!(principal.actor_ref().actor_id, "device-001");
}

#[test]
fn device_auth_decision_rejects_missing_association_context() {
    let request = DeviceAuthRequest::new("xiaozhi.websocket", "device-001")
        .with_mode(DeviceAuthMode::BearerToken);

    let error = DeviceAuthDecision::allow(request).expect_err("missing tenant must fail");

    assert_eq!(error.code, "security.device_auth.missing_context");
}

#[test]
fn device_auth_decision_rejects_bearer_token_mismatch() {
    let request = DeviceAuthRequest::new("xiaozhi.websocket", "device-001")
        .with_tenant("t1")
        .with_organization("o1")
        .with_product("prod1")
        .with_mode(DeviceAuthMode::BearerToken)
        .with_evidence("Authorization", "Bearer device-token")
        .with_evidence("token", "other-token");

    let error = DeviceAuthDecision::allow(request).expect_err("bearer mismatch must fail");
    assert_eq!(error.code, "security.device_auth.bearer_mismatch");
}

#[test]
fn standard_device_auth_modes_cover_protocol_plugin_security_modes() {
    let modes = DeviceAuthMode::standard_modes();

    for expected in [
        DeviceAuthMode::BearerToken,
        DeviceAuthMode::Hmac,
        DeviceAuthMode::MtlsX509,
        DeviceAuthMode::HardwareAttestation,
        DeviceAuthMode::BrokerCredential,
        DeviceAuthMode::BridgeTrust,
    ] {
        assert!(modes.contains(&expected));
        assert!(!expected.manifest_name().is_empty());
    }
}
