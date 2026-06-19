# SDKWork AIoT API Contracts

## Purpose

`apis/` stores SDKWork AIoT authored API contract sources and materialization inputs.

## Owner

sdkwork-aiot.

## Allowed Content

- `app-api/iot/` App API OpenAPI authority inputs.
- `backend-api/iot/` Backend API OpenAPI authority inputs.
- API examples, changelogs, fixtures, and contract validation inputs.

## Forbidden Content

- Generated SDK transport output.
- SDK family workspaces.
- Rust route, handler, service, or repository implementation code.
- Runtime state, credentials, or local override files.

## OpenAPI Authorities

| Surface | Authority | OpenAPI source |
| --- | --- | --- |
| App API | `sdkwork-aiot-app-api` | `apis/app-api/iot/sdkwork-aiot-app-api.openapi.json` |
| Backend API | `sdkwork-aiot-backend-api` | `apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json` |

## Route Manifests

| Surface | Manifest |
| --- | --- |
| App API | `sdks/_route-manifests/app-api/sdkwork-aiot-app-api.route-manifest.json` |
| Backend API | `sdks/_route-manifests/backend-api/sdkwork-aiot-admin-api.route-manifest.json` |

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`

## Verification

Run `pnpm test:openapi-web-context`, `pnpm test:app-openapi-context`, and `cargo test -p sdkwork-aiot-architecture`.
