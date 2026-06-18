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

const families = [
  { id: 'sdkwork-aiot-app-sdk', apiSurface: 'app-api' },
  { id: 'sdkwork-aiot-backend-sdk', apiSurface: 'backend-api' },
];

let changed = 0;

for (const family of families) {
  const relativePath = `sdks/${family.id}/openapi/${family.id}.openapi.json`;
  const absolutePath = path.join(repoRoot, relativePath);
  const openapi = JSON.parse(fs.readFileSync(absolutePath, 'utf8'));

  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== 'object' || !operation.operationId) {
        continue;
      }

      if (operation['x-sdkwork-request-context'] !== 'WebRequestContext') {
        operation['x-sdkwork-request-context'] = 'WebRequestContext';
        changed += 1;
      }
      if (operation['x-sdkwork-api-surface'] !== family.apiSurface) {
        operation['x-sdkwork-api-surface'] = family.apiSurface;
        changed += 1;
      }
    }
  }

  fs.writeFileSync(absolutePath, `${JSON.stringify(openapi, null, 2)}\n`, 'utf8');
}

console.log(`OpenAPI WebRequestContext extensions updated (${changed} field writes)`);
