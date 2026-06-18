# Load environment variables from .env file if it exists
ifneq (,$(wildcard ./.env))
    include .env
    export $(shell sed 's/=.*//' .env)
endif

# Default database URL fallback if not set in .env
DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/rust_backend

.PHONY: help db-status migration-create migration-up migration-down migration-redo db-setup

help: ## Show this help message with available commands
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

db-status: ## Check the status of database migrations
	diesel migration list

migration-create: ## Create a new up/down SQL migration. Usage: make migration-create name=create_users
	@if [ -z "$(name)" ]; then \
		echo "Error: 'name' variable is required. Example: make migration-create name=create_users"; \
		exit 1; \
	fi
	diesel migration generate $(name) --migration-dir=migrations

migration-up: ## Run all pending database migrations (Generates schema.rs automatically)
	diesel migration run --migration-dir=migrations

migration-down: ## Rollback the single most recent migration step
	diesel migration revert --migration-dir=migrations

migration-redo: ## Rollback and re-run the latest migration step (Useful during local development)
	diesel migration redo --migration-dir=migrations

db-setup: ## Run initial setup, create database, and execute any existing migrations
	diesel database setup --migration-dir=migrations

