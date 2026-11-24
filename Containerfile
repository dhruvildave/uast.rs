FROM rust:alpine AS build
WORKDIR /app
RUN apk add --no-cache make
COPY . .
RUN make && cp ./target/release/uast /bin/uast

FROM scratch AS final
COPY --from=build /bin/uast /bin/
USER 10001:10001
ENTRYPOINT ["/bin/uast"]
