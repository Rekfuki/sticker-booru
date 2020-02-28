POSTGRES_HOST ?= localhost
POSTGRES_PORT ?= 5432
POSTGRES_USER ?= stickerbooru
POSTGRES_DATABASE ?= stickerbooru

.PHONY: run
run: run-local-db run-local-server

.PHONY: run-local-db
run-local-db: password-set
	docker-compose up -d

.PHONY: run-local-server
run-local-server: password-set
	POSTGRES_USER="$(POSTGRES_USER)" \
	POSTGRES_HOST="$(POSTGRES_HOST)" \
	POSTGRES_PORT="$(POSTGRES_PORT)" \
	POSTGRES_DATABASE="$(POSTGRES_DATABASE)" \
	cargo run

.PHONY: password-set
password-set:
	@[ "${POSTGRES_PASSWORD}" ] || ( echo ">> POSTGRES_PASSWORD is not set"; exit 1 )