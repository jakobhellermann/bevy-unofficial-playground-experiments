use std::ffi::OsStr;
use std::io::Write;
use std::{process::Command, string::FromUtf8Error};

use crate::SourceHash;

const BEVY_BUILDER_CONTAINER: &str = "bevy-builder";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to run `podman {0}`: {1}")]
    PodmanCLI(String, String),
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("invalid utf-8 in podman output: {0}")]
    InvalidUTF8(#[from] FromUtf8Error),
}

fn podman(command: &str, args: &[&OsStr]) -> Result<String, Error> {
    let output = Command::new("podman").arg(command).args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::PodmanCLI(command.to_string(), stderr));
    }
    let mut output = String::from_utf8(output.stdout)?;
    output.truncate(output.trim_end().len());
    Ok(output)
}

fn create_container(image: &str) -> Result<String, Error> {
    podman("create", &[image.as_ref()])
}

fn cp_from_container(
    container_id: &str,
    container_path: &str,
    host_path: &OsStr,
) -> Result<String, Error> {
    let container_arg = format!("{}:{}", container_id, container_path);
    podman("cp", &[container_arg.as_ref(), host_path])
}

fn cp_file_to_container(
    container_id: &str,
    container_path: &str,
    file: &OsStr,
) -> Result<String, Error> {
    let arg = format!("{}:{}", container_id, container_path);
    podman("cp", &[file, arg.as_ref()])
}
fn write_file_to_container(
    container_id: &str,
    container_path: &str,
    source: &str,
) -> Result<(), Error> {
    let mut tempfile = tempfile::NamedTempFile::new()?;
    write!(tempfile, "{}", source)?;

    cp_file_to_container(container_id, container_path, tempfile.path().as_os_str())?;
    Ok(())
}

fn rm_container(container_id: &str) -> Result<(), Error> {
    podman("rm", &[container_id.as_ref()])?;
    Ok(())
}

pub fn compile(source: &str) -> Result<String, Error> {
    let hash = hash_source(source).to_string();

    let container_id = create_container(BEVY_BUILDER_CONTAINER)?;
    write_file_to_container(&container_id, "/project/src/main.rs", source)?;

    podman("start", &["--attach".as_ref(), container_id.as_ref()])?;

    let dir = std::env::temp_dir().join("bevy-playground").join(&hash);
    std::fs::create_dir_all(&dir)?;

    cp_from_container(&container_id, "/project/out/.", dir.as_os_str())?;

    rm_container(&container_id)?;

    Ok(hash)
}

pub fn read_output_js(hash: &str) -> Result<String, Error> {
    let path = std::env::temp_dir()
        .join("bevy-playground")
        .join(&hash.0)
        .join("bevy-project.js");
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
pub fn read_output_wasm(hash: &str) -> Result<Vec<u8>, Error> {
    let path = std::env::temp_dir()
        .join("bevy-playground")
        .join(&hash.0)
        .join("bevy-project_bg.wasm");
    let content = std::fs::read(path)?;
    Ok(content)
}

fn hash_source(source: &str) -> u64 {
    use std::hash::Hasher;

    let mut hasher = fnv::FnvHasher::default();
    hasher.write(source.as_bytes());
    hasher.finish()
}
