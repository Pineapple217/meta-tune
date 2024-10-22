# Build Stage
FROM rust:1.78.0 as builder

RUN USER=root cargo new --bin meta-tune
WORKDIR ./meta-tune
COPY ./Cargo.toml ./Cargo.toml
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

# Build web app with own code
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/meta_tune*
RUN cargo build --release


FROM debian:bookworm
ARG APP=/usr/src/app

RUN apt-get update && apt install -y openssl

ARG GIT_COMMIT=unspecified
LABEL org.opencontainers.image.version=$GIT_COMMIT
LABEL org.opencontainers.image.source=https://github.com/Pineapple217/meta-tune

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /meta-tune/target/release/meta-tune ${APP}/meta-tune
COPY --from=builder /meta-tune/static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./meta-tune"]
