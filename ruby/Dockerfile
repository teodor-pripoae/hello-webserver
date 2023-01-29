FROM alpine:latest

COPY . /app
WORKDIR /app

RUN apk update && \
    apk --no-cache add ruby ruby-bundler ruby-dev build-base libc-dev && \
    addgroup -S web && \
    adduser -S web -G web && \
    chown web:web /app

USER web

RUN bundle config set --local path 'vendor/bundle' && \
    bundle install --path vendor/bundle

EXPOSE 8080

CMD bin/rackup -s puma -p 8080 -o 0.0.0.0 --env production
