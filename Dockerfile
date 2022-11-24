FROM archlinux

# WARNING: This Dockerfile is for development only. Don't use this in an actual engagement

RUN pacman -Syu --noconfirm && \
    pacman -S base-devel rust protobuf mkcert --noconfirm && \
    useradd -m meltout

COPY . /home/meltout
RUN chown -R meltout:meltout /home/meltout
USER meltout:meltout
WORKDIR /home/meltout

RUN cargo clean && \
    cargo build && \
    # Generate CA certs
    rm -rf certs  && \
    mkdir certs && \
    cd certs && \
    CAROOT=$PWD mkcert -ecdsa randomthinghere.server && \
    mv randomthinghere.server.pem server.pem && \
    mv randomthinghere.server-key.pem server.key

CMD [ "/bin/bash" ]
