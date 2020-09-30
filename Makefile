.PHONY: help
help:
	@awk -F':.*##' '/^[-_a-zA-Z0-9]+:.*##/{printf"%-16s\t%s\n",$$1,$$2}' $(MAKEFILE_LIST) | sort

.PHONY: release-build
release-build: ## Build mackerel_agent for release.
	cargo build --release
	strip /tmp/target-vagrant/release/mackerel_agent

.PHONY: test
test: ## Test.
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test --no-fail-fast -- --nocapture
