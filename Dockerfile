################
##### Builder
FROM rust as builder

# pre build app
WORKDIR /
RUN USER=root cargo new app
COPY Cargo.toml Rocket.toml /app/
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# build app
COPY src /app/src/
RUN touch /app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM gcr.io/distroless/static-debian11 as runner

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/multi_page_website /
WORKDIR /app
COPY ./static /app/static
COPY ./templates /app/templates

# set run env
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/multi_page_website"]