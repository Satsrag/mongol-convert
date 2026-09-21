#!/usr/bin/env node
// Exercise a packaged skill from an unrelated working directory, using only Node.js.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

if (!process.argv[2]) throw new Error('Usage: node test-mongolian-convert.mjs <skill-directory>');
const skill = resolve(process.argv[2]);
const work = mkdtempSync(join(tmpdir(), 'mongol-convert-skill-test-'));
const script = join(skill, 'scripts/convert.mjs');
const configPath = join(skill, 'config.json');
const originalConfig = readFileSync(configPath);
const manifest = JSON.parse(readFileSync(join(skill, 'assets/mongol-convert/manifest.json'), 'utf8'));
const draft = 'ᠤᠯᠤᠰ ᠤᠨ\nᠰᠠᠶᠢᠨ 2026';
const repaired = 'ᠤᠯᠤᠰ\u202fᠤᠨ\nᠰᠠᠶᠢᠨ 2026';

function run(args = [], input = draft, expectedStatus = 0) {
  const result = spawnSync(process.execPath, [script, ...args], {
    input, encoding: 'utf8', cwd: work,
  });
  assert.equal(result.status, expectedStatus, result.stderr || String(result.error));
  return result;
}

try {
  for (const [name, expected] of Object.entries(manifest.files_sha256)) {
    const bytes = readFileSync(join(skill, 'assets/mongol-convert', name));
    assert.equal(createHash('sha256').update(bytes).digest('hex'), expected, name);
  }
  for (const dependency of manifest.dependencies) {
    for (const name of dependency.license_files) {
      assert.ok(readFileSync(join(skill, 'assets/mongol-convert/licenses', name)).length, name);
    }
  }
  assert.equal(run(['--version']).stdout, `mongol-convert ${manifest.mongol_convert_version} (bundled WebAssembly)\n`);
  assert.match(run(['--help']).stdout, /Usage:/);
  // Pin a known UTN57 sequence and ensure stdout receives no extra newline or diagnostics.
  assert.equal(run([], 'ᠰᠠᠶᠢᠨ').stdout, 'ᠰᠠᠢ\u180dᠢ\u180dᠠ\u180c');
  assert.equal(run(['--to', 'menk_letter']).stdout, repaired);

  for (const target of ['utn57', 'menk_shape', 'menk_letter', 'delehi', 'z52', 'zvvnmod', 'utn57_shape']) {
    const reportPath = join(work, `${target}.json`);
    assert.ok(run(['--to', target, '--report', reportPath]).stdout.length);
    const report = JSON.parse(readFileSync(reportPath, 'utf8'));
    assert.equal(report.output_encoding, target);
    assert.equal(report.mongol_convert_version, manifest.mongol_convert_version);
    assert.equal(report.repair_count, 1);
    assert.equal(report.source_encoding, 'menk_letter');
    assert.deepEqual(report.warnings, []);
  }

  writeFileSync(join(work, 'draft.txt'), draft);
  const fileResult = run(['--input', 'draft.txt', '--output', 'result.txt', '--to', 'menk_letter']);
  assert.equal(fileResult.stdout, '');
  assert.equal(readFileSync(join(work, 'result.txt'), 'utf8'), repaired);
  run(['--input', 'draft.txt', '--output', 'draft.txt'], '', 1);
  assert.equal(readFileSync(join(work, 'draft.txt'), 'utf8'), draft);

  writeFileSync(configPath, JSON.stringify({ output_encoding: 'menk_letter' }));
  assert.equal(run().stdout, repaired);
  assert.equal(run(['--to', 'utn57'], 'ᠰᠠᠶᠢᠨ').stdout, 'ᠰᠠᠢ\u180dᠢ\u180dᠠ\u180c');
  assert.equal(JSON.parse(readFileSync(configPath, 'utf8')).output_encoding, 'menk_letter');
  assert.match(run([], 'Сайн байна', 1).stderr, /Cyrillic/);
  assert.equal(run(['--allow-cyrillic', '--to', 'menk_letter'], 'МУ').stdout, 'МУ');
  assert.match(run([], '\ue000', 1).stderr, /private-use/);
  run([], Buffer.from([0xff]), 1);
  run(['--to', 'unknown'], '', 1);
  console.log('Skill package checks passed: provenance, all 7 encodings, suffix repair, files, configuration, and input validation.');
} finally {
  writeFileSync(configPath, originalConfig);
  rmSync(work, { recursive: true, force: true });
}
