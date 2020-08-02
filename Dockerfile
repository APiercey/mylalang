FROM rust:1-alpine3.11 as builder

RUN mkdir /builder

WORKDIR /builder

COPY . /builder

RUN cargo update

RUN cargo build --release

FROM alpine:3.11.0 as mylalang

COPY --from=builder /builder/target/release/mylalang /bin/
COPY --from=builder /builder/target/release/imy      /bin/
