FROM rust:slim-bookworm as build

RUN USER=root cargo new --bin neuron-info-website
WORKDIR /neuron-info-website

COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/

RUN cargo build --release

FROM bitnami/minideb:bookworm

WORKDIR /

COPY --from=build /neuron-info-website/target/release/neuron-info-website neuron-info-website
COPY ./static/ ./static/

EXPOSE 8080

CMD ["./neuron-info-website"]
