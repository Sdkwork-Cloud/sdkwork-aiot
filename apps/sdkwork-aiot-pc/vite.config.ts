import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'node:path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@sdkwork/aiot-pc-console-agent': path.resolve(__dirname, 'packages/sdkwork-aiot-pc-console-agent/src/index.ts'),
      '@sdkwork/aiot-pc-console-device': path.resolve(__dirname, 'packages/sdkwork-aiot-pc-console-device/src/index.ts'),
      '@sdkwork/aiot-pc-console-iot': path.resolve(__dirname, 'packages/sdkwork-aiot-pc-console-iot/src/index.ts'),
      '@sdkwork/aiot-pc-console-voice': path.resolve(__dirname, 'packages/sdkwork-aiot-pc-console-voice/src/index.ts'),
      '@sdkwork/aiot-pc-core': path.resolve(__dirname, 'packages/sdkwork-aiot-pc-core/src/index.ts'),
      '@sdkwork/aiot-app-core': path.resolve(__dirname, '../sdkwork-aiot-shared/packages/sdkwork-aiot-app-core/src/index.ts'),
      '@sdkwork/aiot-app-sdk': path.resolve(__dirname, '../../sdks/sdkwork-aiot-app-sdk/sdkwork-aiot-app-sdk-typescript/src/index.ts'),
      '@sdkwork/core-pc-react': path.resolve(__dirname, '../../../sdkwork-core/sdkwork-core-pc-react/src/index.ts'),
      '@sdkwork/ui-pc-react': path.resolve(__dirname, '../../../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts'),
    },
  },
  server: {
    port: 5175,
  },
});
