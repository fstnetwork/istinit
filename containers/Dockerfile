FROM rust:alpine AS builder

RUN apk add --no-cache -U musl-dev file && \
  rustup default stable && \
  rustup component add rustfmt;

WORKDIR /build
COPY . /build

ARG RELEASE=1

ENV RUST_BACKTRACE=1

RUN set -eux; \
  if [ "${RELEASE}" == "1" ]; then \
    cargo build --release; \
    cp target/release/istinit /usr/bin; \
    strip /usr/bin/istinit; \
  else \
    cargo build ; \
    cp target/debug/istinit /usr/bin; \
  fi; \
  file /usr/bin/istinit; \
  ldd /usr/bin/istinit;


FROM scratch

COPY --from=builder /usr/bin/istinit /usr/bin/

ENTRYPOINT [ "/usr/bin/istinit" ]
