//! This module is for command `ndocker images`.

use crate::NdockerPlugin;
use crate::commands::image::Image;

use nu_plugin::PluginCommand;
use nu_protocol::{CustomValue, Example, IntoPipelineData, Record, Span, Value};

use bollard::query_parameters::ListImagesOptionsBuilder;

use tokio::runtime::Runtime;

pub struct ImagesCommand;

impl PluginCommand for ImagesCommand {
    type Plugin = NdockerPlugin;

    fn name(&self) -> &str {
        "ndocker images"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build("ndocker images")
            .input_output_types(vec![(
                nu_protocol::Type::Nothing,
                nu_protocol::Type::table(),
            )])
            .switch("all", "Show all the information of the image.", Some('a'))
            .switch("short", "Show a short version of information", Some('s'))
    }

    fn description(&self) -> &str {
        "List Docker images and their infomation."
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let rt = Runtime::new().map_err(|e| {
            nu_protocol::LabeledError::new(format!("Failed to create runtime: {e}"))
        })?;
        let images = rt
            .block_on(
                plugin
                    .docker_socket
                    .list_images(Some(ListImagesOptionsBuilder::new().build())),
            )
            .map_err(|e| {
                nu_protocol::LabeledError::new(format!("Failed to list Docker images: {e}"))
            })?;

        if let Some(timeout) = plugin.timeout {
            rt.shutdown_timeout(timeout);
            return Err(nu_protocol::LabeledError::new(format!(
                "Timeout: Operation time exceeded {} seconds",
                timeout.as_secs()
            )));
        }

        let span = call.head.clone();
        let result: Vec<Value>;
        if call.has_flag("all") == Ok(true) {
            result = images
                .into_iter()
                .map(|image| Image::new(image))
                .map(|image| Image::clone_value(&image, span.clone()))
                .collect::<Vec<_>>();
        } else if call.has_flag("short") == Ok(true) {
            result = images
                .into_iter()
                .map(|image| Image::new(image))
                .map(|image| image.short_version(span.clone()))
                .collect::<Vec<_>>();
        } else {
            result = images
                .into_iter()
                .map(|image| Image::new(image))
                .map(|image| image.standard_version(span.clone()))
                .collect::<Vec<_>>();
        }

        let result = Value::List {
            vals: result,
            internal_span: span.clone(),
        };
        Ok(result.into_pipeline_data())
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![
            Example {
                description: "List all Docker images",
                example: "ndocker images",
                result: Some(Value::test_list(vec![Value::test_record(
                    Record::from_raw_cols_vals(
                        vec![
                            "id".into(),
                            "repotags".into(),
                            "created".into(),
                            "size".into(),
                        ],
                        vec![
                            Value::test_string("8daff9993116"),
                            Value::test_list(vec![Value::test_string("rust:1.84.0")]),
                            Value::test_string("6 months ago"),
                            Value::test_string("1.4 GB"),
                        ],
                        Span::unknown(),
                        Span::unknown(),
                    )
                    .unwrap(),
                )])),
            },
            Example {
                description: "List all Docker images in a short version",
                example: "ndocker images -s",
                result: Some(Value::test_list(vec![Value::test_record(
                    Record::from_raw_cols_vals(
                        vec!["repotags".into(), "created".into(), "size".into()],
                        vec![
                            Value::test_list(vec![Value::test_string("rust:1.84.0")]),
                            Value::test_string("6 months ago"),
                            Value::test_string("1.4 GB"),
                        ],
                        Span::unknown(),
                        Span::unknown(),
                    )
                    .unwrap(),
                )])),
            },
            Example {
                description: "List all Docker images with all information, \"containers\" is the number of containers using the image. -1 means your docker daemon does not support this feature.",
                example: "ndocker images -a",
                result: Some(Value::test_list(vec![Value::test_record(
                    Record::from_raw_cols_vals(
                        vec![
                            "id".into(),
                            "parent_id".into(),
                            "repotags".into(),
                            "created".into(),
                            "size".into(),
                            "shared_size".into(),
                            "containers".into(),
                        ],
                        vec![
                            Value::test_string("8daff9993116"),
                            Value::test_string(""),
                            Value::test_list(vec![Value::test_string("rust:1.84.0")]),
                            Value::test_string("6 months ago"),
                            Value::test_string("1.4 GB"),
                            Value::test_string("-1 B"),
                            Value::test_string("-1"),
                        ],
                        Span::unknown(),
                        Span::unknown(),
                    )
                    .unwrap(),
                )])),
            },
            Example {
                description: "List \"id\", \"repotags\" and  \"size\" of all docker images",
                example: "ndocker images | select id repotags size",
                result: None,
            },
            Example {
                description: "List all docker images in shrot version and sort by \"size\"",
                example: "ndocker images -s | sort-by size",
                result: None,
            },
        ]
    }
}
