use anyhow::{Context, Result};
use pdf_extract::extract_text;
use rig::providers::openai;
use std::{env, path::Path};

use rig::cli_chatbot::cli_chatbot;
use rig::{embeddings::EmbeddingsBuilder, vector_store::in_memory_store::InMemoryVectorStore};

fn load_pdf<P: AsRef<Path>>(path: P) -> Result<String> {
    extract_text(&path)
        .with_context(|| format!("Failed to extract text from {}", path.as_ref().display()))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let openai_client = openai::Client::from_env();
    let mut vector_store: InMemoryVectorStore<String> = InMemoryVectorStore::default();

    let embedding_model = openai_client.embedding_model("text-embedding-ada-002");

    let current_dir = env::current_dir()?;
    let documents_dir = current_dir.join("documents");

    let pdf_path = documents_dir.join("example.pdf");

    let pdf_text = load_pdf(pdf_path).context("Failed to load PDF")?;

    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .document(&pdf_text)?
        .build()
        .await?
        .into_iter()
        .map(|(key, emb)| (key.to_owned(), emb))
        .collect::<Vec<_>>();

    vector_store.add_documents(embeddings);

    let rag_agent = openai_client.agent("gpt-4o-mini")
      .preamble("You are a helpful assistant that answers questions based on the given context from PDF documents.")
      .dynamic_context(2, vector_store.index(embedding_model))
      .build();

    // Use the cli_chatbot function to create the CLI interface
    cli_chatbot(rag_agent).await?;

    Ok(())
}
