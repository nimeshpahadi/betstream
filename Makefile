.PHONY: all build-backend build-frontend compose-up compose-down clean logs

all: build-backend build-frontend

build-backend:
	docker build -f Dockerfile -t manualbetting-backend .

build-frontend:
	docker build -f betting-frontend/Dockerfile -t manualbetting-frontend .

compose-up:
	docker-compose up -d --build

compose-down:
	docker-compose down

clean:
	docker-compose down -v
	docker rmi manualbetting-backend manualbetting-frontend || true
	docker system prune -f

logs:
	docker-compose logs -f
