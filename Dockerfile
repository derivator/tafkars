FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Ensure working C compile setup (not installed by default in arm64 images)
RUN apt update && apt install build-essential -y

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build the app
RUN cargo build --release


FROM debian:bullseye-20230612-slim AS runtime

# Use an unprivileged user.
RUN adduser -c 'tafkars user' tafkars --home /nonexistent --no-create-home --disabled-password
# Install ca-certificates for proxy to work
RUN apt update && apt install ca-certificates -y && rm -rf /var/lib/apt/lists/*

WORKDIR /app

USER tafkars

# Copy resources
COPY --from=builder /app/target/release/tafkars-lemmy /usr/local/bin

EXPOSE 8000
ENTRYPOINT ["tafkars-lemmy"]