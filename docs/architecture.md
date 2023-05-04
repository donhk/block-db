### Client

```
chunks[] <--- document
```

### Server

```
chunks[] ---> server ---> fs
```

### Protocol

```
t0  client -(I want to upload)-> server
t1  client <-(sessid)- server
t2  client -[chunks..]-> server
t3  client <-[ack]- server
```