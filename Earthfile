VERSION 0.7
FROM rust:1.70
WORKDIR /build

build:
	COPY --dir runner/* .
	RUN pwd && ls
	RUN cargo build --release
