import fs from 'node:fs';
import path from 'node:path';

import { REPO_ROOT } from './aiot-topology.mjs';

export const ENV_DEVICE_DB_PATH = 'SDKWORK_AIOT_DEVICE_DB_PATH';
export const ENV_OUTBOX_DISPATCHER_ENABLED = 'SDKWORK_AIOT_OUTBOX_DISPATCHER_ENABLED';

const EDGE_PROCESS_ID = 'edge.device-ingress';
const APP_PROCESS_ID = 'application.app-http';
const ADMIN_PROCESS_ID = 'application.admin-http';

export function assertSupportedDevDatabaseEngine(databaseEngine) {
  if (databaseEngine === 'sqlite' || databaseEngine === 'postgres') {
    return databaseEngine;
  }
  throw new Error(
    `unsupported --database ${databaseEngine}; expected sqlite or postgres`,
  );
}

export function resolveDevDeviceDatabasePath(databaseEngine = 'sqlite') {
  if (databaseEngine !== 'sqlite') {
    return null;
  }
  return path.join(REPO_ROOT, '.sdkwork', 'dev', 'aiot-device.db');
}

export function mergePostgresDeviceDatabaseEnv(baseEnv) {
  throw new Error(
    'postgres dev orchestration is not ready: synchronous device repositories still require --database sqlite until SDKWORK_AIOT_DEVICE_DATABASE postgres pools land in sdkwork-aiot-storage-sqlx',
  );
}

export function mergeDeviceDatabaseEnv(baseEnv, { databaseEngine = 'sqlite' } = {}) {
  const engine = assertSupportedDevDatabaseEngine(databaseEngine);
  if (engine === 'postgres') {
    return mergePostgresDeviceDatabaseEnv(baseEnv);
  }
  const env = { ...baseEnv };
  const deviceDbPath = resolveDevDeviceDatabasePath(engine);
  if (deviceDbPath) {
    fs.mkdirSync(path.dirname(deviceDbPath), { recursive: true });
    env[ENV_DEVICE_DB_PATH] = deviceDbPath;
  }
  return env;
}

export function mergeProcessRuntimeEnv(processSpec, baseEnv) {
  const env = { ...baseEnv };
  if (processSpec.id === EDGE_PROCESS_ID) {
    env[ENV_OUTBOX_DISPATCHER_ENABLED] = '1';
    return env;
  }
  if (processSpec.id === APP_PROCESS_ID || processSpec.id === ADMIN_PROCESS_ID) {
    env[ENV_OUTBOX_DISPATCHER_ENABLED] = '0';
    return env;
  }
  return env;
}
