use bollard::exec::{CreateExecOptions, StartExecOptions};
use serde::{Serialize, Deserialize};
use bollard::auth::DockerCredentials;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerRegistryRef{
    path: String,
    auth: Option<DockerCredentials>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum TagOrDigest {
    Tag(String),
    Digest(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DcokerImageName{
    repository: String,
    image: Option<TagOrDigest>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerImageRef {
    repository: String,
    tag: Option<String>,
    registry: Option<DockerRegistryRef>,
}

use bollard::Docker;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    let images = &docker.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await?;

    for image in images {
        println!("-> {:?}", image);
    }

    let config = CreateExecOptions {
        cmd: Some(vec!["ps", "-ef"]),
        attach_stdout: Some(true),
        ..Default::default()
        };
        
    let exec = docker.create_exec("help", config).await?;

    let start_options = StartExecOptions{
        detach: true,
        ..Default::default()
    };
    let running = docker.start_exec(&exec.id, start_options).await?;
    running.
    Ok(())
}