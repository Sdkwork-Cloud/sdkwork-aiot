# Tests

Cross-package integration and contract tests that do not belong to a single crate or app surface.

- Rust crate/service tests remain colocated under each package `tests/` directory.
- Topology baggage scan: `pnpm test:topology-baggage`
- OpenAPI context contracts: `pnpm test:app-openapi-context`, `pnpm test:openapi-web-context`
- Architecture alignment: `cargo test -p sdkwork-aiot-architecture`
