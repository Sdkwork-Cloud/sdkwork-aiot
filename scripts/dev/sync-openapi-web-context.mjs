#!/usr/bin/env node
/**
 * Ensures OpenAPI authorities declare WebRequestContext and api-surface extensions
 * required by WEB_FRAMEWORK_SPEC.md section 7.
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const authorities = [
  {
    relativePath: 'apis/app-api/iot/sdkwork-aiot-app-api.openapi.json',
    apiSurface: 'app-api',
  },
  {
    relativePath: 'apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json',
    apiSurface: 'backend-api',
  },
];

let changed = 0;
const checkOnly = process.argv.includes('--check');

for (const authority of authorities) {
  const absolutePath = path.join(repoRoot, authority.relativePath);
  const original = fs.readFileSync(absolutePath, 'utf8');
  const openapi = JSON.parse(original.replace(/^\uFEFF/u, ''));

  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== 'object' || !operation.operationId) {
        continue;
      }

      if (operation['x-sdkwork-request-context'] !== 'WebRequestContext') {
        operation['x-sdkwork-request-context'] = 'WebRequestContext';
        changed += 1;
      }
      if (operation['x-sdkwork-api-surface'] !== authority.apiSurface) {
        operation['x-sdkwork-api-surface'] = authority.apiSurface;
        changed += 1;
      }
    }
  }

  const next = `${JSON.stringify(openapi, null, 2)}\n`;
  if (checkOnly) {
    if (next !== original.replace(/^\uFEFF/u, '')) {
      console.error(`OpenAPI materialization drift detected in ${authority.relativePath}`);
      process.exit(1);
    }
    continue;
  }

  fs.writeFileSync(absolutePath, next, 'utf8');
}

if (checkOnly) {
  console.log('OpenAPI WebRequestContext extensions are materialized');
  process.exit(0);
}

console.log(`OpenAPI WebRequestContext extensions updated (${changed} field writes)`);
