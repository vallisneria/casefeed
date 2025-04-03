GIT_HASH = $(shell git rev-parse --short HEAD)

build-all: build-casefeed-web-image build-ccourt-crawler-image build-court-crawler-image

build-casefeed-web-image:
	docker build --platform linux/amd64 --tag registry.gitlab.com/vallisneria/casefeed/casefeed-web:$(GIT_HASH) -f ./casefeed-web/Dockerfile .

build-ccourt-crawler-image:
	docker build --platform linux/amd64 --tag registry.gitlab.com/vallisneria/casefeed/ccourt-crawler:$(GIT_HASH) -f ./ccourt-crawler/Dockerfile .

build-court-crawler-image:
	docker build --platform linux/amd64 --tag registry.gitlab.com/vallisneria/casefeed/court-crawler:$(GIT_HASH) -f ./court-crawler/Dockerfile .

push-all: push-casefeed-web-image push-ccourt-crawler-image push-court-crawler-image

push-casefeed-web-image:
	docker push registry.gitlab.com/vallisneria/casefeed/casefeed-web:$(GIT_HASH)

push-ccourt-crawler-image:
	docker push registry.gitlab.com/vallisneria/casefeed/ccourt-crawler:$(GIT_HASH)

push-court-crawler-image:
	docker push registry.gitlab.com/vallisneria/casefeed/court-crawler:$(GIT_HASH)

build-helm:
	cd Chart && helm package .
