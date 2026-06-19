import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const authorities = [
  'apis/app-api/iot/sdkwork-aiot-app-api.openapi.json',
  'apis/backend-api/iot/sdkwork-aiot-backend-api.openapi.json',
];

const removeRefs = new Set([
  '#/components/parameters/SdkworkTenantId',
  '#/components/parameters/SdkworkOrganizationId',
  '#/components/parameters/SdkworkUserId',
  '#/components/parameters/SdkworkDataScope',
  '#/components/parameters/SdkworkPermissionScope',
]);

for (const relativePath of authorities) {
  const openapiPath = path.resolve(__dirname, '..', relativePath);

  const doc = JSON.parse(fs.readFileSync(openapiPath, 'utf8'));

  for (const pathItem of Object.values(doc.paths ?? {})) {
    if (!pathItem || typeof pathItem !== 'object' || !Array.isArray(pathItem.parameters)) {
      continue;
    }
    pathItem.parameters = pathItem.parameters.filter((entry) => !removeRefs.has(entry?.$ref));
    if (pathItem.parameters.length === 0) {
      delete pathItem.parameters;
    }
  }

  for (const key of [
    'SdkworkTenantId',
    'SdkworkOrganizationId',
    'SdkworkUserId',
    'SdkworkDataScope',
    'SdkworkPermissionScope',
  ]) {
    delete doc.components.parameters[key];
  }

  fs.writeFileSync(openapiPath, `${JSON.stringify(doc, null, 2)}\n`);
  console.log(`stripped client context headers from ${openapiPath}`);
}
