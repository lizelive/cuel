use bollard::exec::{CreateExecOptions, StartExecOptions};
use serde::{Serialize, Deserialize};
use bollard::auth::DockerCredentials;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerRegistryRef{
    path: String,
    auth: Option<DockerCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagOrDigest {
    Tag(String),
    Digest(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerImageName{
    repository: String,
    image: Option<TagOrDigest>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DockerImageRef {
    repository: String,
    tag: Option<String>,
    registry: Option<DockerRegistryRef>,
}

pub enum Image {
    Docker(DockerImageRef),
}