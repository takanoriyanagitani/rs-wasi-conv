FROM rust:1.63.0 AS builder
RUN curl \
  --silent \
  --show-error \
  --fail \
  --location \
  https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh \
  | bash -
RUN apt-get update
RUN apt-get install -y libclang-dev
RUN cargo install rs-wasi-conv

RUN rustup target add wasm32-wasi

WORKDIR /wasi-lib/csv2json
COPY ./wasi-csv2json/Cargo.toml ./
COPY ./wasi-csv2json/src/       ./src/
RUN cargo build --target wasm32-wasi --profile release-wasi
RUN mkdir -p /usr/share/wasm/wasi
RUN cp ./target/wasm32-wasi/release-wasi/wasi_csv2json.wasm /usr/share/wasm/wasi

WORKDIR /wasi-lib/json2cbor
COPY ./wasi-json2cbor/Cargo.toml ./
COPY ./wasi-json2cbor/src/       ./src/
RUN cargo build --target wasm32-wasi --profile release-wasi
RUN cp ./target/wasm32-wasi/release-wasi/wasi_json2cbor.wasm /usr/share/wasm/wasi

WORKDIR /wasi-lib/copy
COPY ./wasi-dummy-copy/Cargo.toml ./
COPY ./wasi-dummy-copy/src/       ./src/
RUN cargo build --target wasm32-wasi --profile release-wasi
RUN cp ./target/wasm32-wasi/release-wasi/wasi_dummy_copy.wasm /usr/share/wasm/wasi

WORKDIR /test/data
COPY ./in.csv ./in.dat

# test ok -> out.dat will be created
RUN sh -c '\
  . ~/.wasmedge/env; \
  ls \
    /usr/share/wasm/wasi/wasi_csv2json.wasm \
    /usr/share/wasm/wasi/wasi_json2cbor.wasm \
    /usr/share/wasm/wasi/wasi_dummy_copy.wasm \
  | rs-wasi-conv \
  '
RUN test -f ./out.dat
