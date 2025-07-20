use nu_plugin::Plugin;
use nu_plugin::PluginCommand;

use bollard::Docker;

use crate::commands::*;

pub struct NdockerPlugin {
    pub docker_socket: Docker,
    pub timeout: Option<std::time::Duration>,
}

impl NdockerPlugin {
    pub fn new() -> Self {
        NdockerPlugin {
            docker_socket: Docker::connect_with_local_defaults()
                .expect("Failed to connect to Docker socket"),
            timeout: None,
        }
    }
}

impl Plugin for NdockerPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(image::images::ImagesCommand)]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
