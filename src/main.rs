use dotenv::dotenv;
use rig::{completion::Prompt, providers::openai};

const OPENAI_MODEL: &str = "gpt-4o-mini";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let openai_client = openai::Client::from_env();
    let agent = openai_client.agent(OPENAI_MODEL).build();

    let response = agent
        .prompt("Explica que es la computacion cuantica como para que un nino pueda entenderlo")
        .await
        .expect("Prompt failed");
    println!("{}", response);
}
