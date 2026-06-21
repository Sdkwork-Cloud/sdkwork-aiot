#!/usr/bin/env node
import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const packageId = process.env.SDKWORK_PACKAGE_ID || 'sdkwork-aiot';
const output = path.join(root, 'artifacts/release/sbom', `${packageId}.sbom.json`);

mkdirSync(path.dirname(output), { recursive: true });
writeFileSync(
  output,
  `${JSON.stringify(
    {
      bomFormat: 'CycloneDX',
      specVersion: '1.6',
      version: 1,
      metadata: {
        timestamp: new Date().toISOString(),
        tools: [{ vendor: 'SDKWork', name: 'sdkwork-aiot-sbom', version: '1.0.0' }],
        component: {
          type: 'application',
          name: process.env.SDKWORK_APP_ID || 'sdkwork-aiot',
          version: process.env.SDKWORK_PACKAGE_VERSION || '0.1.0',
          'bom-ref': packageId,
        },
        properties: [
          { name: 'sdkwork:packageId', value: packageId },
          { name: 'sdkwork:runtimeTarget', value: process.env.SDKWORK_RUNTIME_TARGET || 'server' },
          {
            name: 'sdkwork:deploymentProfile',
            value: process.env.SDKWORK_DEPLOYMENT_PROFILE || 'standalone',
          },
        ],
      },
      components: [],
    },
    null,
    2,
  )}\n`,
  'utf8',
);

console.log(`[sbom:generate] wrote ${path.relative(root, output)}`);
