ARG TARGETPLATFORM
FROM --platform=$TARGETPLATFORM ubuntu:focal
RUN apt update \
  && apt install -y --no-install-recommends libinput10 \
  && apt clean \
  && rm -rf /var/lib/apt/lists/*
VOLUME ["/var/lib/btknmle"]
COPY ./target/*/release/btknmle /
CMD ["/btknmle"]
