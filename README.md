# Rust EDA Boiler

![img.png](docs/img.png)

A boiler for event driven services in Rust using a DDD inspired structure with submodules.

### Features

* SPSC Ringbuffer using [Disruptor-rs](https://github.com/nicholassm/disruptor-rs)
* Aeron subscriptions using [Rusteron](https://github.com/gsrxyz/rusteron/tree/main)
* Repository pattern (on heap ```Box<>```)
* Logging with ```env_logger```
* Unit tests
* Integration tests (in ```tests/```)
* ```Dockerfile```

## How To Run

Run using:

```
RUST_LOG=info cargo run
```

Or:

```
cargo build --release
cd target/release/
export RUST_LOG=info
./oms
```

### Docker

```
docker build -t rust-oms-app .
docker run --env USE_AERON=false rust-oms-app
```

### Sample Output

Error logging is expected based on the modulo of the Id.

```bash
[2025-12-26T10:28:14Z INFO  oms] Starting up OMS
[2025-12-26T10:28:14Z INFO  oms] Initialised ringbuffer, will now publish messages. Use Aeron: false
[2025-12-26T10:28:14Z INFO  oms] Total events published to ringbuffer: 1
[2025-12-26T10:28:14Z INFO  oms] Received event with sequence: 0, Id: 1
[2025-12-26T10:28:14Z INFO  oms] Processed order: 1
[2025-12-26T10:28:15Z INFO  oms] Total events published to ringbuffer: 2
[2025-12-26T10:28:15Z INFO  oms] Received event with sequence: 1, Id: 2
[2025-12-26T10:28:15Z ERROR oms] Error processing order: OmsHandlerError: code Invalid parameters, id 2
[2025-12-26T10:28:16Z INFO  oms] Received event with sequence: 2, Id: 3
[2025-12-26T10:28:16Z INFO  oms] Total events published to ringbuffer: 3
[2025-12-26T10:28:16Z INFO  oms] Processed order: 3
[2025-12-26T10:28:17Z INFO  oms] Received event with sequence: 3, Id: 4
[2025-12-26T10:28:17Z INFO  oms] Total events published to ringbuffer: 4
[2025-12-26T10:28:17Z ERROR oms] Error processing order: OmsHandlerError: code Invalid parameters, id 4
[2025-12-26T10:28:18Z INFO  oms] Received event with sequence: 4, Id: 5
[2025-12-26T10:28:18Z INFO  oms] Total events published to ringbuffer: 5
[2025-12-26T10:28:18Z INFO  oms] Processed order: 5
[2025-12-26T10:28:19Z INFO  oms] Total events published to ringbuffer: 6
[2025-12-26T10:28:19Z INFO  oms] Received event with sequence: 5, Id: 6
[2025-12-26T10:28:19Z ERROR oms] Error processing order: OmsHandlerError: code Invalid parameters, id 6
```

