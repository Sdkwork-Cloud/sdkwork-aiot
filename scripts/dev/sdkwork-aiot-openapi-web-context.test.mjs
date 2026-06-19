import assert from 'node:assert/strict';
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

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

for (const authority of authorities) {
  const openapi = readJson(authority.relativePath);

  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== 'object' || !operation.operationId) {
        continue;
      }

      assert.equal(
        operation['x-sdkwork-request-context'],
        'WebRequestContext',
        `${authority.relativePath} operation ${operation.operationId} must declare x-sdkwork-request-context`,
      );
      assert.equal(
        operation['x-sdkwork-api-surface'],
        authority.apiSurface,
        `${authority.relativePath} operation ${operation.operationId} must declare x-sdkwork-api-surface`,
      );
    }
  }
}

console.log('sdkwork-aiot OpenAPI WebRequestContext contract passed');
