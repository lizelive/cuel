use crate::{id::Id, image::DockerImageRef};
use bollard::*;

pub trait Task {
    
}

pub trait Container {
    type Task : Sized + Task;
    fn id(&self) -> Id;
    fn kill(self);
    fn status(self);
    fn list(&self) -> Vec<Box<dyn Task>>;
}

struct ComputeState;

struct Compute{
    id: Id,
    provider: Id,
    state: ComputeState,
}

pub trait ComputeProvider {
    fn id(&self) -> Id;
    fn name(&self) -> String;
    type Container: Sized + Container;
    fn list_containers(&self) -> Vec<Self::Container>;
    fn start(&mut self, image: &DockerImageRef) -> Self::Container;
    fn terminate(&mut self);
    fn status(&self);
    fn poll();
}

pub trait Cuel {
    fn list_computes() -> Vec<ComputeProvider>;
}