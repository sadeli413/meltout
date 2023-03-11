FROM archlinux

# WARNING: This Dockerfile is for development only. Don't use this in an actual engagement

RUN pacman -Sy base-devel rustup protobuf mkcert --noconfirm && \
  useradd -m meltout

COPY . /home/meltout
RUN chown -R meltout:meltout /home/meltout
USER meltout:meltout
WORKDIR /home/meltout

RUN rustup toolchain install nightly-x86_64-unknown-linux-gnu && \
  cargo clean && \
  cargo build --release && \
  # Generate CA certs
  rm -rf certs  && \
  mkdir certs && \
  cd certs && \
  CAROOT=$PWD mkcert -ecdsa server && \
  mv server-key.pem server.key

CMD [ "/bin/bash" ]
