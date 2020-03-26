FROM --platform=$TARGETPLATFORM rust as builder
RUN apt update \
    && apt install -y --no-install-recommends \
       libinput-dev \
       libudev-dev \
       llvm \
       clang \
    && mkdir /build
WORKDIR /build
COPY . .
RUN cd btknmle && cargo install --path .

FROM --platform=$TARGETPLATFORM debian:buster-slim
RUN apt update \
  && apt install -y --no-install-recommends libinput10 \
  && apt clean \
  && rm -rf /var/lib/apt/lists/*
VOLUME ["/var/lib/btknmle"]
COPY --from=builder /usr/local/cargo/bin/btknmle /
CMD ["/btknmle"]
