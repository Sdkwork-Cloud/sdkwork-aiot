import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

const families = [
  {
    id: 'sdkwork-aiot-app-sdk',
    apiSurface: 'app-api',
  },
  {
    id: 'sdkwork-aiot-backend-sdk',
    apiSurface: 'backend-api',
  },
];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

for (const family of families) {
  const openapiPath = `sdks/${family.id}/openapi/${family.id}.openapi.json`;
  const openapi = readJson(openapiPath);

  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== 'object' || !operation.operationId) {
        continue;
      }

      assert.equal(
        operation['x-sdkwork-request-context'],
        'WebRequestContext',
        `${family.id} operation ${operation.operationId} must declare x-sdkwork-request-context`,
      );
      assert.equal(
        operation['x-sdkwork-api-surface'],
        family.apiSurface,
        `${family.id} operation ${operation.operationId} must declare x-sdkwork-api-surface`,
      );
    }
  }
}

console.log('sdkwork-aiot OpenAPI WebRequestContext contract passed');
