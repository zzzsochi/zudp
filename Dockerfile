FROM alpine as builder
WORKDIR /srv/src
COPY . /srv/src
RUN apk add --no-cache rust cargo
RUN cargo build --release


FROM alpine
LABEL maintainer="Alexander Zelenyak <zzz.sochi@gmail.com>"
RUN apk add --no-cache libgcc
COPY --from=builder /srv/src/target/release/zudp /usr/bin/zudp
ENTRYPOINT ["/usr/bin/zudp"]
