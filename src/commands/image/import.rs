//! This module is for command `ndocker image import`

use std::collections::HashMap;

use crate::NdockerPlugin;
use crate::commands::image::Image;
use crate::utils::file::{check_file_exists, read_file_stream};
use crate::utils::net::check_url;

use bollard::{body_stream, body_full};
use bytes::Bytes;
use nu_plugin::PluginCommand;
use nu_protocol::{CustomValue, IntoPipelineData, LabeledError, Value, PipelineData};

use bollard::query_parameters::{CreateImageOptionsBuilder, ListImagesOptionsBuilder};
use bollard::secret::CreateImageInfo;

use futures_util::stream::{Stream, StreamExt};

enum ImportSrc {
    File(String),
    Url(String),
    Stdin,
}

pub struct ImageImportCommand;

impl ImageImportCommand {
    async fn import_from_file(
        plugin: &<ImageImportCommand as PluginCommand>::Plugin,
        current_path: &String,
        path: String,
        create_image_options: CreateImageOptionsBuilder,
    ) -> Result<impl Stream<Item = Result<CreateImageInfo, bollard::errors::Error>>, LabeledError>
    {
        let file_stream = read_file_stream(current_path.clone(), path)
            .await
            .map_err(|e| {
                nu_protocol::LabeledError::new(format!("Failed to read file stream: {}", e))
            })?;

        Ok(plugin.docker_socket.create_image(
            Some(create_image_options.from_src("-").build()),
            Some(body_stream(file_stream)),
            None,
        ))
    }

    async fn import_from_stdin(
        plugin: &<ImageImportCommand as PluginCommand>::Plugin,
        input: PipelineData,
        create_image_options: CreateImageOptionsBuilder,
    ) -> Result<impl Stream<Item = Result<CreateImageInfo, bollard::errors::Error>>, LabeledError>
    {
        if let PipelineData::ByteStream(stream, _) = input {
            let bytes = stream.into_bytes().map_err(|e| {
                nu_protocol::LabeledError::new(format!("Failed to read stdin: {e}"))
            })?;
            let bytes = Bytes::from_owner(bytes);
            Ok(plugin.docker_socket.create_image(
                Some(create_image_options.from_src("-").build()),
                Some(body_full(bytes)),
                None,
            ))
        } else {
            Err(nu_protocol::LabeledError::new(
                "Expected binary input from stdin".to_string(),
            ))
        }
    }

    async fn import_from_url(
        plugin: &<ImageImportCommand as PluginCommand>::Plugin,
        url: String,
        create_image_options: CreateImageOptionsBuilder,
    ) -> Result<impl Stream<Item = Result<CreateImageInfo, bollard::errors::Error>>, LabeledError>
    {
        Ok(plugin.docker_socket.create_image(
            Some(create_image_options.from_src(&url).build()),
            None,
            None,
        ))
    }

    fn get_import_source(file: &String, current_path: &String) -> Result<ImportSrc, LabeledError> {
        if file == "-" {
            Ok(ImportSrc::Stdin)
        } else if check_url(file).is_ok() {
            Ok(ImportSrc::Url(file.clone()))
        } else if check_file_exists(current_path, file).is_ok() {
            Ok(ImportSrc::File(file.clone()))
        } else {
            Err(nu_protocol::LabeledError::new(format!(
                "File or URL does not exist: {}",
                file
            )))
        }
    }

    fn handle_url_progress(
        response: CreateImageInfo,
        id: &mut String,
        line_size: &mut usize,
    ) {
        *id = response.status.unwrap_or_default();
        let progress = response.progress.unwrap_or_default();
        let output;

        if id.is_empty() && progress.is_empty() {
            return;
        } else if id.is_empty() {
            output = format!("\rImporting image: {}", progress);
        } else if progress.is_empty() {
            if *line_size == 0 {
                eprintln!("{}", id);
            } else {
                eprintln!();
                eprintln!("{}", id);
            }
            return;
        } else {
            output = format!("\r{}: {}", id, progress);
        }

        if *line_size > 0 {
            eprint!("\r{}", " ".repeat(*line_size));
        }
        *line_size = output.len();
        eprint!("\r{}", output);
    }

    fn handle_named_params(
        call: &nu_plugin::EvaluatedCall,   
    ) -> HashMap<String, Value> {
        call.named
            .clone()
            .into_iter()
            .filter(|n| n.1.is_some())
            .map(|n| (n.0.item.clone(), n.1.clone().unwrap()))
            .collect::<HashMap<String, Value>>()
    }

    fn option_get_commit_message(
        params: &HashMap<String, Value>, 
        create_image_options: CreateImageOptionsBuilder
    ) -> CreateImageOptionsBuilder{
        let mut option = create_image_options;
        if let Some(message) = params.get("message") {
            let message = message.as_str().unwrap_or_default();
            option = option.message(message);
        }
        option
    }

