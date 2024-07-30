FROM rust:1.80

ENV SQLX_OFFLINE true

copy .sqlx .sqlx
COPY src src
COPY Cargo.toml Cargo.toml
COPY .env .env

RUN cargo install --path .

CMD ["parfect-server"]
