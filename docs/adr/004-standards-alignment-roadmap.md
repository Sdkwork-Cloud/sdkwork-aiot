# ADR 004: SDKWork Standards Alignment Roadmap

## Status

Accepted — supersedes the HTTP-stack deferral scope in ADR 002 for app/backend API surfaces.

## Context

`sdkwork-specs` now mandates:

- `WEB_FRAMEWORK_SPEC.md` for every HTTP `*-api` runtime
- `DATABASE_SPEC.md` through `sdkwork-database` for persistence runtimes
- `GITHUB_WORKFLOW_SPEC.md` for packaging and release workflows
- Route manifest and OpenAPI metadata for `WebRequestContext` and `apiSurface`

The AIoT server already aligns on API contracts, SDK workspaces, topology, security posture, and architecture tests. Remaining intentional exceptions:

- a custom minimal HTTP stack in `sdkwork-aiot-transport` for gateway/device ingress (ADR 002)

## Decision

1. Treat standards alignment as a phased migration, not a one-shot rewrite.
2. Keep device gateway transport (`sdkwork-aiot-transport`) on the current minimal stack until a funded gateway migration milestone.
3. Migrate app/backend HTTP APIs to `sdkwork-web-framework` using the documented custom-transport path:
   - adopt `sdkwork-web-core` request-context traits and the standard interceptor chain semantics
   - reuse `sdkwork-iam-web-adapter` for dual-token resolution where appbase proxy headers are present
   - optionally adopt `sdkwork-web-axum` for admin/app API servers after route crates are split
4. Migrate persistence from direct `rusqlite` to `sdkwork-database-sqlx` pools and migration helpers while preserving the existing `iot_` table contract.
5. Integrate `sdkwork-utils` for cross-language value parsing, string normalization, identifiers, and datetime helpers; consolidate duplicated client-side readers in `@sdkwork/aiot-app-core`.
6. Do not integrate `sdkwork-discovery` until the repository exposes RPC/gRPC services.
7. Keep `sdkwork-aiot-gateway` device ingress on the minimal transport stack documented in ADR 002; it is not an HTTP `*-api` surface and therefore does not require `sdkwork-web-framework` integration.
8. Add GitHub packaging through `sdkwork.workflow.json` and `.github/workflows/package.yml` immediately.
9. Require route manifests and OpenAPI authorities to declare `WebRequestContext` and `apiSurface` metadata immediately.
10. Expose repository-root scripts through the standard `dev`, `api`, `sdk`, `gateway`, `release`, `deploy`, `topology`, and `sbom` command families without application-code prefixes.

## Phases

| Phase | Scope | Exit criteria | Status |
| --- | --- | --- | --- |
| A | Workflow, root dictionary, route/OpenAPI metadata | `sdkwork.workflow.json`, route manifests, OpenAPI extensions, architecture tests | Done |
| B | Web framework core adoption | `sdkwork-web-framework` workspace deps; custom transport uses framework context + interceptor semantics | Done |
| C | Route crate split + Axom servers | `sdkwork-router-iot-app-api`, `sdkwork-router-iot-backend-api`, Tokio/Axum service shells | Done |
| D | Database framework adoption | `sdkwork-database-config` bootstrap + `sdkwork-database-sqlx` pool helpers; repository SQL via sqlx pools | Done |
| E | Crate rename cleanup | `sdkwork-iot-device-service`, `sdkwork-aiot-service-host`, `sdkwork-iot-platform-service`; no forbidden `core/runtime` crate names | Done |
| F | Shared persistence + `apis/` authority layout | Single `AiotDeviceDatabase` pool for device/credential/admin entities; authored OpenAPI under `apis/` | Done |
| G | Utils framework adoption | `sdkwork-utils-rust` in Rust persistence; `@sdkwork/utils` in shared app-core; repository script surface per `PNPM_SCRIPT_SPEC.md` | Done |
| H | API/SDK/gateway command surface | `api:*`, `sdk:*`, `gateway:*` root scripts; `tools/aiot_sdk_generate.mjs`; client runtime env helpers | Done |
| I | Repository script standard compliance | Remove `aiot:*` public scripts; `--deployment-profile` dev axis; workspace standard test; root `plugins/` dictionary | Done |
| J | Agent/workflow entrypoint compliance | `AGENTS.md` v2 progressive loading; `PNPM_SCRIPT_SPEC.md` + `GITHUB_WORKFLOW_SPEC.md` references; `check:agent-workflow-standard`; `sdkwork_utils_ref` in `package.yml`; durable local guidance in `specs/README.md` | Done |
| K | Postgres device persistence | `SDKWORK_AIOT_DEVICE_DATABASE_*` env wiring; async sqlx repositories; dev `--database postgres` orchestration | In progress — config resolution + fail-fast guardrails landed; repositories pending |

## Consequences

- ADR 002 remains authoritative for gateway/device transport only.
- Architecture tests will gain additional guardrails as phases B–E land.
- Full alignment requires multiple PRs; Phase A is intentionally shippable without behavior changes.

## Verification

- `pnpm check`
- `pnpm verify`
- `pnpm check:agent-workflow-standard`
- `pnpm test:topology-validate`
- `pnpm test:app-openapi-context`
- `pnpm test:openapi-web-context`
- `cargo test -p sdkwork-aiot-architecture`
- `cargo test --workspace`
