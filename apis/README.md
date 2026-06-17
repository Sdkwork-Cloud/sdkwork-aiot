# SDKWork AIoT API Contracts

This directory documents where the HTTP API authorities live for the SDKWork AIoT server.

Runtime route contracts are declared in `crates/sdkwork-aiot-http-api` via `standard_api_route_contracts()` and materialized route manifests under `sdks/_route-manifests/`.

## OpenAPI Authorities

| Surface | Authority | OpenAPI source |
| --- | --- | --- |
| App API | `sdkwork-aiot-app-api` | `sdks/sdkwork-aiot-app-sdk/openapi/sdkwork-aiot-app-sdk.openapi.json` |
| Backend API | `sdkwork-aiot-backend-api` | `sdks/sdkwork-aiot-backend-sdk/openapi/sdkwork-aiot-backend-sdk.openapi.json` |

## Route Manifests

| Surface | Manifest |
| --- | --- |
| App API | `sdks/_route-manifests/app-api/sdkwork-aiot-app-api.route-manifest.json` |
| Backend API | `sdks/_route-manifests/backend-api/sdkwork-aiot-admin-api.route-manifest.json` |

## Device Protocol Paths

Device-facing Xiaozhi compatibility routes are owned by `services/sdkwork-aiot-gateway` and are not part of the app/backend OpenAPI authorities:

- `/iot/xiaozhi/ws`
- `/iot/xiaozhi/ota`
- `/iot/xiaozhi/activate`

## Regenerating Route Manifests

When HTTP route contracts change, refresh the committed manifests:

```powershell
$env:SDKWORK_EXPORT_ROUTE_MANIFESTS='1'
cargo test -p sdkwork-aiot-http-api export_route_manifest_artifacts_when_requested -- --exact
```
