# This form has working dependency caching with chef.
# You have to deal with copying every Cargo.* file to make the dependency caching work.
# You also need to make dummy main.rs and lib.rs files so that changes to them aren't passed through.
# Depending on how big your images get, copy can get quite slow as well.
# None of this is terrible, but with a large project, it can be a pain.

FROM rust:1.49-slim-buster
WORKDIR /earthbuild

install-chef:
    # Installing in debug just to make this run faster for sample.
    RUN cargo install --debug cargo-chef

prepare-cache:
    FROM +install-chef
    # When something changes here, all the copies and moves can be slow.
    COPY Cargo.toml Cargo.lock ./
    COPY bin/Cargo.toml ./bin/
    COPY lib_a/Cargo.toml ./lib_a/
    COPY lib_b/Cargo.toml ./lib_b/
    COPY lib_c/Cargo.toml ./lib_c/
    RUN mkdir -p ./bin/src && touch ./bin/src/main.rs
    RUN mkdir -p ./lib_a/src && touch ./lib_a/src/lib.rs
    RUN mkdir -p ./lib_b/src && touch ./lib_b/src/lib.rs
    RUN mkdir -p ./lib_c/src && touch ./lib_c/src/lib.rs
    RUN cargo chef prepare
    SAVE ARTIFACT recipe.json

build-cache:
    FROM +install-chef
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home

build:
    COPY +build-cache/target target 
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY --dir bin lib_a lib_b lib_c Cargo.lock Cargo.toml ./
    RUN cargo build
    RUN cargo test --no-run

test:
    FROM +build
    RUN cargo test