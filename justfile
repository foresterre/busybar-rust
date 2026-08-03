default:
    @just --list

check:
    cargo check --workspace

test:
    cargo test --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace

msrv-verify:
    cargo msrv verify --manifest-path crates/busylib/Cargo.toml
    cargo msrv verify --manifest-path crates/busybar/Cargo.toml

msrv-find:
    cargo msrv find --min 1.85 --manifest-path crates/busylib/Cargo.toml --write-msrv -- cargo check -p busylib
    cargo msrv find --min 1.85 --manifest-path crates/busybar/Cargo.toml --write-msrv -- cargo check -p busybar

precommit: fmt check clippy test
    just msrv-verify || { just msrv-find; exit 1; }

book:
    mdbook serve --open book

bump version:
    #!/usr/bin/env python3
    import datetime, json, pathlib, re, subprocess, sys

    manifest = pathlib.Path("Cargo.toml")
    text = manifest.read_text()
    old = re.search(r'^version = "(.+)"$', text, re.M).group(1)
    major, minor, patch = (int(version) for version in old.split("."))

    bumped = {
        "major": f"{major + 1}.0.0",
        "minor": f"{major}.{minor + 1}.0",
        "patch": f"{major}.{minor}.{patch + 1}",
    }
    if "{{ version }}" not in bumped:
        sys.exit("bump expects major, minor or patch, got {{ version }}")
    new = bumped["{{ version }}"]

    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )

    changelogs = {
        crate["name"]: pathlib.Path(crate["manifest_path"]).with_name("CHANGELOG.md")
        for crate in metadata["packages"]
        if crate["publish"] != []
    }

    # Check that all publishable crates have changelogs
    missing = [name for name, changelog in changelogs.items() if not changelog.is_file()]
    if missing:
        sys.exit(f"crates without a changelog: {', '.join(missing)}")

    # Check that the documented dependency versions are the ones we are bumping
    docs = [pathlib.Path("README.md"), pathlib.Path("book/src/api/index.md")]
    dependency = f'busylib = "{old}"'
    stale = [str(doc) for doc in docs if dependency not in doc.read_text()]
    if stale:
        sys.exit(f"docs without a `{dependency}` dependency: {', '.join(stale)}")

    # Update Cargo manifest versions
    manifest.write_text(text.replace(f'version = "{old}"', f'version = "{new}"'))

    # Update changelogs
    heading = f"## {new} - {datetime.date.today()}"
    for changelog in changelogs.values():
        changelog.write_text(
            changelog.read_text().replace(
                "## Unreleased\n",
                f"## Unreleased\n\nNo notable changes.\n\n{heading}\n",
                1,
            )
        )

    # Update the documented dependency versions
    for doc in docs:
        doc.write_text(doc.read_text().replace(dependency, f'busylib = "{new}"'))

    subprocess.run(["cargo", "update", "--workspace", "--quiet"], check=True)
    print(f"bumped {old} to {new}")

proto-update *dir:
    cargo run -p busylib-proto-build -- {{dir}}
    cargo fmt --all
