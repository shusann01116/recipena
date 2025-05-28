gen.api.webhook:
	docker run --rm \
		-v $(PWD):/local openapitools/openapi-generator-cli generate \
		-i https://raw.githubusercontent.com/line/line-openapi/refs/heads/main/webhook.yml \
		-g go \
		--additional-properties=packageName=line-api-webhook \
		-o /local/go/line-api/webhook

deploy:
	gcloud run deploy recipena --source .
