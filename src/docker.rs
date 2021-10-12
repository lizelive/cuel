use bollard::{Docker, exec::{CreateExecOptions, StartExecOptions}, image::ListImagesOptions};
use tokio::test;

#[tokio::test]
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
    let running = docker.start_exec(&exec.id, Some(start_options)).await?;
    running.
    Ok(())
}