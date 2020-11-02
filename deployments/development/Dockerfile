FROM rust:1-slim

SHELL ["/bin/bash", "-ex", "-o", "pipefail", "-c"]

WORKDIR /mnt
VOLUME /mnt

RUN apt-get -y update \
 && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    rsync \
 && rustup default nightly \
 && cargo install cargo-watch \
 && rustup toolchain add nightly-2020-10-08 \
 && rustup component add clippy \
 && rustup component add rust-src \
 && rustup component add --toolchain nightly-2020-10-08 rustfmt \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain ./
RUN mkdir -p src \
 && touch src/lib.rs \
 && cargo build \
 && mv target /tmp

ENTRYPOINT ["./deployments/development/entrypoint.sh"]
