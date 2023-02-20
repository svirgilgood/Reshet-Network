FROM rust:1.60 as builder

RUN USER=root cargo new --bin codex
WORKDIR ./codex
RUN cargo build --release
RUN rm src/*.rs


ADD . ./

RUN rm ./target/release/deps/codex*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app


RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080


ENV TZ=Etx/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /codex/target/release/codex ${APP}

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

WORKDIR ${APP}

CMD ["./codex"]