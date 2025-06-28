# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Run the application locally
- `cargo test` - Run all tests
- `cargo test --verbose` - Run tests with verbose output

### Code Quality
- `cargo fmt` - Format code according to Rust standards
- `cargo fmt --all -- --check` - Check if code is properly formatted (CI command)
- `cargo clippy` - Run the linter for code quality checks
- `cargo clippy --all-targets --all-features -- -D warnings` - Run clippy with strict warnings (CI command)

### Docker and Deployment
- `make build-image` - Build Docker image
- `make deploy` - Deploy to Google Cloud Run (requires `gcloud auth login`)
- `docker run -p 8080:8080 --env-file .env recipena` - Run containerized version

## Architecture Overview

This is a Rust-based LINE bot that processes recipe URLs and saves them to Notion using a clean architecture pattern:

### Layer Structure
- **`src/app/`** - Application layer containing business logic services
- **`src/domain/`** - Domain models and core business entities  
- **`src/infra/`** - Infrastructure layer with external integrations
- **`src/libs/`** - External library wrappers and abstractions

### Key Components
- **Recipe Service** (`src/app/recipe.rs`) - Core business logic for recipe processing
- **LINE Integration** (`src/libs/line/`) - LINE Bot SDK wrapper and client
- **Notion Integration** (`src/libs/notion/`) - Notion API client for database operations
- **HTML Parsing** (`src/infra/html.rs`) - Web scraping for recipe title extraction
- **Axum Server** (`src/libs/axum/`) - HTTP server implementation with middleware

### Configuration
- Environment variables loaded via `src/config.rs`
- Supports both file-based config (`.recipena`) and environment variables
- Required env vars: `LINE_CHANNEL_ACCESS_TOKEN`, `LINE_CHANNEL_SECRET`, `NOTION_INTEGRATION_TOKEN`, `NOTION_DATABASE_ID`

### Dependency Injection Pattern
The codebase uses trait-based dependency injection with Arc<dyn Trait> for testability. All external dependencies are abstracted behind traits with mock implementations available for testing.

## Testing Strategy

- Unit tests use `mockall` crate for mocking external dependencies
- Test files are located alongside source code (e.g., tests in `src/app/recipe.rs`)
- Integration tests use the `test-case` crate for parameterized testing
- All external HTTP clients and database operations are mocked in tests

## Key Dependencies

- **Axum** - Async web framework for HTTP server
- **Tokio** - Async runtime
- **line-bot-sdk-rust** - Custom LINE Bot SDK (fork maintained by project)
- **notion-client** - Notion API integration
- **reqwest** - HTTP client for web scraping
- **tracing** - Structured logging with Stackdriver integration
- **config** - Configuration management
- **validator** - Request validation with derive macros