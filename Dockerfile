FROM rust:1.79 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/fabric_treasury /app/fabric_treasury
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/app/fabric_treasury"]
