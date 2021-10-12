use std::collections::HashMap;

use id::Id;
use image::Image;

mod id;
mod state;
mod task;
mod image;
mod compute;
mod rpc;
mod docker;


struct ProivderMetadata{
    id: Id,
    name: String,
    kind: String,
}

struct ContainerConfig{
    impl_stuff : (),
}

struct StartContainerOperation {
    provider: Option<Id>,
    image: Option<Image>,
}

struct ContainerInfo {
    provider: Id,
    id: Id,
    image: Image,
}

trait Api {
    fn list_providers(&self) -> Vec<ProivderMetadata>;
    fn get_provider(&self, id: Id) -> Option<ProivderMetadata>;
    fn list_container(&self, request: StartContainerOperation) -> ContainerInfo;
    fn start_container(&self, request: StartContainerOperation) -> ContainerInfo;
}


struct DesiredState



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


struct Cuel{
    providers: HashMap<String, Box<dyn Compute>>
}