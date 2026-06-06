# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

No `sdkwork.app.config.json` is present at this root. If the task changes application behavior, runtime config, SDK wiring, release metadata, or app-owned capabilities, first locate the nearest application root that has this manifest or add one according to the root specs.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `CLAUDE.md`: Claude Code compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `GEMINI.md`: Gemini CLI compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `CODEX.md`: Codex compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: not present here; required for application roots.
- `.sdkwork/`: reserved local dictionary folder; create only for local skills, plugins, manifests, or AI workspace metadata.
- `specs/`: local application/component contracts and narrowing rules.
- `sdks/`: SDK families, OpenAPI authorities, route manifests, and generated SDK artifacts.
- `Cargo.toml`: language/build manifests.
- Local directories to inspect first when relevant: `crates/`, `docs/`, `external/`, `sdks/`, `services/`, `specs/`.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` when present.
3. Read local `specs/README.md` and `specs/component.spec.json` when present.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` when relevant.
5. Read `../sdkwork-specs/README.md` and the task-specific root specs.
6. Inspect implementation files only after the relevant dictionary entries are clear.

## Required Specs By Task Type

- Agent/workflow changes: `../sdkwork-specs/SOUL.md`, `../sdkwork-specs/AGENTS_SPEC.md`, `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- Any code change: `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- Rust code: `../sdkwork-specs/RUST_CODE_SPEC.md` and `../sdkwork-specs/RUST_RPC_SPEC.md` when RPC is touched.
- Java/Spring code: `../sdkwork-specs/JAVA_CODE_SPEC.md` and `../sdkwork-specs/WEB_BACKEND_SPEC.md` when HTTP backend behavior is touched.
- TypeScript/Node code: `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../sdkwork-specs/FRONTEND_SPEC.md`, `../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and exactly one detailed UI architecture spec.
- API, SDK, database, runtime, security, and deployment changes must follow the task matrix in `../sdkwork-specs/README.md`.

Language-specific specs are on-demand; do not load Rust, Java, TypeScript, and frontend specs for unrelated tasks.

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes.

Load language specs only when touched: Rust uses `RUST_CODE_SPEC.md`, Java/Spring uses `JAVA_CODE_SPEC.md`, TypeScript/Node uses `TYPESCRIPT_CODE_SPEC.md`, and frontend/UI uses `FRONTEND_CODE_SPEC.md`.

For Rust, keep `src/lib.rs` limited to module declarations, re-exports, light docs, and wiring; move handlers, services, repositories, DTOs, SQL, provider clients, and tests into focused modules.

## Build, Test, and Verification

Run commands from this directory unless a command explicitly targets another path.

- `cargo fmt --all --check`: verify Rust formatting across workspace crates.
- `cargo test --workspace`: run workspace Rust tests.
- `cargo clippy --workspace --tests -- -D warnings`: lint Rust tests and crates with warnings denied.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, persistence, security, or cross-package boundaries change.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the task is explicitly about generated artifacts and the source contract is verified. Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning module, package, crate, or app root. Record the exact verification commands and important outputs before reporting completion.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, or changing generated SDK ownership. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.

## Existing Local Guidance

The repository-specific guidance below was preserved from the previous `AGENTS.md`. If it conflicts with the SDKWORK sections above or with `../sdkwork-specs/`, the SDKWORK standards win.

### Project Structure & Module Organization

This repository is a Rust workspace for the SDKWork AIoT server. Shared libraries live under `crates/`, including contracts, protocol, runtime, storage, security, observability, transport, HTTP API, architecture checks, and the `sdkwork-aiot-adapter-xiaozhi` integration. Runnable services live under `services/`: `sdkwork-aiot-gateway`, `sdkwork-aiot-admin-api`, and `sdkwork-aiot-app-api`. Tests are colocated in each crate or service under `tests/`, usually with `*_standard.rs` names. Generated or packaged SDK artifacts live in `sdks/`; specification inputs live in `specs/`; design and planning notes live in `docs/`. The `external/` tree contains reference projects and submodules and should not be edited for normal product changes.

### Build, Test, and Development Commands

- `cargo build --workspace`: compile all workspace crates and services.
- `cargo test --workspace`: run the full Rust test suite.
- `cargo test -p sdkwork-aiot-gateway`: run tests for one package.
- `cargo run -p sdkwork-aiot-gateway`: start the local gateway service.
- `cargo run -p sdkwork-aiot-xiaozhi-simulator-ui`: launch the cross-platform Xiaozhi simulator UI.
- PowerShell gateway bind example: `$env:SDKWORK_AIOT_GATEWAY_BIND='127.0.0.1:18080'; cargo run -p sdkwork-aiot-gateway`.
- Optional persistent device DB: `$env:SDKWORK_AIOT_DEVICE_DB_PATH='D:\\data\\aiot-device.db'`.

### Coding Style & Naming Conventions

Use Rust 2021 idioms and keep modules small, typed, and explicit. Run `cargo fmt --all` before submitting changes. Prefer `snake_case` for modules, functions, variables, and test names; use `PascalCase` for structs, enums, and traits; use `SCREAMING_SNAKE_CASE` for constants. Package names follow the existing `sdkwork-aiot-*` pattern. Keep public APIs documented when they define cross-crate behavior.

### Testing Guidelines

Use Rust integration tests in each package's `tests/` directory. Name test files by behavior or standard surface, for example `xiaozhi_standard.rs`, `gateway_standard.rs`, or `transport_standard.rs`. Add focused tests for protocol compatibility, gateway routing, adapter parsing, and error cases before changing behavior. Run the narrow package test first, then `cargo test --workspace` before opening a pull request.

### Commit & Pull Request Guidelines

Recent commits use short imperative summaries such as `Model Raspberry Pi hardware gateway profiles` and `Implement SDKWork AIoT server foundation`. Follow that style: one clear sentence, no trailing period. Pull requests should include a concise description, affected crates or services, test evidence, and any configuration or protocol compatibility notes. Include screenshots only for browser-facing changes such as the Xiaozhi simulator.

### Security & Configuration Tips

Do not commit real device tokens, broker credentials, certificates, or local bind secrets. Prefer environment variables for service configuration. When changing Xiaozhi access paths, verify both real-device headers and browser simulator query-parameter compatibility.
