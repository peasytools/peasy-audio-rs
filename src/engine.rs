use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PeasyError {
    #[error("Command failed: {0}")]
    CommandFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Get version of the underlying tool.
pub fn version() -> Result<String, PeasyError> {
    Ok("peasy-audio 0.1.1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version().unwrap();
        assert!(v.contains("0.1.1"));
    }
}
