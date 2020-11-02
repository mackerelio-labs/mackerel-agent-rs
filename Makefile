.PHONY: help
help:
	@awk -F':.*##' '/^[-_a-zA-Z0-9]+:.*##/{printf"%-16s\t%s\n",$$1,$$2}' $(MAKEFILE_LIST) | sort

export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1

within_docker := $(shell sh -c 'stat /.dockerenv 2> /dev/null')

.PHONY: clean
clean: ## Clean all resources.
ifeq ($(within_docker),)
	docker-compose down
endif
	rm -rfv target

.PHONY: resync
resync: ## Re-sync the Docker volume when it's broken.
ifeq ($(within_docker),)
	docker-compose stop app-src
	find . -name '.unison.*' -exec rm -vrf {} +;
	docker-compose run --rm app-src /docker-entrypoint.d/precopy_appsync.sh
	docker-compose up -d app-src
endif

.PHONY: release-build
release-build: ## Build mackerel_agent for release.
ifeq ($(within_docker),)
	docker-compose run --rm app make release-build
else
	cargo build --release
	strip target/release/mackerel_agent
endif

.PHONY: sh
sh: ## Start a development shell.
ifeq ($(within_docker),)
	docker-compose pull
	docker-compose build --build-arg BUILDKIT_INLINE_CACHE=1 --force-rm --pull
	docker-compose run --rm app bash
endif

.PHONY: start
start: ## Build and start a mackerel_agent in debug mode.
ifeq ($(within_docker),)
	docker-compose run --rm app make start
else
	cargo run
endif

.PHONY: test
test: ## Test.
	cargo +nightly-2020-10-08 fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test --no-fail-fast -- --nocapture
ifeq ($(within_docker),)
	find . -name '*.sh' -exec shellcheck {} +
	find . -name '*.yml' -exec yamllint {} +
	hadolint deployments/*/Dockerfile
endif
