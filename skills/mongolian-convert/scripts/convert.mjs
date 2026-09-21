#!/usr/bin/env node
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';
import { initSync, translate_with_options, version } from '../assets/mongol-convert/mongol_convert.mjs';

const skillRoot = new URL('../', import.meta.url);
const targets = ['utn57', 'menk_shape', 'menk_letter', 'delehi', 'z52', 'zvvnmod', 'utn57_shape'];
const aliases = new Map([['menkshape', 'menk_shape'], ['menkletter', 'menk_letter']]);

async function main() {
  const { values } = parseArgs({
    options: {
      input: { type: 'string', short: 'i', default: '-' },
      output: { type: 'string', short: 'o' },
      to: { type: 'string' },
      report: { type: 'string' },
      'allow-cyrillic': { type: 'boolean', default: false },
      help: { type: 'boolean', short: 'h' },
      version: { type: 'boolean' },
    },
    allowPositionals: false,
  });
  if (values.help) {
    process.stdout.write(`Convert a MenkLetter draft with the bundled mongol-convert converter and suffix repair enabled.
Usage: node convert.mjs --input draft.txt [--to encoding] [--output result.txt] [--report report.json]
Targets: ${targets.join(', ')}
Default target: config.json output_encoding (initially utn57).
Input defaults to stdin (-); output defaults to stdout. Diagnostics use stderr.
--allow-cyrillic permits intentionally preserved Cyrillic fragments.
This script does not transliterate Cyrillic Mongolian; prepare the MenkLetter draft first.
`);
    return;
  }

  initSync({ module: await readFile(new URL('assets/mongol-convert/mongol_convert_bg.wasm', skillRoot)) });
  const converterVersion = version();
  const manifest = JSON.parse(await readFile(new URL('assets/mongol-convert/manifest.json', skillRoot), 'utf8'));
  if (converterVersion !== manifest.mongol_convert_version) {
    throw new Error(`Expected bundled mongol-convert ${manifest.mongol_convert_version}, found ${converterVersion}.`);
  }
  if (values.version) {
    process.stdout.write(`mongol-convert ${converterVersion} (bundled WebAssembly)\n`);
    return;
  }

  const config = JSON.parse(await readFile(new URL('config.json', skillRoot), 'utf8'));
  const requested = values.to ?? config.output_encoding;
  const normalized = typeof requested === 'string' ? requested.trim().toLowerCase() : '';
  const target = aliases.get(normalized) ?? normalized;
  if (!targets.includes(target)) {
    throw new Error(`Unsupported output encoding ${JSON.stringify(requested)}; choose ${targets.join(', ')}.`);
  }

  const inputPath = values.input === '-' ? null : resolve(values.input);
  const outputPath = values.output && values.output !== '-' ? resolve(values.output) : null;
  const reportPath = values.report ? resolve(values.report) : null;
  const paths = [inputPath, outputPath, reportPath].filter(Boolean);
  if (new Set(paths).size !== paths.length) throw new Error('Input, output and report must be different files.');
  const configPath = fileURLToPath(new URL('config.json', skillRoot));
  if (outputPath === configPath || reportPath === configPath) throw new Error('Output cannot overwrite skill configuration.');

  let bytes;
  if (inputPath) {
    bytes = await readFile(inputPath);
  } else {
    const chunks = [];
    for await (const chunk of process.stdin) chunks.push(chunk);
    bytes = Buffer.concat(chunks);
  }
  const input = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  const hasCyrillic = /\p{Script=Cyrillic}/u.test(input);
  if (hasCyrillic && !values['allow-cyrillic']) {
    throw new Error('Cyrillic remains in the input. First transliterate to MenkLetter; use --allow-cyrillic only for intentionally preserved fragments.');
  }
  if (/[\uE000-\uF8FF\u{F0000}-\u{FFFFD}\u{100000}-\u{10FFFD}]/u.test(input)) {
    throw new Error('Input contains private-use glyph codes; this script expects a MenkLetter letter draft.');
  }

  const converted = translate_with_options('menk_letter', target, input, true);
  let text, repairs, warnings;
  try {
    text = converted.text;
    repairs = converted.repairs;
    warnings = converted.warnings;
  } finally {
    converted.free();
  }
  const report = {
    mongol_convert_version: converterVersion,
    source_encoding: 'menk_letter',
    output_encoding: target,
    repair_suffix_separators: true,
    repair_count: repairs.length,
    repairs,
    warnings,
    preserved_cyrillic: hasCyrillic,
  };
  if (reportPath) await writeFile(reportPath, JSON.stringify(report, null, 2) + '\n', 'utf8');
  if (outputPath) await writeFile(outputPath, text, 'utf8');
  else process.stdout.write(text);
  process.stderr.write(`mongol-convert ${converterVersion}: menk_letter -> ${target}; ${repairs.length} suffix separator(s) repaired.\n`);
  for (const warning of warnings) process.stderr.write(`mongol-convert warning: ${warning}\n`);
}

main().catch(error => {
  process.stderr.write(`mongolian-convert: ${error.message ?? error}\n`);
  process.exitCode = 1;
});
