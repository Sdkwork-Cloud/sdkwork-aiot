use sdkwork_aiot_contract::{
    aiot_component_manifest, aiot_domain_record, standard_api_surfaces, standard_permissions,
    AiotActorRef, AiotComponentManifest, AiotOwnershipRef, AiotRequestContext, IOT_APP_API_PREFIX,
    IOT_BACKEND_API_PREFIX, IOT_PERMISSION_COMMANDS_EXECUTE, IOT_PERMISSION_DEVICES_READ,
    IOT_PERMISSION_DEVICES_WRITE, IOT_PERMISSION_FIRMWARE_WRITE, IOT_PERMISSION_RUNTIME_READ,
    IOT_XIAOZHI_BASE_PATH,
};

#[test]
fn request_context_uses_appbase_iam_association_without_owning_iam() {
    let ctx = AiotRequestContext::new("t1", "o1")
        .with_user("u1")
        .with_actor(AiotActorRef::iam_user("u1"))
        .with_permission("iot.devices.read")
        .with_data_scope("tenant:t1");

    assert_eq!(ctx.tenant_id, "t1");
    assert_eq!(ctx.organization_id, "o1");
    assert_eq!(ctx.user_id.as_deref(), Some("u1"));
    assert!(ctx.has_permission("iot.devices.read"));
    assert_eq!(ctx.actor.actor_type, "iam_user");
}

#[test]
fn ownership_ref_models_iam_and_device_owners_without_foreign_tables() {
    let user_owner = AiotOwnershipRef::iam_user("u1");
    let device_owner = AiotOwnershipRef::device("dev1");

    assert_eq!(user_owner.owner_type, "user");
    assert_eq!(user_owner.owner_id, "u1");
    assert_eq!(device_owner.owner_type, "device");
    assert_eq!(device_owner.owner_id, "dev1");
}

#[test]
fn component_manifest_declares_composable_building_block_contract() {
    let manifest = AiotComponentManifest::new("sdkwork-aiot-service-host", "runtime")
        .with_capability("embedded_runtime")
        .with_capability("standalone_server");

    assert_eq!(manifest.name, "sdkwork-aiot-service-host");
    assert_eq!(manifest.domain, "runtime");
    assert!(manifest
        .capabilities
        .contains(&"embedded_runtime".to_string()));
    assert!(manifest
        .capabilities
        .contains(&"standalone_server".to_string()));
}

#[test]
fn standard_paths_and_permissions_are_stable() {
    assert_eq!(IOT_APP_API_PREFIX, "/app/v3/api/iot");
    assert_eq!(IOT_BACKEND_API_PREFIX, "/backend/v3/api/iot");
    assert_eq!(IOT_XIAOZHI_BASE_PATH, "/iot/xiaozhi");
    assert_eq!(IOT_PERMISSION_DEVICES_READ, "iot.devices.read");
    assert_eq!(IOT_PERMISSION_DEVICES_WRITE, "iot.devices.write");
    assert_eq!(IOT_PERMISSION_COMMANDS_EXECUTE, "iot.commands.execute");
    assert_eq!(IOT_PERMISSION_FIRMWARE_WRITE, "iot.firmware.write");
    assert_eq!(IOT_PERMISSION_RUNTIME_READ, "iot.runtime.read");
}

#[test]
fn domain_record_aligns_sdkwork_surfaces() {
    let domain = aiot_domain_record();

    assert_eq!(domain.domain, "iot");
    assert_eq!(domain.database_prefix, "iot");
    assert_eq!(domain.api_tag, "iot");
    assert_eq!(domain.permission_prefix, "iot");
    assert_eq!(domain.event_prefix, "iot");
    assert!(domain.capabilities.contains(&"deviceRegistry"));
    assert!(domain.capabilities.contains(&"protocolGateway"));
    assert!(domain.capabilities.contains(&"commandControl"));
    assert!(domain.capabilities.contains(&"otaProvisioning"));
    assert!(domain
        .external_shared_kernels
        .contains(&"sdkwork-appbase.iam"));
}

#[test]
fn api_surfaces_follow_sdkwork_v3_prefixes_and_sdk_namespaces() {
    let surfaces = standard_api_surfaces();

    assert!(surfaces
        .iter()
        .any(|surface| surface.name == "app" && surface.prefix == "/app/v3/api/iot"));
    assert!(surfaces
        .iter()
        .any(|surface| { surface.name == "backend" && surface.prefix == "/backend/v3/api/iot" }));
    assert!(surfaces
        .iter()
        .all(|surface| surface.openapi_required && surface.generated_sdk_required));
    assert!(surfaces.iter().all(|surface| surface
        .operation_id_examples
        .iter()
        .all(|id| id.contains('.'))));
}

#[test]
fn permission_catalog_is_complete_enough_for_first_standard_slice() {
    let permissions = standard_permissions();

    for expected in [
        "iot.products.read",
        "iot.products.write",
        "iot.profiles.read",
        "iot.devices.read",
        "iot.devices.write",
        "iot.sessions.disconnect",
        "iot.commands.execute",
        "iot.twins.write",
        "iot.telemetry.read",
        "iot.firmware.write",
        "iot.protocolAdapters.read",
        "iot.runtime.read",
    ] {
        assert!(
            permissions.contains(&expected),
            "missing permission {expected}"
        );
    }
}

#[test]
fn default_component_manifest_declares_library_first_integration_contract() {
    let manifest = aiot_component_manifest();

    assert_eq!(manifest.name, "sdkwork-aiot-server");
    assert_eq!(manifest.domain, "iot");
    assert!(manifest
        .capabilities
        .contains(&"embedded_runtime".to_string()));
    assert!(manifest
        .capabilities
        .contains(&"standalone_server".to_string()));
    assert!(manifest
        .capabilities
        .contains(&"protocol_plugins".to_string()));
    assert!(manifest
        .required_features
        .contains(&"external_appbase_iam_context".to_string()));
}
