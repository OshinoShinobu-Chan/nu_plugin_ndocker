//! Utility functions for network operations in the plugin.

#[derive(Debug)]
pub enum NetworkErrorType {
    UrlError,
    OtherError,
}

#[derive(Debug)]
pub struct NetworkError {
    pub error_type: NetworkErrorType,
    pub message: String,
}

pub fn check_url(url: &str) -> Result<(), NetworkError> {
    if url.is_empty() {
        return Err(NetworkError {
            error_type: NetworkErrorType::UrlError,
            message: "URL cannot be empty".to_string(),
        });
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(NetworkError {
            error_type: NetworkErrorType::UrlError,
            message: format!("Invalid URL format: {}", url),
        });
    }
    Ok(())
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
