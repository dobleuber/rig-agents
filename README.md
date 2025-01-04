# RIG Agents

A Rust-based project that implements various AI agents using the RIG (Rust Inference Gateway) framework. This project demonstrates the integration of different AI capabilities including chat completions, vector databases, and specialized tools.

## Features

- OpenAI GPT integration for natural language processing
- Qdrant vector database integration for efficient similarity search
- PDF document processing capabilities
- RESTful API integrations
- Async runtime support with Tokio

## Prerequisites

- Rust 2021 edition or later
- Environment variables configuration (see Configuration section)

## Configuration

Create a `.env` file in the root directory with the following variables:

```env
QDRANT_URL=your_qdrant_url
QDRANT_API_KEY=your_qdrant_api_key
OPENAI_API_KEY=your_openai_api_key
RAPIDAPI_KEY=your_rapidapi_key
```

## Dependencies

Key dependencies include:
- `rig-core`: Core framework for AI agent implementations
- `qdrant-client`: Client for Qdrant vector database
- `serde`: Serialization/deserialization support
- `tokio`: Async runtime
- `anyhow`: Error handling
- `dotenv`: Environment variable management

## Getting Started

1. Clone the repository
2. Copy `.env.example` to `.env` and fill in your API keys
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the main example:
   ```bash
   cargo run
   ```

## Project Structure

- `src/main.rs`: Entry point and basic chat example
- `src/examples/`: Various agent implementations and examples
- `src/lib.rs`: Core library functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.
