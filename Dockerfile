############### Planner stage ###############
FROM rust:1.49 AS planner

WORKDIR /app

RUN cargo install cargo-chef

# Copy all files from our working environment
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json


############### Cacher stage ###############
FROM rust:1.49 AS cacher

WORKDIR /app

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json


############### Builder stage ###############

# We use the latest Rust stable release as base image
FROM rust:1.49 AS builder

WORKDIR /app

# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

# Enforce sqlx offline mode
ENV SQLX_OFFLINE true

COPY . .

# Build out application, leveraging the cached deps
RUN cargo build --release --bin zero2prod


############### Runtime stage ###############

FROM debian:buster-slim AS runtime

WORKDIR /app

# Install OpenSSL - it is dynamically linked by some the dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Using production environment
ENV APP_ENVIRONMENT production

# Lauch our binary
ENTRYPOINT ["./zero2prod"]

# We need the configuration file at runtime
COPY configuration configuration

# Copy the compiled binary from the builder environment
COPY --from=builder /app/target/release/zero2prod zero2prod
