# Scripts

Thin command entrypoints for SDKWork AIoT development, verification, and packaging.

| Script | Purpose |
| --- | --- |
| `aiot-dev.mjs` | Topology-aware split-service development orchestrator |
| `dev-with-simulator.mjs` | Dev stack plus Xiaozhi simulator UI |
| `lib/aiot-topology.mjs` | Topology profile adapter for `@sdkwork/app-topology` |
| `dev/*.test.mjs` | Contract and baggage tests |
| `gateway-package.mjs` | Release gateway/server binary packaging |
| `sbom-generate.mjs` / `sbom-check.mjs` | SBOM evidence helpers |

Public root commands are declared in `package.json` per `PNPM_SCRIPT_SPEC.md`.
