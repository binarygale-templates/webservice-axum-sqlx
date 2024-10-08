FROM alpine:latest as builder
RUN apk upgrade -U && apk add alpine-sdk curl git && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cargo install --locked --path . --root ./out

FROM alpine:latest
WORKDIR /app
RUN apk upgrade --no-cache && \
    addgroup -g 10001 app && adduser -u 10001 -G app -h /app -D app

USER app:app
COPY --from=builder /app/out/bin/webservice-axum-sqlx /app

ENV LISTEN [::]:8081
EXPOSE 8081
CMD ["/app/webservice-axum-sqlx"]
HEALTHCHECK CMD "wget -q http://localhost:8081/readyz || exit 1"
