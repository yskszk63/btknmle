ARG TARGETPLATFORM
FROM --platform=$TARGETPLATFORM debian:buster-slim
RUN apt update \
  && apt install -y --no-install-recommends libinput10 \
  && apt clean \
  && rm -rf /var/lib/apt/lists/*
VOLUME ["/var/lib/btknmle"]
COPY .target/btknmle /btknmle
CMD ["/btknmle"]
