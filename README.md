# Zudp

Simle tool for receive UDP datagrams.

## Use cases

My use case is mock datadog-agent statsd server on develop cluster.

## Project status

Unsupported endless pre-alpha. Not for production use!

## Receive

```
$ zudp listen 127.0.0.1:8125
127.0.0.1:57083 13 "some.data:666"
127.0.0.1:64879 13 "some.data:666"
127.0.0.1:59449 13 "some.data:666"
```

## Sent test datagram

```
$ zudp send 127.0.0.1:8125 some.data:666
$ zudp send 127.0.0.1:8125 some.data:666
$ zudp send 127.0.0.1:8125 some.data:666
```

## Docker

https://hub.docker.com/r/zzzsochi/zudp
