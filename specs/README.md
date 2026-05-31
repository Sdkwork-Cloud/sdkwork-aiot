# SDKWork AIoT Server Component Specs

This component is the SDKWork AIoT server foundation. It is designed as a reusable Rust component first and service binaries second.

## Boundary

- Domain: `iot`
- Database prefix: `iot_`
- App API prefix: `/app/v3/api/iot`
- Backend API prefix: `/backend/v3/api/iot`
- Device compatibility prefix: `/iot/xiaozhi`
- IAM owner: external `sdkwork-appbase`

AIoT stores only IAM association fields such as `tenant_id`, `organization_id`, `user_id`, `owner_type`, `owner_id`, `created_by`, and `updated_by`. It must not create IAM tables, IAM APIs, or a parallel IAM crate.

## Integration Modes

- Embedded library: host applications build `AiotRuntime` and mount selected routes/listeners.
- Standalone server: `services/sdkwork-aiot-*` binaries assemble the same runtime builder and expose configured routes.
- Plugin protocol: xiaozhi and future protocols register as protocol adapters; core domain models stay protocol-neutral.

## Protocol Plugin Standard

Every device protocol implementation is a plugin. Xiaozhi is the first compatibility plugin, not a core special case.

`ProtocolAdapterManifest` is the canonical plugin contract. A plugin must declare:

- `scope`: standard adapter, compatibility plugin, or bridge adapter.
- `protocol_ids`: stable protocol identifiers such as `xiaozhi.websocket`, `mqtt.v5`, `coap.lwm2m`, or `modbus.bridge`.
- `transports`: WebSocket, TCP, UDP, MQTT, CoAP, serial, BLE, HTTP, or future bindings.
- `codecs`: JSON text, JSON-RPC, binary media, binary payload, protobuf, CBOR, topic payload, or register map.
- `session_policies`: stateful device sessions, stateless uplink, broker sessions, bridge sessions, or gateway-multiplexed sessions.
- `capability_bridges`: mapping rules from protocol payloads to SDKWork semantic capabilities.
- `security_modes`: device auth modes such as bearer token, HMAC, mTLS, X.509, broker credential, or bridge trust.
- `hardware_families`, `runtime_profiles`, and `firmware_profiles`: compatibility metadata for chips, SDKs, RTOS/runtime stacks, and firmware ecosystems.

Adapters may decode protocol frames into `ProtocolEnvelope` and encode `ProtocolEnvelope` back to transport frames. They must not own database writes, IAM user/session logic, or app/backend API handlers.

Runtime registration uses the manifest plus `AiotProtocolRoute` metadata. The runtime maps protocol-neutral `MessageClass` values into standard ingest actions and then into core domain ingest plans. Storage implementations consume repository/outbox/dead-letter ports, not plugin-specific payloads.

## Canonical Specs

This component narrows the root SDKWork standards:

- `API_SPEC.md`
- `SDK_SPEC.md`
- `DOMAIN_SPEC.md`
- `DATABASE_SPEC.md`
- `COMPONENT_SPEC.md`
- `SECURITY_SPEC.md`
- `OBSERVABILITY_SPEC.md`
- `TEST_SPEC.md`
