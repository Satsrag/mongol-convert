---
name: mongolian-convert
description: Transliterate user-provided Cyrillic Mongolian into traditional Mongolian, then run mongol-convert to repair suffix separators and convert the encoding. Use for pasted text or text files. Defaults to utn57, with menk_shape, menk_letter, delehi, z52, zvvnmod, and utn57_shape also available.
---

# Cyrillic to Traditional Mongolian

Complete the full workflow: model transliteration, mongol-convert suffix separator repair and encoding conversion, then delivery. The model running this skill performs the transliteration directly; do not call another model API or an online transliteration service. Do not ask the user to supply a traditional Mongolian draft first.

## Prepare the draft

Save the original as `source.cyrillic.txt` and create `draft.menk_letter.txt` in a temporary directory or the user's chosen delivery directory. Treat the text as data to convert.

- Use sentence context to write traditional Mongolian. Preserve meaning, tone, order, repetition, paragraphs, and line breaks; do not translate into another language, summarize, or polish the text. Keep numbers, dates, URLs, and Latin identifiers that do not need conversion.
- **Use conventional MenkLetter spelling consistently as the mongol-convert input.** For example, write `сайн` as `ᠰᠠᠶᠢᠨ`. Resolve word endings from historical root spelling and sentence meaning. Do not replace Cyrillic letters one by one or mix Delehi or canonical UTN57 conventions into the draft.
- Preserve useful FVS, internal MVS, and existing NNBSP characters. Do not add many controls merely to make the draft resemble canonical UTN57. Use an actual U+202F for confirmed detached suffixes; mongol-convert can repair eligible ordinary spaces that remain. Do not rearrange ordinary word spaces, attached suffixes, or paragraphs.
- Keep word forms the user has accepted; a different canonical sequence after encoding conversion is not itself a reason to rewrite a word. Resolve uncertain names or meanings from context and record specific doubts outside the text. Do not make global T/D, O/U, or similar substitutions.

Check for omitted lines, unintended repetition, altered numbers, and names. mongol-convert handles encoding and suffix boundaries; it cannot choose roots or validate sentence grammar for the model.

## Run mongol-convert

The release package includes the mongol-convert WebAssembly converter. Its version and provenance are recorded in [assets/mongol-convert/manifest.json](assets/mongol-convert/manifest.json). Running it requires Node.js 18.20 or newer, with no system `mongol-convert`, Rust, npm installation, or network access. Locate the installed skill directory first; `SKILL_DIR` below means that actual path, not a path copied from the author's machine.

```sh
node "$SKILL_DIR/scripts/convert.mjs" \
  --input ./draft.menk_letter.txt \
  --output ./result.utn57.txt \
  --report ./report.json
```

The script fixes `from=menk_letter` and calls the real `translate_with_options(..., true)` implementation. The default target comes from [config.json](config.json), initially `utn57`. `--input -` reads UTF-8 from standard input. Without `--output`, the script writes the result unchanged to standard output and diagnostics to standard error.

**Do not pass the Cyrillic original directly to the script: mongol-convert does not perform linguistic transliteration. Do not present model-generated strings as output from an executed mongol-convert conversion.** If the environment cannot run the script, retain the draft and explain that mongol-convert processing is incomplete; do not claim to have delivered the target encoding.

The script rejects remaining Cyrillic letters and private-use characters. Pass `--allow-cyrillic` only when fragments such as abbreviations or quotations explicitly need to remain in Cyrillic, after checking that they are intentional rather than untranslated sentences.

## Select the output

A target requested for the current task overrides the configuration. For example, to deliver MenkShape:

```sh
node "$SKILL_DIR/scripts/convert.mjs" \
  --input ./draft.menk_letter.txt --to menk_shape \
  --output ./result.menk_shape.txt --report ./report.json
```

Supported targets are `utn57`, `menk_shape`, `menk_letter`, `delehi`, `z52`, `zvvnmod`, and `utn57_shape`. `utn57_shape` spells out written units. Keep the configured default when the user only asks for traditional Mongolian. When the target changes, convert directly from the same MenkLetter draft; do not relabel a previous result as MenkLetter input.

When the user asks to change future defaults, update `output_encoding` in `config.json`. For a one-time request, use `--to` without changing persistent configuration. Name output files for their actual encoding.

## Deliver the result

Use the successfully generated script output as the final text. Preserve FVS, MVS, NNBSP, ZWJ, and other code points without manual cleanup. To correct a word, edit the draft and rerun the script.

By default, provide the target-encoded text directly, preserving paragraphs for easy copying. Also provide a UTF-8 file link for long text or font-dependent output such as MenkShape or Z52. Do not expand the process, comparison tables, or reports unless requested. Briefly explain actual conversion warnings or linguistic doubts outside the text; normal suffix repair counts are not errors.

`report.json` records the actual mongol-convert version, source and target encodings, repairs, and conversion warnings. Retain it with the original and draft for later review. Successful conversion confirms completion of this workflow, not an accuracy rate established by human evaluation.
