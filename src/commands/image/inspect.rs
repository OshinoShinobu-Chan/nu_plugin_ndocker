//! This module is for command `ndocker image inspect`.

use crate::NdockerPlugin;

use nu_plugin::{EvaluatedCall, PluginCommand};
use nu_protocol::{IntoPipelineData, Value};

pub struct ImageInspectCommand;

impl PluginCommand for ImageInspectCommand {
    type Plugin = NdockerPlugin;

    fn name(&self) -> &str {
        "ndocker image inspect"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build("ndocker image inspect")
            .input_output_types(vec![
                (nu_protocol::Type::Nothing, nu_protocol::Type::record()),
                (nu_protocol::Type::Nothing, nu_protocol::Type::String),
            ])
            .switch("string", "Show information in plain string", Some('s'))
            .required(
                "IMAGE",
                nu_protocol::Type::String.to_shape(),
                "The ID or name of the image to inspect.",
            )
    }

    fn description(&self) -> &str {
        "Inspect a Docker image and show its detailed information."
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            nu_protocol::LabeledError::new(format!("Failed to create runtime: {e}"))
        })?;
        let image_id: String = call.req(0)?;
        let image_inspect = rt
            .block_on(plugin.docker_socket.inspect_image(&image_id))
            .map_err(|e| {
                nu_protocol::LabeledError::new(format!("Failed to inspect Docker image: {e}"))
            })?;

        if let Some(timeout) = plugin.timeout {
            rt.shutdown_timeout(timeout);
            return Err(nu_protocol::LabeledError::new(format!(
                "Timeout: Operation time exceeded {} seconds",
                timeout.as_secs()
            )));
        }

        if call.has_flag("string") == Ok(true) {
            let result = serde_json::to_string_pretty(&image_inspect)
                .map_err(|e| nu_protocol::LabeledError::new(format!("Failed to serialize: {e}")))?;
            Ok(Value::string(result, call.head.clone()).into_pipeline_data())
        } else {
            let result = serde_json::to_string(&image_inspect)
                .map_err(|e| nu_protocol::LabeledError::new(format!("Failed to serialize: {e}")))?;
            let from_json_operation_id = engine.find_decl("from json")?.ok_or_else(|| {
                nu_protocol::LabeledError::new("Failed to find 'from json' operation".to_string())
            })?;
            let reuslt = engine.call_decl(
                from_json_operation_id,
                EvaluatedCall::new(call.head),
                Value::string(result, call.head.clone()).into_pipeline_data(),
                true,
                false,
            )?;
            Ok(reuslt)
        }
    }
}
