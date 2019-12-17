ARG BASE_CONTAINER=rust:1.39-stretch
FROM $BASE_CONTAINER

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get install -yq --no-install-recommends \
        postgresql \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/db

COPY . .

RUN cargo install --path .
