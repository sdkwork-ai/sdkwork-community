import { resolveBrowserDistOutDir } from '../../../../sdkwork-specs/tools/browser-dist-layout.mjs';

import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, path.dirname(fileURLToPath(import.meta.url)), '');
  const bootstrapAccessToken = mode === 'development'
    ? process.env.SDKWORK_ACCESS_TOKEN ?? env.SDKWORK_ACCESS_TOKEN ?? ''
    : '';
  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(bootstrapAccessToken),
    },
    plugins: [react(), tailwindcss()],
    server: {
      port: 3000,
      host: true
    },
    build: {
      outDir: resolveBrowserDistOutDir(resolveViteEnvironment(mode, env)),
      sourcemap: mode !== 'production'
    }
  };
});
