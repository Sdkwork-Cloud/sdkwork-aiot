#!/usr/bin/env node
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const root = process.cwd();

function exists(relativePath) {
  return existsSync(path.join(root, relativePath));
}

function read(relativePath) {
  return readFileSync(path.join(root, relativePath), 'utf8');
}

test('sdkwork-aiot uses the SDKWork standard project-root directory dictionary', () => {
  for (const directory of [
    'apis',
    'apps',
    'crates',
    'sdks',
    'jobs',
    'tools',
    'plugins',
    'examples',
    'configs',
    'deployments',
    'scripts',
    'docs',
    'tests',
  ]) {
    assert.ok(exists(`${directory}/README.md`), `${directory}/README.md must exist`);
  }
});

test('sdkwork-aiot keeps API authority inputs under apis', () => {
  for (const apiPath of [
    'apis/app-api/iot/sdkwork-aiot-app-api.openapi.json',
    'apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json',
  ]) {
    assert.ok(exists(apiPath), `${apiPath} must exist`);
  }
});

test('sdkwork-aiot root package.json follows PNPM script standard', () => {
  const result = spawnSync(
    'node',
    [
      '../sdkwork-specs/tools/check-pnpm-script-standard.mjs',
      '--root',
      '.',
      '--application-code-prefix',
      'aiot',
    ],
    { cwd: root, encoding: 'utf8' },
  );
  assert.equal(
    result.status,
    0,
    `pnpm script standard failed:\n${result.stdout}\n${result.stderr}`,
  );
});

test('sdkwork-aiot dev scripts use deployment-profile axis instead of retired hosting flags', () => {
  const packageJson = JSON.parse(read('package.json'));
  const devCommand = String(packageJson.scripts?.dev ?? '');
  assert.match(
    devCommand,
    /--deployment-profile\s+standalone/u,
    'dev must use --deployment-profile standalone',
  );
  assert.doesNotMatch(devCommand, /--hosting/u, 'dev must not use retired --hosting');
  assert.doesNotMatch(
    JSON.stringify(packageJson.scripts ?? {}),
    /"aiot:/u,
    'public root scripts must not use application-code prefix aiot:',
  );
});

test('sdkwork-aiot declares sdkwork framework dependencies in workflow manifest', () => {
  const workflow = read('sdkwork.workflow.json');
  for (const dependency of [
    'sdkwork-web-framework',
    'sdkwork-database',
    'sdkwork-utils',
    'sdkwork-sdk-generator',
    'sdkwork-app-topology',
  ]) {
    assert.match(workflow, new RegExp(`"id": "${dependency}"`, 'u'), `${dependency} dependency required`);
  }
});

test('sdkwork-aiot uses responsibility-specific Rust crate names', () => {
  for (const cratePath of [
    'crates/sdkwork-iot-device-service/Cargo.toml',
    'crates/sdkwork-aiot-service-host/Cargo.toml',
    'crates/sdkwork-iot-platform-service/Cargo.toml',
    'crates/sdkwork-router-iot-app-api/Cargo.toml',
    'crates/sdkwork-router-iot-backend-api/Cargo.toml',
  ]) {
    assert.ok(exists(cratePath), `${cratePath} must exist`);
  }

  const forbidden = [
    'crates/sdkwork-aiot-core',
    'crates/sdkwork-aiot-runtime',
    'crates/sdkwork-aiot-backend',
    'crates/sdkwork-aiot-common',
  ];
  for (const cratePath of forbidden) {
    assert.equal(exists(cratePath), false, `${cratePath} must not exist`);
  }
});

test('sdkwork-aiot dev orchestrator resolves deployment profiles through topology adapter', () => {
  const devScript = read('scripts/aiot-dev.mjs');
  assert.match(devScript, /resolveDevProfileFromDeploymentProfile/u);
  assert.match(devScript, /--deployment-profile/u);
  assert.match(
    devScript,
    /--hosting is retired/u,
    'aiot-dev must reject retired --hosting flag',
  );
});
