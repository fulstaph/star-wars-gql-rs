SHELL := bash
.ONESHELL:
MAKEFLAGS += --no-builtin-rules

.PHONY: help build-dev run-dev-env stop-dev

export APP_NAME := wookiepedia-gql-rs
export BRANCH_NAME := $(git branch --show-current)
NOCACHE := $(if $(NOCACHE),"--no-cache")

DCOMPOSE := docker-compose -p $(APP_NAME)

help: ## List all available targets with help
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build-dev: ## Build develop containers
	@docker build ${NOCACHE} --pull -f ./build/helper.Dockerfile -t helper:latest .

run-dev-env:
	$(DCOMPOSE) up -d postgres

dev-migration-up: ## Up all migrations to local database
	$(DCOMPOSE) run --rm helper sqlx migrate run

stop-dev: ## Stop develop environment
	$(DCOMPOSE) down
