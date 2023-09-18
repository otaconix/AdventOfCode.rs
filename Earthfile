VERSION 0.7
FROM rust:1.70
WORKDIR /build

build:
	COPY runner/* .
	RUN pwd && ls
	RUN cargo build --release
