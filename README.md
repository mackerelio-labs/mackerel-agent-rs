# mackerel-agent-rs

! This is just an experimental version. !

Another [mackerel-agent](https://github.com/mackerelio/mackerel-agent) implemented in Rust.

## Development

### Pre-requirements

- [Docker](https://www.docker.com/products/docker-desktop)
- make

rustc & Cargo is installed in the Docker container. You can freely install them on your computer too by yourself.

### Start development

Run `make sh` to start a devepolment console. `make test` to run tests. `make start` to start a mackerel_agent in debug mode.

```console
$ make help
clean           	 Clean all resources.
release-build   	 Build mackerel_agent for release.
resync          	 Re-sync the Docker volume when it's broken.
sh              	 Start a development shell.
start           	 Build and start a mackerel_agent in debug mode.
test            	 Test.
```
