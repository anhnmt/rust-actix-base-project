APP_NAME=rust-api-base
APP_VERSION=latest
DOCKER_REGISTRY=registry.gitlab.com/xdorro/registry
MAIN_DIR=./cmd

docker.build:
	docker build -f ./build/Dockerfile -t $(DOCKER_REGISTRY)/$(APP_NAME):$(APP_VERSION) .

docker.push:
	docker push $(DOCKER_REGISTRY)/$(APP_NAME):$(APP_VERSION)

docker.run:
	docker rm -f $(APP_NAME)
	docker run -dp 8080:8080 --name $(APP_NAME) $(DOCKER_REGISTRY)/$(APP_NAME):$(APP_VERSION)

docker.dev: docker.build docker.push

docker.test: docker.build docker.run


