#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const packageId = process.env.SDKWORK_PACKAGE_ID || 'sdkwork-aiot';
const output = path.join(root, 'artifacts/release/sbom', `${packageId}.sbom.json`);

if (!existsSync(output)) {
  console.error(`[sbom:check] missing SBOM evidence at ${path.relative(root, output)}`);
  process.exit(1);
}

console.log(`[sbom:check] ok ${path.relative(root, output)}`);
