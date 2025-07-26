mod commands;
mod ndocker;
mod utils;

pub use ndocker::NdockerPlugin;
use nu_plugin::{JsonSerializer, serve_plugin};

fn main() {
    serve_plugin(&NdockerPlugin::new(), JsonSerializer);
}
