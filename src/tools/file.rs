use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct ReadFileArgs {
    pub path: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadFileError {
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("Failed to read file {0}")]
    ReadError(String),
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";

    type Args = ReadFileArgs;
    type Output = String;
    type Error = ReadFileError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Reads a file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Path to the file" },
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if !std::path::Path::new(&args.path).exists() {
            return Err(ReadFileError::FileNotFound(args.path));
        }
        let content = std::fs::read_to_string(&args.path)
            .map_err(|e| ReadFileError::ReadError(e.to_string()))?;
        Ok(content)
    }
}
