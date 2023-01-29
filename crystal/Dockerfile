FROM crystallang/crystal:1-alpine as builder
WORKDIR /src
COPY . .
RUN crystal build --release --static app.cr -o /src/webserver

FROM busybox
WORKDIR /app
COPY --from=builder /src/webserver /app/webserver
EXPOSE 8080
ENTRYPOINT ["/app/webserver"]
