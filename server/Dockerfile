# dev
FROM rust:1.57.0 as develop
WORKDIR /app
# ローカル環境でサーバーを稼働させるために必要なパッケージをインストール
RUN cargo install cargo-watch
# DieselでPostgreSQLを使うために必要なパッケージをインストール
RUN apt install -y libpq-dev
# Dieselでmigrationを使うために必要なパッケージをインストール
RUN cargo install diesel_cli
COPY . .

# build
FROM develop as build
RUN update-ca-certificates
RUN cargo build --release

# prod
FROM rust:1.57.0-slim-buster as prod
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=build /app/target/release/rust-api .
CMD [ "./rust-api" ]
