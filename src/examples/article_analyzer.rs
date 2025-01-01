use anyhow::Result;
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod entities;
use entities::{Entity, SentimentClassification};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
enum Topic {
    Politics,
    Technology,
    Sports,
    Entertainment,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct NewsArticleAnalysis {
    topic: Topic,
    sentiment: SentimentClassification,
    entities: Vec<Entity>,
    key_points: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let openai_client = openai::Client::from_env();

    let news_analyzer = openai_client
        .extractor::<NewsArticleAnalysis>("gpt-4o-mini")
        .preamble("
            You are a news article analysis AI. For the given news article:
            1. Classify the main topic (Politics, Technology, Sports, Entertainment, or Other).
            2. Analyze the overall sentiment (Positive, Negative, or Neutral) with a confidence score.
            3. Identify and extract named entities (Person, Organization, Location) with their start and end indices.
            4. Extract 3-5 key points from the article.
        ")
        .build();

    let article_text = r#"The latest news in politics and technology."#;

    let result = news_analyzer.extract(article_text).await?;

    println!("Article Analysis:");
    println!("Topic: {:?}", result.topic);
    println!(
        "Sentiment: {:?} (Confidence: {:.2})",
        result.sentiment.sentiment, result.sentiment.confidence
    );
    println!("\nEntities:");
    for entity in &result.entities {
        println!(
            "- {:?}: {} ({}:{})",
            entity.entity_type, entity.text, entity.start, entity.end
        );
    }
    println!("\nKey Points:");
    for (i, point) in result.key_points.iter().enumerate() {
        println!("{}. {}", i + 1, point);
    }

    Ok(())
}
