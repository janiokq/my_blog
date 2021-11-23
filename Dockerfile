#cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
LABEL maintainer="Janiokq_blog <https://github.com/janiokq/my_blog>"
RUN mkdir -p /run_space
COPY target/x86_64-unknown-linux-musl/release/my_blog /run_space
COPY config/app.toml /run_space
ENV APP_HOME=/run_space
WORKDIR /run_space

ENTRYPOINT ["./my_blog"]
