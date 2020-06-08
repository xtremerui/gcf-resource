FROM rustlang/rust:nightly-alpine
COPY . /src
WORKDIR /src

RUN cargo build --bins --out-dir /assets -Z unstable-options
