use nu_plugin::Plugin;
use nu_plugin::PluginCommand;

use crate::commands::*;

pub struct NdockerPlugin;

impl Plugin for NdockerPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
