use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const REPOSITORY: &str = "https://github.com/busy-app/busybar-protobuf";
const REVISION: &str = "dba670e2ddb5cda511af997ca5fcb1254e90917f";

const PROTOS: [&str; 15] = [
    "error.proto",
    "frame.proto",
    "input.proto",
    "state.proto",
    "timer.proto",
    "state/audio.proto",
    "state/ble.proto",
    "state/brightness.proto",
    "state/device_name.proto",
    "state/matter.proto",
    "state/power.proto",
    "state/timezone.proto",
    "state/update.proto",
    "state/wifi.proto",
    "util/json.proto",
];

fn main() -> ExitCode {
    match run() {
        Ok(destination) => {
            println!("wrote {}", destination.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<PathBuf, Box<dyn Error>> {
    let crates = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("the manifest should sit inside crates/")?
        .to_path_buf();

    let workspace = crates.parent().ok_or("crates/ should have a parent")?;

    let schemas = match std::env::args().nth(1) {
        Some(directory) => PathBuf::from(directory),
        None => checkout(&workspace.join("target/busybar-protobuf"))?,
    };

    let out = workspace.join("target/busylib-proto-build");
    std::fs::create_dir_all(&out)?;

    let generated = generate(&schemas, &out)?;
    let destination = crates.join("busylib-proto/src/generated/protos.rs");

    std::fs::write(&destination, generated)?;

    Ok(destination)
}

fn checkout(directory: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let target = directory.to_string_lossy().into_owned();

    if !directory.join(".git").exists() {
        if let Some(parent) = directory.parent() {
            std::fs::create_dir_all(parent)?;
        }

        run_command("git", &["clone", "--quiet", REPOSITORY, &target])?;
    }

    run_command("git", &["-C", &target, "fetch", "--quiet", "origin"])?;
    run_command("git", &["-C", &target, "checkout", "--quiet", REVISION])?;

    Ok(directory.to_path_buf())
}

fn run_command(program: &str, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new(program).args(arguments).status()?;

    if !status.success() {
        return Err(format!("`{program}` failed with {status}").into());
    }

    Ok(())
}

fn generate(schemas: &Path, out: &Path) -> Result<String, Box<dyn Error>> {
    let files: Vec<PathBuf> = PROTOS.iter().map(|proto| schemas.join(proto)).collect();

    // just to be sure :)
    for file in &files {
        if !file.exists() {
            return Err(format!("{} does not hold the schemas", schemas.display()).into());
        }
    }

    let descriptors = protox::compile(&files, [schemas])?;

    let mut config = prost_build::Config::new();
    config.out_dir(out);
    config.include_file("_protos.rs");
    config.type_attribute(".", "#[derive(::serde::Serialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"snake_case\")]");
    config.field_attribute(
        "BSB_Frame.Frame.data",
        "#[serde(serialize_with = \"crate::serde_util::base64_bytes::serialize\")]",
    );
    config.compile_fds(descriptors)?;

    inline(out)
}

fn inline(out: &Path) -> Result<String, Box<dyn Error>> {
    let root = std::fs::read_to_string(out.join("_protos.rs"))?;
    let mut inlined = String::new();

    for line in root.lines() {
        match included_module(line) {
            Some(module) => inlined.push_str(&std::fs::read_to_string(out.join(module))?),
            None => inlined.push_str(line),
        }

        inlined.push('\n');
    }

    Ok(inlined)
}

fn included_module(line: &str) -> Option<&str> {
    let arguments = line.trim().strip_prefix("include!(")?;

    arguments
        .split('"')
        .skip(1)
        .step_by(2)
        .last()
        .map(|module| module.trim_start_matches('/'))
}
