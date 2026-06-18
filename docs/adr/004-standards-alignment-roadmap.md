# ADR 004: SDKWork Standards Alignment Roadmap

## Status

Accepted — supersedes the HTTP-stack deferral scope in ADR 002 for app/backend API surfaces.

## Context

`sdkwork-specs` now mandates:

- `WEB_FRAMEWORK_SPEC.md` for every HTTP `*-api` runtime
- `DATABASE_SPEC.md` through `sdkwork-database` for persistence runtimes
- `GITHUB_WORKFLOW_SPEC.md` for packaging and release workflows
- Route manifest and OpenAPI metadata for `WebRequestContext` and `apiSurface`

The AIoT server already aligns on API contracts, SDK workspaces, topology, security posture, and architecture tests, but still uses:

- a custom minimal HTTP stack in `sdkwork-aiot-transport` for gateway/device ingress (ADR 002)
- legacy crate names such as `sdkwork-aiot-http-api` instead of `sdkwork-router-iot-*`

ADR 002 intentionally deferred Axum migration for gateway/device workloads. That deferral remains valid for device ingress, but app/backend HTTP APIs must converge on `sdkwork-web-framework`.

## Decision

1. Treat standards alignment as a phased migration, not a one-shot rewrite.
2. Keep device gateway transport (`sdkwork-aiot-transport`) on the current minimal stack until a funded gateway migration milestone.
3. Migrate app/backend HTTP APIs to `sdkwork-web-framework` using the documented custom-transport path:
   - adopt `sdkwork-web-core` request-context traits and the standard interceptor chain semantics
   - reuse `sdkwork-iam-web-adapter` for dual-token resolution where appbase proxy headers are present
   - optionally adopt `sdkwork-web-axum` for admin/app API servers after route crates are split
4. Migrate persistence from direct `rusqlite` to `sdkwork-database-sqlx` pools and migration helpers while preserving the existing `iot_` table contract.
5. Do not integrate `sdkwork-discovery` until the repository exposes RPC/gRPC services.
6. Add GitHub packaging through `sdkwork.workflow.json` and `.github/workflows/package.yml` immediately.
7. Require route manifests and OpenAPI authorities to declare `WebRequestContext` and `apiSurface` metadata immediately.

## Phases

| Phase | Scope | Exit criteria | Status |
| --- | --- | --- | --- |
| A | Workflow, root dictionary, route/OpenAPI metadata | `sdkwork.workflow.json`, route manifests, OpenAPI extensions, architecture tests | Done |
| B | Web framework core adoption | `sdkwork-web-framework` workspace deps; custom transport uses framework context + interceptor semantics | Done |
| C | Route crate split + Axom servers | `sdkwork-router-iot-app-api`, `sdkwork-router-iot-backend-api`, Tokio/Axum service shells | Done |
| D | Database framework adoption | `sdkwork-database-config` bootstrap + `sdkwork-database-sqlx` pool helpers; repository SQL via sqlx pools | Done |
| E | Crate rename cleanup | Remove generic `core/runtime/http-api` names per `NAMING_SPEC.md` | Pending |

## Consequences

- ADR 002 remains authoritative for gateway/device transport only.
- Architecture tests will gain additional guardrails as phases B–E land.
- Full alignment requires multiple PRs; Phase A is intentionally shippable without behavior changes.

## Verification

- `pnpm test:topology-validate`
- `pnpm test:app-openapi-context`
- `pnpm test:openapi-web-context`
- `cargo test -p sdkwork-aiot-architecture`
- `cargo test --workspace`
