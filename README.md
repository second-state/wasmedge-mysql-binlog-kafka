# wasmedge-mysql-binlog-kafka

A WasmEdge app to send MySQL binlog json events to apache kafka

## Build a Wasm image and publish it to Docker hub

```bash
docker buildx build --provenance=false --platform wasi/wasm -t secondstate/mysql-binlog-kafka .
docker push secondstate/mysql-binlog-kafka
```

