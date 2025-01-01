use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SentimentClassification {
    pub sentiment: Sentiment,
    pub confidence: f32,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub enum EntityType {
    Person,
    Organization,
    Location,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct Entity {
    pub text: String,
    pub entity_type: EntityType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ExtractedEntities {
    pub entities: Vec<Entity>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TextAnalysis {
    pub sentiment: SentimentClassification,
    pub entities: Vec<Entity>,
}