    fn option_get_platform(
        params: &HashMap<String, Value>, 
        create_image_options: CreateImageOptionsBuilder
    ) -> CreateImageOptionsBuilder {
        let mut option = create_image_options;
        if let Some(platform) = params.get("platform") {
            let platform = platform.as_str().unwrap_or_default();
            option = option.platform(platform);
        }
        option
    }

    fn option_get_changes(
        params: &HashMap<String, Value>, 
        create_image_options: CreateImageOptionsBuilder
    ) -> CreateImageOptionsBuilder {
        let mut options = create_image_options;
        if let Some(changes) = params.get("change") {
            let changes = changes
                .clone()
                .into_list()
                .unwrap_or_default()
                .into_iter()
                .map(|v| v.into_string().unwrap_or_default())
                .collect::<Vec<String>>();
            options = options.changes(changes);
        }
        options
    }
}

impl PluginCommand for ImageImportCommand {
    type Plugin = NdockerPlugin;

    fn name(&self) -> &str {
        "ndocker image import"
    }

    fn description(&self) -> &str {
        "Import the contents from a tarball to create a filesystem image."
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build("ndocker image import")
            .input_output_types(vec![
                (nu_protocol::Type::Nothing,
                nu_protocol::Type::Custom("Image".to_string().into_boxed_str())),
                (nu_protocol::Type::Binary, 
                nu_protocol::Type::Custom("Image".to_string().into_boxed_str())),]
            )
            .switch(
                "interactive",
                "Apply Dockerfile instruction to the created image interactively",
                Some('I'),
            )
            .named(
                "change",
                nu_protocol::Type::List(Box::new(nu_protocol::Type::String)).to_shape(),
                "Apply Dockerfile instruction to the created image",
                Some('c'),
            )
            .named(
                "message",
                nu_protocol::Type::String.to_shape(),
                "Set a commit message for the image",
                Some('m'),
            )
            .named(
                "platform",
                nu_protocol::Type::String.to_shape(),
                "Set the platform for the image, in the format os[/arch[/variant]], for example: linux/amd64/v5",
                None,
            )
            .required(
                "file|URL|-",
                nu_protocol::Type::String.to_shape(),
                "The path to the tarball to import.",
            )
            .optional("REPOSITORY[:TAG]", nu_protocol::Type::String.to_shape(), 
                "The repository and tag to apply to the imported image. If not specified, the image will not be tagged.")
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            nu_protocol::LabeledError::new(format!("Failed to create runtime: {e}"))
        })?;

        let file = call.req::<String>(0)?;
        let current_path = engine.get_current_dir().map_err(|e| {
            nu_protocol::LabeledError::new(format!("Failed to get current directory: {e}"))
        })?;
        let import_src = ImageImportCommand::get_import_source(&file, &current_path)?;

        let mut options = CreateImageOptionsBuilder::new();

        if let Ok(repotag) = call.req::<String>(1) {
            options = options.repo(&repotag);
        }

        let params = Self::handle_named_params(call);

        options = Self::option_get_commit_message(&params, options);
        options = Self::option_get_platform(&params, options);
        options = Self::option_get_changes(&params, options);

        let mut id = String::new();
        let imported_image = rt.block_on(async {
            match import_src {
                ImportSrc::File(path) => {
                    let mut response_stream = 
                        Self::import_from_file(plugin, &current_path, path, options).await?;
                    while let Some(response) = response_stream.next().await {
                        let response = response.map_err(|e| {
                            nu_protocol::LabeledError::new(format!("Failed to import image from file: {e}"))
                        })?;
                        id = response.status.unwrap_or_default();
                        eprintln!("{}", &id);
                    }
                }
                ImportSrc::Stdin => {
                    let mut response_stream = 
                        Self::import_from_stdin(plugin, input, options).await?;
                    while let Some(response) = response_stream.next().await {
                        let response = response.map_err(|e| {
                            nu_protocol::LabeledError::new(format!("Failed to import image from stdin: {e}"))
                        })?;
                        id = response.status.unwrap_or_default();
                        eprintln!("{}", &id);
                    }
                }
                ImportSrc::Url(url) => {
                    let mut response_stream = 
                        Self::import_from_url(plugin, url, options).await?;
                    let mut line_size = 0;
                    while let Some(response) = response_stream.next().await {
                        let response = response.map_err(|e| {
                            nu_protocol::LabeledError::new(format!("Failed to import image from URL: {e}"))
                        })?;
                        Self::handle_url_progress(response, &mut id, &mut line_size);
                    }
                    eprintln!();
                }
            };
            
            plugin
                .docker_socket
                .list_images(Some(ListImagesOptionsBuilder::new().build()))
                .await
                .map_err(|e| {
                    nu_protocol::LabeledError::new(format!("Failed to list containers: {e}"))
                })
        })?;
        let result = imported_image
            .into_iter()
            .filter(|image| image.id == id)
            .map(|image| Image::new(image))
            .map(|image| Image::clone_value(&image, call.head.clone()))
            .collect::<Vec<_>>();
        Ok(Value::List {
            vals: result,
            internal_span: call.head.clone(),
        }
        .into_pipeline_data())
    }
}
