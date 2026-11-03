# Runtime image only. The binary inside is the release build that `pr/base`
# produced and tested, downloaded into `.deploy/incoming/logandaim-server` by the
# `image` job, which pushes this image to GHCR. The VPS never compiles: it pulls the
# `sha-<commit>` tag. A build stage here would produce a second binary, from the same
# source, that the gate never ran.
FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --home-dir /app --no-create-home app
WORKDIR /app
COPY .deploy/incoming/logandaim-server /usr/local/bin/logandaim-server
USER 10001:10001
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/logandaim-server"]
