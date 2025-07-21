//! This module is for command `ndocker image history`.

use crate::NdockerPlugin;
use crate::commands::image::ImageHistory;

use nu_plugin::PluginCommand;
use nu_protocol::{CustomValue, IntoPipelineData, Value};

pub struct ImageHistoryCommand;

impl PluginCommand for ImageHistoryCommand {
    type Plugin = NdockerPlugin;

    fn name(&self) -> &str {
        "ndocker image history"
    }

    fn description(&self) -> &str {
        "List the history of a Docker image."
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build("ndocker image history")
            .input_output_types(vec![(
                nu_protocol::Type::Nothing,
                nu_protocol::Type::table(),
            )])
            .switch(
                "wide",
                "Show full information of the string instead of a short version.",
                Some('w'),
            )
            .required(
                "IMAGE",
                nu_protocol::Type::String.to_shape(),
                "The ID or name of the image to show history for.",
            )
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            nu_protocol::LabeledError::new(format!("Failed to create runtime: {e}"))
        })?;
        let image_id: String = call.req(0)?;
        let histories = rt
            .block_on(plugin.docker_socket.image_history(&image_id))
            .map_err(|e| {
                nu_protocol::LabeledError::new(format!("Failed to get image history: {e}"))
            })?;

        let span = call.head.clone();
        let result: Vec<Value>;
        if call.has_flag("wide") == Ok(true) {
            result = histories
                .into_iter()
                .map(|history| ImageHistory::new(history))
                .map(|history| history.full_version(span.clone()))
                .collect::<Vec<_>>();
        } else {
            result = histories
                .into_iter()
                .map(|history| ImageHistory::new(history))
                .map(|history| history.clone_value(span.clone()))
                .collect::<Vec<_>>();
        }
        let result = Value::List {
            vals: result,
            internal_span: span.clone(),
        };
        Ok(result.into_pipeline_data())
    }
}
