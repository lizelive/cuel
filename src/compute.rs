use crate::image::DockerImageRef;
use bollard::*;

trait ComputeProvider {
    fn start(&mut self, image: &DockerImageRef);
    fn terminate(&mut self);
    fn status(&self);
    fn poll();
}