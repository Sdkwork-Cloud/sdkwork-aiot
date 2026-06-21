# sdkwork-aiot-database-host

Domain: iot
Capability: database-host
Package type: rust-crate
Status: standard

This README is the SDKWork module entrypoint for `sdkwork-aiot-database-host`. The machine-readable component contract is `../../specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `AiotDatabaseHost`
- `bootstrap_aiot_database`

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Database assets live under `database/` and are loaded through `sdkwork-database` module manifests and lifecycle orchestration. Use root `db:*` scripts for plan, init, migrate, seed, status, drift, and bootstrap operations.

## SaaS/Private/Local Behavior

SQLite and PostgreSQL engines are supported through shared sdkwork-database assets. Environment selection follows deployment topology and `AIOT_DEVICE_*` lifecycle options rather than crate-local hardcoding.

## Security

Do not embed credentials in this crate. Connection and lifecycle configuration must come from approved environment and deployment inputs.

## Extension Points

Schema evolution belongs in `database/` assets and sdkwork-database lifecycle scripts. Do not add direct `rusqlite` or ad hoc SQL bootstrap paths here.

## Verification

- `pnpm db:validate`
- `cargo test -p sdkwork-aiot-database-host`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
