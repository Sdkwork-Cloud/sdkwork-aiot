import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import path from 'node:path';

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@sdkwork/aiot-h5-core': path.resolve(__dirname, 'packages/sdkwork-aiot-h5-core/src/index.ts'),
      '@sdkwork/aiot-h5-console-agent': path.resolve(__dirname, 'packages/sdkwork-aiot-h5-console-agent/src/index.ts'),
      '@sdkwork/aiot-h5-console-device': path.resolve(__dirname, 'packages/sdkwork-aiot-h5-console-device/src/index.ts'),
      '@sdkwork/aiot-h5-console-iot': path.resolve(__dirname, 'packages/sdkwork-aiot-h5-console-iot/src/index.ts'),
      '@sdkwork/aiot-h5-console-voice': path.resolve(__dirname, 'packages/sdkwork-aiot-h5-console-voice/src/index.ts'),
      '@sdkwork/aiot-app-core': path.resolve(__dirname, '../sdkwork-aiot-shared/packages/sdkwork-aiot-app-core/src/index.ts'),
      '@sdkwork/aiot-app-sdk': path.resolve(__dirname, '../../sdks/sdkwork-aiot-app-sdk/sdkwork-aiot-app-sdk-typescript/src/index.ts'),
    },
  },
  server: { port: 5176 },
});
