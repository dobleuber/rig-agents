use anyhow::Result;
use rig::providers::openai;

mod entities;
use entities::TextAnalysis;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let openai_client = openai::Client::from_env();
    let text_analyzer = openai_client
        .extractor::<TextAnalysis>("gpt-4o-mini")
        .preamble("
            You are a text analysis AI. For the given text:
            1. Classify the overall sentiment (Positive, Negative, or Neutral) with a confidence score.
            2. Identify and extract named entities (Person, Organization, Location) with their start and end indices.
        ")
        .build();

    let text = "I had a great time visiting Google's headquarters in Mountain View. Sundar Pichai's leadership has been impressive.";
    let result = text_analyzer.extract(text).await?;

    println!("Text: {}", text);
    println!(
        "Sentiment: {:?} (Confidence: {:.2})",
        result.sentiment.sentiment, result.sentiment.confidence
    );
    println!("Entities:");
    for entity in result.entities {
        println!(
            "- {:?}: {} ({}:{})",
            entity.entity_type, entity.text, entity.start, entity.end
        );
    }

    Ok(())
}
