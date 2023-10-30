VERSION 0.7
FROM rust:1.73
WORKDIR /build

install-chef:
	RUN cargo install --locked cargo-chef

prepare-cache:
	FROM +install-chef
	COPY --dir Cargo.toml Cargo.lock crates years runner .
	RUN cargo chef prepare
	SAVE ARTIFACT recipe.json

build-cache:
	FROM +install-chef
	COPY +prepare-cache/recipe.json ./
	CACHE target
	RUN cargo chef cook --release

build:
	FROM +build-cache
	COPY --dir Cargo.* crates years runner .
	RUN cargo build --frozen --offline --release

run-all:
	ARG --required AOC_CONTACT_INFO
	ARG --required AOC_SESSION

	FROM +build
	CACHE /root/.cache/aoc-runner
	FOR year IN $(ls years)
		FOR day IN $(ls "years/${year}")
			RUN echo "Running aoc-${year}-${day}"
			RUN ./target/release/aoc-runner ${year} ${day} ./target/release/aoc-${year}-${day}
		END
	END
