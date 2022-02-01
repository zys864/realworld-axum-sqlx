FROM rust AS build
WORKDIR /build
COPY . .
RUN cargo build --release


FROM debian:stable-slim
WORKDIR /api_server
ENV RUST_LOG=INFO \
    SALT=db9ddb9ddb9d \
    JWT_SECRET=db9ddb9ddb9d
COPY --from=build /build/target/release/realworld-axum-sqlx .
CMD ["realworld-axum-sqlx"]

