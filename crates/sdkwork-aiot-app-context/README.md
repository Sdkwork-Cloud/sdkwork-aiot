# sdkwork-aiot-app-context

Domain: iot
Capability: request-context-bridge
Package type: rust-crate
Status: standard

This README is the SDKWork module entrypoint for `sdkwork-aiot-app-context`. The machine-readable component contract is `../../specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `aiot_context_from_web_request`

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

This crate maps framework-resolved `WebRequestContext` values into `AiotRequestContext` for app and backend HTTP surfaces. IAM association fields come from `sdkwork-appbase`; AIoT must not create parallel IAM tables or APIs.

## SaaS/Private/Local Behavior

Tenant, organization, actor, permission, and trace mapping stays consistent across deployment profiles because it is derived from the shared web-framework request context.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extend actor or scope mapping only through typed contract changes in `sdkwork-aiot-contract` and approved IAM integration updates.

## Verification

- `cargo test -p sdkwork-aiot-app-context`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
