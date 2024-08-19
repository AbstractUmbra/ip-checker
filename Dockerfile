FROM rust:latest AS builder

ARG APP_NAME=ip-checker
ENV APP_NAME=${APP_NAME}

WORKDIR /app

RUN apt-get update -y && apt-get upgrade -y && apt-get install musl-tools -y && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ && echo "fn main() {}" > src/main.rs
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target x86_64-unknown-linux-musl
COPY src src
RUN touch src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN strip target/x86_64-unknown-linux-musl/release/$APP_NAME


FROM alpine:latest

LABEL org.opencontainers.image.source=https://github.com/abstractumbra/ip-checker
LABEL org.opencontainers.image.description="Home network IP checker."
LABEL org.opencontainers.image.licenses=ARR

ARG APP_NAME=ip-checker
ENV APP_NAME=${APP_NAME}
ENV ROCKET_PROFILE=release

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/$APP_NAME .

CMD [ "./$APP_NAME" ]
