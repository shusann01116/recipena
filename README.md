# Recipena

A Rust-based LINE bot that automatically saves recipes to Notion. Simply send a recipe URL to the bot, and it will extract the recipe information and store it in your Notion database.

## Features

- **Recipe URL Processing**: Automatically extracts recipe information from URLs
- **LINE Bot Integration**: Seamless interaction through LINE messaging
- **Notion Database Storage**: Organized recipe storage in your Notion workspace
- **Error Handling**: Robust error handling and logging
- **Cloud Deployment**: Ready for deployment on Google Cloud Run

## Prerequisites

- Rust 2024 edition or later
- Docker (for containerized deployment)
- Google Cloud CLI (for deployment)
- LINE Developer Account
- Notion Integration

## Installation

### Local Development

1. Clone the repository:
```bash
git clone <repository-url>
cd recipena
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Set up environment variables (see below)

4. Run the application:
```bash
cargo run
```

### Docker

1. Build the Docker image:
```bash
make build-image
```

2. Run the container:
```bash
docker run -p 8080:8080 --env-file .env recipena
```

## Environment Variables

Create a `.env` file or set the following environment variables:

- `LINE_CHANNEL_ACCESS_TOKEN` - Your LINE bot channel access token
- `LINE_CHANNEL_SECRET` - Your LINE bot channel secret
- `NOTION_INTEGRATION_TOKEN` - Your Notion integration token
- `NOTION_DATABASE_ID` - The ID of your Notion database for storing recipes
- `PORT` - Server port (default: 8080)

## Usage

### Setting up LINE Bot

1. Create a LINE Bot channel on [LINE Developers Console](https://developers.line.biz/)
2. Get your Channel Access Token and Channel Secret
3. Set the webhook URL to your deployed service endpoint + `/webhook`

### Setting up Notion Integration

1. Create a new integration in [Notion Developers](https://developers.notion.com/)
2. Create a database in Notion for storing recipes
3. Share the database with your integration
4. Get the integration token and database ID

### Using the Bot

1. Add the bot as a friend on LINE
2. Send a recipe URL to the bot
3. The bot will automatically extract recipe information and save it to your Notion database

## API Endpoints

- `POST /webhook` - LINE webhook endpoint for receiving messages
- `GET /health` - Health check endpoint

## Deploy

### Google Cloud Run

Configure the environment variables in the cloud run service. Run `gcloud auth login` beforehand.

```bash
make deploy
```

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Project Structure

```
src/
├── app/           # Application layer
├── domain/        # Domain models and logic
├── infra/         # Infrastructure layer
├── libs/          # External library integrations
├── config.rs      # Configuration management
├── error.rs       # Error handling
├── logger.rs      # Logging setup
├── main.rs        # Application entry point
└── prelude.rs     # Common imports
```

## Technologies Used

- **Rust** - Systems programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **LINE Bot SDK** - LINE messaging integration
- **Notion Client** - Notion API integration
- **Reqwest** - HTTP client
- **Tracing** - Structured logging
- **Docker** - Containerization
