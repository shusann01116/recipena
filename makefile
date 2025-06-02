build-image:
	docker build -t recipena .

deploy:
	gcloud run deploy recipena --source .
