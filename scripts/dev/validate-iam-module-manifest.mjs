#!/usr/bin/env node
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifestPath = path.join(repoRoot, 'specs/iam.module.manifest.json');
const openapiAuthorities = [
  'apis/app-api/iot/sdkwork-aiot-app-api.openapi.json',
  'apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json',
];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function permissionDomain(code) {
  return code.split('.')[0];
}

function permissionMatchesPattern(code, pattern) {
  if (pattern === code) {
    return true;
  }
  if (pattern.endsWith('.*')) {
    const prefix = pattern.slice(0, -1);
    return code.startsWith(prefix);
  }
  return false;
}

function collectOpenApiPermissions(relativePath) {
  const openapi = readJson(relativePath);
  const permissions = new Set();
  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (operation && typeof operation === 'object') {
        const permission = operation['x-sdkwork-required-permission'];
        if (typeof permission === 'string' && permission.length > 0) {
          permissions.add(permission);
        }
      }
    }
  }
  return permissions;
}

function validateManifestShape(manifest) {
  assert.equal(manifest.schemaVersion, 1, 'schemaVersion must be 1');
  assert.equal(manifest.kind, 'sdkwork.iam.module', 'kind must be sdkwork.iam.module');
  assert.equal(manifest.moduleId, 'iot', 'moduleId must be iot');
  assert.equal(manifest.domain, 'iot', 'domain must be iot');
  assert.equal(manifest.owner, 'sdkwork-aiot', 'owner must be sdkwork-aiot');
  assert.ok(Array.isArray(manifest.permissions?.catalog), 'permissions.catalog must be an array');
  assert.ok(Array.isArray(manifest.permissions?.openapiAuthorities), 'permissions.openapiAuthorities must be an array');
  assert.ok(Array.isArray(manifest.dependencies?.requiresModules), 'dependencies.requiresModules must be an array');
  assert.ok(
    manifest.dependencies.requiresModules.includes('iam-kernel'),
    'iot module must depend on iam-kernel',
  );
}

function validatePermissionCatalog(manifest) {
  const codes = new Set();
  for (const permission of manifest.permissions.catalog) {
    assert.match(
      permission.code,
      /^iot\.[a-zA-Z0-9_]+\.[a-zA-Z0-9_]+$/,
      `permission code ${permission.code} must follow iot.{resource}.{action}`,
    );
    assert.equal(
      permissionDomain(permission.code),
      manifest.domain,
      `permission ${permission.code} must stay inside domain ${manifest.domain}`,
    );
    assert.ok(!codes.has(permission.code), `duplicate permission code ${permission.code}`);
    codes.add(permission.code);

    if (permission.status === 'deprecated') {
      assert.ok(
        permission.replacementCode,
        `deprecated permission ${permission.code} must declare replacementCode`,
      );
    }
  }
  return codes;
}

function validateOpenApiAuthorities(manifest) {
  const declared = manifest.permissions.openapiAuthorities;
  assert.deepEqual(
    [...declared].sort(),
    [...openapiAuthorities].sort(),
    'permissions.openapiAuthorities must declare canonical apis/ OpenAPI authorities',
  );
  for (const authority of declared) {
    assert.ok(
      fs.existsSync(path.join(repoRoot, authority)),
      `openapi authority ${authority} must exist`,
    );
  }
}

function validateOpenApiPermissionSubset(catalogCodes) {
  for (const authority of openapiAuthorities) {
    for (const permission of collectOpenApiPermissions(authority)) {
      assert.ok(
        catalogCodes.has(permission),
        `${authority} declares ${permission} but specs/iam.module.manifest.json catalog does not`,
      );
    }
  }
}

function validateRolePatterns(manifest, catalogCodes) {
  for (const role of manifest.roles?.domainStandardRoles ?? []) {
    for (const pattern of role.permissionPatterns ?? []) {
      const matches = [...catalogCodes].some((code) => permissionMatchesPattern(code, pattern));
      assert.ok(
        matches,
        `role ${role.code} pattern ${pattern} must expand against catalog permissions`,
      );
    }
  }

  for (const extension of manifest.roles?.roleGrantExtensions ?? []) {
    for (const pattern of extension.patterns ?? []) {
      const matches = [...catalogCodes].some((code) => permissionMatchesPattern(code, pattern));
      assert.ok(
        matches,
        `role grant extension ${extension.roleCode} pattern ${pattern} must expand against catalog permissions`,
      );
    }
  }
}

function validateAppbaseMirror() {
  const appbaseMirror = path.resolve(repoRoot, '../sdkwork-appbase/iam/modules/iot/iam.module.manifest.json');
  if (!fs.existsSync(appbaseMirror)) {
    return;
  }
  const local = JSON.stringify(readJson('specs/iam.module.manifest.json'));
  const remote = JSON.stringify(JSON.parse(fs.readFileSync(appbaseMirror, 'utf8')));
  assert.equal(
    local,
    remote,
    'specs/iam.module.manifest.json must stay semantically identical to sdkwork-appbase/iam/modules/iot/iam.module.manifest.json',
  );
}

const manifest = readJson('specs/iam.module.manifest.json');
validateManifestShape(manifest);
const catalogCodes = validatePermissionCatalog(manifest);
validateOpenApiAuthorities(manifest);
validateOpenApiPermissionSubset(catalogCodes);
validateRolePatterns(manifest, catalogCodes);
validateAppbaseMirror();

process.stdout.write(
  `[validate-iam-module-manifest] ok (${catalogCodes.size} permissions, ${openapiAuthorities.length} openapi authorities)\n`,
);
