FROM rust:slim AS chef

RUN update-ca-certificates

RUN cargo install cargo-chef
WORKDIR /xiu

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ENV USER=xiu
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY --from=planner /xiu/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --package xiu

FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /xiu

COPY --from=builder /xiu/target/release/xiu ./xiu

USER xiu:xiu

ENTRYPOINT ["/xiu/xiu"]
