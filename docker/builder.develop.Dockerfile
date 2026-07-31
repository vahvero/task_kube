FROM rust:slim-bookworm

RUN cargo --version && apt-get update && apt-get install -y git bash curl build-essential libssl-dev pkg-config

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

RUN node -v && npm -v
RUN npm install -g @angular/cli
WORKDIR /app
