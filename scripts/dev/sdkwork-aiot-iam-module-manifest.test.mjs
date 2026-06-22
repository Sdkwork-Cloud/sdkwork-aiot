#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import assert from 'node:assert/strict';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

test('specs/iam.module.manifest.json satisfies IMF validation rules', () => {
  const result = spawnSync(
    process.execPath,
    [path.join(repoRoot, 'scripts/dev/validate-iam-module-manifest.mjs')],
    { cwd: repoRoot, encoding: 'utf8' },
  );
  assert.equal(
    result.status,
    0,
    `iam module manifest validation failed:\n${result.stdout}\n${result.stderr}`,
  );
});
