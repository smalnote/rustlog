FROM rust:1.80.1-bookworm AS build

WORKDIR /app

COPY . .

RUN cargo build --release --bin prometheus_metrics_example

FROM debian:12.6-slim AS final

RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /app

COPY --from=build /app/target/release/prometheus_metrics_example /app/server
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
EXPOSE 3000 3001
ENTRYPOINT ["/app/docker-entrypoint.sh"]
CMD ["/app/server"]
