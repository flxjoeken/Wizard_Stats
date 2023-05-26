FROM rust:latest
LABEL authors="felix"

RUN apt update && apt upgrade && apt install sqlite3 libsqlite3-dev -y

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["wizard_stats_back"]