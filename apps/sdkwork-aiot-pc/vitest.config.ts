import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { configDefaults, defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

const workspaceRoot = path.dirname(fileURLToPath(import.meta.url));
const workspaceNodeModules = path.join(workspaceRoot, "node_modules");
const workspacePnpmStore = path.join(workspaceNodeModules, ".pnpm");

const sharedUiRuntimePackages: string[] = [];

function packageStorePrefix(packageName: string): string {
  const [scope, name] = packageName.startsWith("@")
    ? packageName.split("/")
    : ["", packageName];
  return scope ? `${scope}+${name}@` : `${name}@`;
}

function resolveWorkspacePackage(packageName: string): string {
  const directPath = path.join(workspaceNodeModules, packageName);
  if (existsSync(directPath)) {
    return directPath;
  }

  const pnpmEntry = readdirSync(workspacePnpmStore)
    .filter((entry) => {
      const packagePath = path.join(workspacePnpmStore, entry, "node_modules", packageName);
      return entry.startsWith(packageStorePrefix(packageName)) || existsSync(packagePath);
    })
    .sort()
    .at(-1);

  if (!pnpmEntry) {
    throw new Error(`Unable to resolve ${packageName} from ${workspacePnpmStore}`);
  }

  return path.join(workspacePnpmStore, pnpmEntry, "node_modules", packageName);
}

function loadWorkspaceRuntimeAliases() {
  return sharedUiRuntimePackages.map((packageName) => ({
    find: packageName,
    replacement: resolveWorkspacePackage(packageName),
  }));
}

function loadTsconfigAliases() {
  const tsconfigBasePath = path.join(workspaceRoot, "tsconfig.base.json");
  const tsconfigBase = JSON.parse(readFileSync(tsconfigBasePath, "utf8"));
  const pathMappings = tsconfigBase?.compilerOptions?.paths ?? {};
  const runtimeAliases = new Set([
    "react",
    "react-dom",
    "react/jsx-runtime",
    "react/jsx-dev-runtime",
  ]);

  return Object.entries(pathMappings).flatMap(([find, replacements]) => {
    if (runtimeAliases.has(find)) {
      return [];
    }

    const replacement = Array.isArray(replacements) ? replacements[0] : undefined;
    if (typeof replacement !== "string") {
      return [];
    }

    return [{
      find: find.endsWith("/*") ? find.slice(0, -2) : find,
      replacement: path.resolve(
        workspaceRoot,
        replacement.endsWith("/*") ? replacement.slice(0, -2) : replacement,
      ),
    }];
  }).sort((left, right) => right.find.length - left.find.length);
}

export default defineConfig({
  root: workspaceRoot,
  plugins: [react()],
  resolve: {
    alias: [
      {
        find: "react",
        replacement: path.join(workspaceNodeModules, "react"),
      },
      {
        find: "react-dom",
        replacement: path.join(workspaceNodeModules, "react-dom"),
      },
      {
        find: "lucide-react",
        replacement: path.join(workspaceNodeModules, "lucide-react"),
      },
      ...loadWorkspaceRuntimeAliases(),
      ...loadTsconfigAliases(),
    ],
    dedupe: [
      "react",
      "react-dom",
      "lucide-react",
      ...sharedUiRuntimePackages,
    ],
  },
  test: {
    exclude: [...configDefaults.exclude],
    environment: "jsdom",
    include: [
      "packages/**/*.test.ts",
      "packages/**/*.test.tsx",
      "sdks/**/*.test.ts",
    ],
    setupFiles: [path.join(workspaceRoot, "vitest.setup.ts")],
  },
  server: {
    fs: {
      allow: [workspaceRoot],
    },
  },
});
