#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { createReadStream } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { pipeline } from 'node:stream/promises';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const releaseDir = path.join(root, 'artifacts/release');
const manifestPath = path.join(releaseDir, 'release-packages.manifest.json');

const binaries = [
  { id: 'linux-x64-standalone-server-tar-gz', crate: 'sdkwork-aiot-gateway', unix: 'sdkwork-aiot-gateway' },
  { id: 'linux-x64-standalone-server-tar-gz', crate: 'sdkwork-aiot-app-api', unix: 'sdkwork-aiot-app-api' },
  { id: 'linux-x64-standalone-server-tar-gz', crate: 'sdkwork-aiot-admin-api', unix: 'sdkwork-aiot-admin-api' },
];

async function sha256File(filePath) {
  const hash = createHash('sha256');
  await pipeline(createReadStream(filePath), hash);
  return hash.digest('hex');
}

async function createTarGz(sourceDir, outputPath) {
  if (process.platform === 'win32') {
    const result = spawnSync(
      'tar',
      ['-czf', outputPath, '-C', sourceDir, '.'],
      { cwd: root, stdio: 'inherit' },
    );
    if (result.status !== 0) {
      throw new Error(`tar failed with exit code ${result.status}`);
    }
    return;
  }
  const result = spawnSync('tar', ['-czf', outputPath, '-C', sourceDir, '.'], {
    cwd: root,
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    throw new Error(`tar failed with exit code ${result.status}`);
  }
}

mkdirSync(releaseDir, { recursive: true });
const bundleDir = path.join(releaseDir, 'bundle');
mkdirSync(bundleDir, { recursive: true });

for (const binary of binaries) {
  const source = path.join(root, 'target/release', binary.unix);
  if (!existsSync(source)) {
    console.error(
      `[release:package] missing release binary ${source}; run pnpm release:build first`,
    );
    process.exit(1);
  }
  const target = path.join(bundleDir, binary.unix);
  writeFileSync(target, readFileSync(source));
}

const linuxTar = path.join(releaseDir, 'linux-x64-server.tar.gz');
await createTarGz(bundleDir, linuxTar);
const checksum = await sha256File(linuxTar);

const manifest = {
  schemaVersion: 1,
  kind: 'sdkwork.release.packages',
  generatedAt: new Date().toISOString(),
  packages: [
    {
      id: 'linux-x64-standalone-server-tar-gz',
      path: path.relative(root, linuxTar).replaceAll('\\', '/'),
      checksumAlgorithm: 'SHA-256',
      checksum,
      enabled: true,
    },
  ],
};

writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);

const appConfigPath = path.join(root, 'sdkwork.app.config.json');
const appConfig = JSON.parse(readFileSync(appConfigPath, 'utf8'));
const packages = appConfig.artifacts?.installConfig?.packages ?? [];
for (const released of manifest.packages) {
  const target = packages.find((pkg) => pkg.id === released.id);
  if (!target) {
    continue;
  }
  target.checksum = released.checksum;
  target.checksumAlgorithm = released.checksumAlgorithm;
  target.enabled = released.enabled ?? true;
  if (target.metadata?.checksumPendingReleaseBuild) {
    delete target.metadata.checksumPendingReleaseBuild;
  }
}
writeFileSync(appConfigPath, `${JSON.stringify(appConfig, null, 2)}\n`);
console.log(`[release:package] synced checksum into sdkwork.app.config.json`);
console.log(`[release:package] wrote ${path.relative(root, linuxTar)} checksum=${checksum}`);
