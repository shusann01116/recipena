# Recipena

LINE bot that can save recipe to Notion. Send a link of a recipe to the bot, and it will save the recipe to Notion.

## Environment Variables

- `LINE_CHANNEL_ACCESS_TOKEN`
- `LINE_CHANNEL_SECRET`
- `NOTION_INTEGRATION_TOKEN`
- `NOTION_DATABASE_ID`
- `PORT`

## Deploy

Configure the environment variables in the cloud run service. Run `gcloud auth login` beforehand.

```bash
make deploy
```
