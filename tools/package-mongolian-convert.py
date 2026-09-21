#!/usr/bin/env python3
"""Package the skill source with web-target mongol-convert bindings and dependency licenses."""

import argparse
import hashlib
import json
import subprocess
import tempfile
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SKILL = ROOT / "skills" / "mongolian-convert"
SOURCE_FILES = (
    "SKILL.md",
    "README.md",
    "agents/openai.yaml",
    "config.json",
    "scripts/convert.mjs",
)


def command(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def runtime_packages(metadata):
    """Walk normal WASM dependencies, excluding build scripts and proc macros."""
    packages = {p["id"]: p for p in metadata["packages"]}
    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
    pending = [p["id"] for p in packages.values() if p["name"] == "mongol-convert-wasm"]
    seen = set()
    while pending:
        package_id = pending.pop()
        package = packages[package_id]
        if package_id in seen or any(
            "proc-macro" in target["kind"] for target in package["targets"]
        ):
            continue
        seen.add(package_id)
        for dependency in nodes[package_id]["deps"]:
            if any(kind["kind"] is None for kind in dependency["dep_kinds"]):
                pending.append(dependency["pkg"])
    return sorted((packages[p] for p in seen), key=lambda p: (p["name"], p["version"]))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--wasm-dir", type=Path, required=True,
        help="Web-target bindings containing mongol_convert.js and mongol_convert_bg.wasm",
    )
    parser.add_argument("--out-dir", type=Path, default=ROOT / "target" / "skills")
    args = parser.parse_args()

    metadata = json.loads(command(
        "cargo", "metadata", "--locked", "--format-version", "1",
        "--filter-platform", "wasm32-unknown-unknown",
    ))
    packages = runtime_packages(metadata)
    version = next(p["version"] for p in packages if p["name"] == "mongol-convert")
    commit = command("git", "rev-parse", "HEAD")
    files = {name: (SKILL / name).read_bytes() for name in SOURCE_FILES}
    files["assets/mongol-convert/mongol_convert.mjs"] = (args.wasm_dir / "mongol_convert.js").read_bytes()
    files["assets/mongol-convert/mongol_convert_bg.wasm"] = (args.wasm_dir / "mongol_convert_bg.wasm").read_bytes()
    files["assets/mongol-convert/licenses/mongol-convert-LICENSE.txt"] = (ROOT / "LICENSE").read_bytes()

    dependencies = []
    for package in packages:
        if package["id"] in metadata["workspace_members"]:
            continue
        crate = Path(package["manifest_path"]).parent
        licenses = sorted(p for p in crate.glob("LICENSE*") if p.is_file())
        if package.get("license_file"):
            license_file = crate / package["license_file"]
            if license_file not in licenses:
                licenses.append(license_file)
        if not licenses:
            raise RuntimeError(f"No license files found for {package['name']}")
        names = []
        for license_file in licenses:
            name = f"{package['name']}-{package['version']}-{license_file.name}"
            files[f"assets/mongol-convert/licenses/{name}"] = license_file.read_bytes()
            names.append(name)
        dependencies.append({
            "name": package["name"],
            "version": package["version"],
            "license": package["license"],
            "license_files": names,
        })

    manifest = {
        "mongol_convert_version": version,
        "source": f"https://github.com/Satsrag/mongol-convert/tree/{commit}",
        "source_commit": commit,
        "files_sha256": {
            name.removeprefix("assets/mongol-convert/"): hashlib.sha256(content).hexdigest()
            for name, content in sorted(files.items()) if name.startswith("assets/mongol-convert/")
        },
        "dependencies": dependencies,
    }
    files["assets/mongol-convert/manifest.json"] = (json.dumps(manifest, indent=2) + "\n").encode()

    # Check the assembled package independently of the source checkout before publishing it.
    with tempfile.TemporaryDirectory(prefix="mongol-convert-skill-") as temp:
        staged = Path(temp) / "mongolian-convert"
        for name, content in files.items():
            destination = staged / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(content)
        subprocess.run(
            ["node", str(ROOT / "tools" / "test-mongolian-convert.mjs"), str(staged)],
            cwd=temp, check=True,
        )

    args.out_dir.mkdir(parents=True, exist_ok=True)
    archive = args.out_dir / f"mongolian-convert-{version}.zip"
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as bundle:
        for name, content in sorted(files.items()):
            # Fixed timestamps and permissions keep identical inputs reproducible.
            entry = zipfile.ZipInfo(f"mongolian-convert/{name}", (1980, 1, 1, 0, 0, 0))
            entry.create_system = 3
            entry.external_attr = 0o100644 << 16
            entry.compress_type = zipfile.ZIP_DEFLATED
            bundle.writestr(entry, content)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    archive.with_suffix(".zip.sha256").write_text(f"{checksum}  {archive.name}\n")
    print(f"Packaged {archive} ({archive.stat().st_size:,} bytes)")


if __name__ == "__main__":
    main()
