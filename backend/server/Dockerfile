# Build with the repository's pinned Rust toolchain and task definition inside the image.
FROM debian:trixie-slim AS build
ENV MISE_DATA_DIR=/mise-data \
    MISE_CONFIG_DIR=/mise-config \
    MISE_CACHE_DIR=/mise-cache \
    MISE_INSTALL_PATH=/usr/local/bin/mise \
    MISE_TRUSTED_CONFIG_PATHS=/src \
    MISE_VERSION=v2026.10.21 \
    PATH=/mise-data/shims:$PATH
WORKDIR /src
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && apt-get install -y --no-install-recommends bash ca-certificates curl git build-essential pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
ENV RUSTUP_HOME=/rustup CARGO_HOME=/cargo PATH=/cargo/bin:/mise-data/shims:$PATH
RUN curl --proto '=https' --proto-redir '=https' --fail --silent --show-error --location https://mise.run | sh
COPY . .
RUN mise trust --all \
    && mise install rust \
    && mise run build:server --release \
    && install -D target/release/logandaim-server /out/logandaim-server

FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl libssl3t64 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --home-dir /app --no-create-home app
WORKDIR /app
COPY --from=build --chown=10001:10001 /out/logandaim-server /usr/local/bin/logandaim-server
USER 10001:10001
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/logandaim-server"]
