#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const topologyDir = path.join(repoRoot, 'configs', 'topology');

const MIN_SECRET_LENGTH = 32;
const FORBIDDEN_LEGACY_DEVICE_TOKEN = 'device-token';

function parseEnvFile(content) {
  const values = new Map();
  for (const line of content.split(/\r?\n/u)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) {
      continue;
    }
    const separator = trimmed.indexOf('=');
    if (separator <= 0) {
      continue;
    }
    const key = trimmed.slice(0, separator).trim();
    const value = trimmed.slice(separator + 1).trim();
    values.set(key, value);
  }
  return values;
}

function requireNonEmptySecret(fileName, values, key) {
  const value = values.get(key);
  assert.ok(
    typeof value === 'string' && value.length >= MIN_SECRET_LENGTH,
    `${fileName} must set ${key} with at least ${MIN_SECRET_LENGTH} characters`,
  );
}

function validateProductionProfile(fileName, values) {
  assert.equal(
    values.get('SDKWORK_AIOT_ENVIRONMENT'),
    'production',
    `${fileName} must declare production environment`,
  );
  assert.notEqual(
    values.get('SDKWORK_AIOT_DEV_MODE'),
    '1',
    `${fileName} must not enable SDKWORK_AIOT_DEV_MODE in production`,
  );
  const devicePath = values.get('SDKWORK_AIOT_DEVICE_DB_PATH');
  assert.ok(
    typeof devicePath === 'string' && devicePath.length > 0,
    `${fileName} must set SDKWORK_AIOT_DEVICE_DB_PATH for durable SQLite persistence`,
  );
  assert.equal(
    values.get('SDKWORK_AIOT_OUTBOX_DISPATCHER_ENABLED'),
    '1',
    `${fileName} must enable outbox dispatch on edge gateway`,
  );

  requireNonEmptySecret(fileName, values, 'SDKWORK_AIOT_INTERNAL_TOKEN');
  requireNonEmptySecret(fileName, values, 'SDKWORK_AIOT_CREDENTIAL_PEPPER');

  const corsOrigins = values.get('SDKWORK_AIOT_CORS_ALLOWED_ORIGINS');
  assert.ok(
    typeof corsOrigins === 'string' && corsOrigins.length > 0,
    `${fileName} must set SDKWORK_AIOT_CORS_ALLOWED_ORIGINS`,
  );

  const legacyDeviceToken = values.get('SDKWORK_AIOT_XIAOZHI_DEVICE_TOKEN');
  assert.ok(
    legacyDeviceToken === undefined,
    `${fileName} must not set SDKWORK_AIOT_XIAOZHI_DEVICE_TOKEN in production`,
  );
  assert.notEqual(
    values.get('SDKWORK_AIOT_INTERNAL_TOKEN'),
    FORBIDDEN_LEGACY_DEVICE_TOKEN,
    `${fileName} must not use the default dev internal token`,
  );
}

const productionProfiles = fs
  .readdirSync(topologyDir)
  .filter((fileName) => fileName.endsWith('.production.env'));

assert.ok(productionProfiles.length >= 2, 'expected self-hosted and cloud production profiles');

for (const fileName of productionProfiles) {
  const content = fs.readFileSync(path.join(topologyDir, fileName), 'utf8');
  validateProductionProfile(fileName, parseEnvFile(content));
}

console.log(
  `[validate-production-topology] ok (${productionProfiles.length} production profiles)`,
);
