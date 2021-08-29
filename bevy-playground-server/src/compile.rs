use std::ffi::OsStr;
use std::io::Write;
use std::string::FromUtf8Error;
use tokio::process::Command;
use tracing::{span, trace, Level};

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

async fn podman(command: &str, args: &[&OsStr]) -> Result<String, Error> {
    let output = Command::new("podman")
        .arg(command)
        .args(args)
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(Error::PodmanCLI(command.to_string(), stderr));
    }
    let mut output = String::from_utf8(output.stdout)?;
    output.truncate(output.trim_end().len());

    Ok(output)
}

async fn create_container(image: &str) -> Result<String, Error> {
    podman("create", &[image.as_ref()]).await
}

async fn cp_from_container(
    container_id: &str,
    container_path: &str,
    host_path: &OsStr,
) -> Result<String, Error> {
    let container_arg = format!("{}:{}", container_id, container_path);
    podman("cp", &[container_arg.as_ref(), host_path]).await
}

async fn cp_file_to_container(
    container_id: &str,
    container_path: &str,
    file: &OsStr,
) -> Result<String, Error> {
    let arg = format!("{}:{}", container_id, container_path);
    podman("cp", &[file, arg.as_ref()]).await
}
async fn write_file_to_container(
    container_id: &str,
    container_path: &str,
    source: &str,
) -> Result<(), Error> {
    let mut tempfile = tempfile::NamedTempFile::new()?;
    write!(tempfile, "{}", source)?;

    cp_file_to_container(container_id, container_path, tempfile.path().as_os_str()).await?;
    Ok(())
}

async fn rm_container(container_id: &str) -> Result<(), Error> {
    podman("rm", &[container_id.as_ref()]).await?;
    Ok(())
}

async fn start_container_attach(container_id: &str) -> Result<(), Error> {
    podman("start", &["--attach".as_ref(), container_id.as_ref()]).await?;
    Ok(())
}

#[derive(serde::Serialize)]
#[serde(tag = "status")]
#[serde(rename_all = "lowercase")]
pub enum CompilationResult {
    Success { id: String },
    Error { msg: String },
}

pub async fn compile(source: &str) -> Result<CompilationResult, Error> {
    let span = span!(Level::DEBUG, "compile source");
    let _enter = span.enter();

    let hash = hash_source(source).to_string();

    let container_id = create_container(BEVY_BUILDER_CONTAINER).await?;
    trace!(target: "created container", id = ?container_id);
    write_file_to_container(&container_id, "/project/src/main.rs", source).await?;
    trace!(parent: &span, container_id = container_id.as_str());

    let result = (|| async {
        match start_container_attach(&container_id).await {
            Ok(_) => {}
            Err(Error::PodmanCLI(_, msg)) => return Ok(CompilationResult::Error { msg }),
            Err(e) => return Err(e),
        };

        trace!(parent: &span, "compilation finished");

        let dir = std::env::temp_dir().join("bevy-playground").join(&hash);
        tokio::fs::create_dir_all(&dir).await?;

        cp_from_container(&container_id, "/project/out/.", dir.as_os_str()).await?;
        trace!(parent: &span, "wrote files");

        Ok(CompilationResult::Success { id: hash })
    })()
    .await;

    rm_container(&container_id).await?;
    trace!(parent: &span, "removed container");

    result
}

pub async fn read_output_js(hash: &SourceHash) -> Result<String, Error> {
    let path = std::env::temp_dir()
        .join("bevy-playground")
        .join(&hash.0)
        .join("bevy-project.js");
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}
pub async fn read_output_wasm(hash: &SourceHash) -> Result<Vec<u8>, Error> {
    let path = std::env::temp_dir()
        .join("bevy-playground")
        .join(&hash.0)
        .join("bevy-project_bg.wasm");
    let content = tokio::fs::read(path).await?;
    Ok(content)
}

fn hash_source(source: &str) -> u64 {
    use std::hash::Hasher;

    let mut hasher = fnv::FnvHasher::default();
    hasher.write(source.as_bytes());
    hasher.finish()
}
