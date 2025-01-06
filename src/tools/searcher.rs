use std::time::Duration;

use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

use tavily::{SearchRequest, Tavily};

#[derive(Debug, Deserialize)]
pub struct SearcherArgs {
    pub query: String,
    pub max_results: Option<i32>,
    pub advanced_search: Option<bool>,
    pub topic: Option<String>,
    pub include_days: Option<i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum SearcherError {
    #[error("Failed to initialize Tavily: {0}")]
    TavilyError(String),
    #[error("Failed to search: {0}")]
    SearchFailed(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearcherOutput {
    pub title: String,
    pub url: String,
    pub content: String,
}
pub struct SearcherTool;

impl Tool for SearcherTool {
    const NAME: &'static str = "searcher";

    type Args = SearcherArgs;
    type Output = Vec<SearcherOutput>;
    type Error = SearcherError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Searches the web".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Query to search for" },
                    "max_results": { "type": "integer", "description": "Maximum number of results to return" },
                    "advanced_search": { "type": "boolean", "description": "Enable advanced search" },
                    "topic": { "type": "string", "description": "Topic to search for, e.g., 'news'" },
                    "include_days": { "type": "integer", "description": "Number of days to include in search, only available with 'news' topic is used" },
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tavily_api_key = std::env::var("TAVILY_API_KEY")
            .map_err(|_| SearcherError::TavilyError("Missing Tavily API key".to_string()))?;

        let advanced_search = match args.advanced_search {
            Some(true) => "advanced",
            _ => "basic",
        };

        let search_request = SearchRequest::new(&tavily_api_key, &args.query)
            .search_depth(advanced_search)
            .max_results(args.max_results.unwrap_or(5))
            .topic(args.topic.unwrap_or("general".to_string()))
            .days(args.include_days.unwrap_or(3));

        let tavily = Tavily::builder(&tavily_api_key)
            .timeout(Duration::from_secs(30))
            .max_retries(3)
            .build()
            .map_err(|e| {
                SearcherError::TavilyError(format!("Failed to initialize Tavily: {:?}", e))
            })?;

        let search_response = tavily
            .call(&search_request)
            .await
            .map_err(|e| SearcherError::SearchFailed(e.to_string()))?;

        let mapped_results = search_response
            .results
            .into_iter()
            .map(|result| SearcherOutput {
                title: result.title,
                url: result.url,
                content: result.content,
            })
            .collect();

        Ok(mapped_results)
    }
}
