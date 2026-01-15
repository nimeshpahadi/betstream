.PHONY: release build-backend build-frontend up down clean logs

export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1

release: build-backend build-frontend

build-backend:
	docker build \
		--progress=plain \
		-f Dockerfile \
		-t manualbetting-backend:latest .

build-frontend:
	docker build \
		--progress=plain \
		-f betting-frontend/Dockerfile \
		-t manualbetting-frontend:latest .

up:
	docker compose up -d

down:
	docker compose down

clean:
	docker compose down -v
	docker rmi manualbetting-backend:latest manualbetting-frontend:latest || true
	docker system prune -f

logs:
	docker compose logs -f
