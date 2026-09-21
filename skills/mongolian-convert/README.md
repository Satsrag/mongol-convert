# mongolian-convert

An agent skill for Cyrillic Mongolian to traditional Mongolian conversion. The agent
prepares a MenkLetter draft, then the bundled mongol-convert WebAssembly converter repairs suffix
separators and produces the requested encoding. The default is UTN57.

## Install

Download `mongolian-convert-<version>.zip` from the
[mongol-convert releases](https://github.com/Satsrag/mongol-convert/releases). Extract the
`mongolian-convert/` folder into your agent's skills directory, or use its skill ZIP
import if supported. Keep the whole folder, including `assets/` and `scripts/`.

The agent needs command execution and Node.js 18.20 or newer. The downloaded package
includes the converter, so using it requires no Rust, npm install, API key, or network
access for the mongol-convert step.

Invoke `$mongolian-convert` with Cyrillic Mongolian text or a text file. You can request
`utn57`, `menk_shape`, `menk_letter`, `delehi`, `z52`, `zvvnmod`, or `utn57_shape`.
To change the persistent default, edit `output_encoding` in `config.json`.

Check the bundled converter from the extracted folder:

```sh
node scripts/convert.mjs --version
```

The script expects an already transliterated MenkLetter draft, not Cyrillic input.
See [SKILL.md](SKILL.md) for the full workflow. MenkShape and other legacy encodings
need a compatible font to display correctly.

## Source and packaging

The source lives in `skills/mongolian-convert/` in the mongol-convert repository. Generated WASM,
JavaScript bindings, dependency licenses, and the provenance manifest are added during
packaging; GitHub's automatic source archives do not contain the ready-to-run bundle.
See [DISTRIBUTION.md](https://github.com/Satsrag/mongol-convert/blob/main/DISTRIBUTION.md#agent-skill)
for build commands.
