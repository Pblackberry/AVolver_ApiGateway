# ===============================
# STAGE 1: Build
# ===============================
FROM rust:1.75 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src

# Build final
RUN cargo build --release


# ===============================
# STAGE 2: Runtime
# ===============================
FROM gcr.io/distroless/cc-debian12

WORKDIR /app


COPY --from=builder /app/target/release/Avolver_ApiGateway /app/app

# Puerto informativo (Cloud Run ignora el número)
EXPOSE 8080

ENV RUST_LOG=info

CMD ["/app/app"]
