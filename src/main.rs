mod commands;
mod ndocker;

pub use ndocker::NdockerPlugin;
use nu_plugin::{JsonSerializer, serve_plugin};

fn main() {
    serve_plugin(&NdockerPlugin, JsonSerializer);
}
