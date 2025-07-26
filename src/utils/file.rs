//! Utility functions for file system operations in the plugin.

use std::io::Error;
use std::path::Path;

use bytes::{Bytes, BytesMut};

use tokio::fs::File;

use tokio_util::codec;
use tokio_util::codec::{BytesCodec, FramedRead};

use futures_util::stream::Map;
use futures_util::stream::StreamExt;

#[derive(Debug)]
pub enum FileErrorType {
    FileError,
    OtherError,
}

#[derive(Debug)]
pub struct FileError {
    pub error_type: FileErrorType,
    pub message: String,
}

pub fn check_file_exists(current_path: &String, path: &str) -> Result<(), FileError> {
    if !Path::new(current_path).join(path).exists() {
        return Err(FileError {
            error_type: FileErrorType::FileError,
            message: format!("File not found: {}", path),
        });
    }
    Ok(())
}

pub async fn read_file_stream(
    current_path: String,
    path: String,
) -> Result<
    Map<FramedRead<File, BytesCodec>, impl FnMut(Result<BytesMut, Error>) -> Bytes>,
    FileError,
> {
    let absolute_path = Path::new(&current_path).join(path);
    let file = File::open(absolute_path).await.map_err(|e| FileError {
        error_type: FileErrorType::FileError,
        message: format!("{}", e),
    })?;

    Ok(codec::FramedRead::new(file, codec::BytesCodec::new()).map(|r| r.unwrap().freeze()))
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
