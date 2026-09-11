FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p sdkwork-api-community-standalone-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/sdkwork-api-community-standalone-gateway /usr/local/bin/sdkwork-api-community-standalone-gateway
ENV SDKWORK_COMMUNITY_SERVER_BIND=0.0.0.0:18094
EXPOSE 18094
CMD ["sdkwork-api-community-standalone-gateway"]
