FROM docker.io/rust

LABEL \
	org.opencontainers.image.title="GitHub action to verify bibtex cite keys match a file" \
	org.opencontainers.image.authors="Emanuel Buholzer <emanuel0xb@gmail.com>" \
	org.opencontainers.image.source="https://github.com/emanuelbuholzer/cite-key-files"

COPY src /root/src
COPY Cargo.lock Cargo.toml /root/

WORKDIR /root
RUN cargo install --path .

ENTRYPOINT ["cite-key-files"]
