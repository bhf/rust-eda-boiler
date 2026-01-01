# Rust EDA Boiler

![img.png](docs/img.png)

A boiler for event driven services in Rust using a DDD inspired structure with submodules. 
Please note that any domain logic included is just as a "toy" example.
### Features

* SPSC Ringbuffer using [Disruptor-rs](https://github.com/nicholassm/disruptor-rs)
* Aeron subscriptions using [Aeron-rs](https://github.com/UnitedTraders/aeron-rs/)
* Repository pattern (on heap, ```Box<>```)
* Logging with ```env_logger```
* Unit tests (inline)
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
./RustEDABoiler
```

### Docker

```
docker build -t rust-eda-app .
docker run --env USE_AERON=false rust-eda-app
```

### Sample Output

Logging at ERROR is expected based on the modulo of the Id.

```bash
[2025-12-26T13:16:06Z INFO  RustEDABoiler] Starting up Application
[2025-12-26T13:16:06Z INFO  RustEDABoiler] Total events published to ringbuffer: 1
[2025-12-26T13:16:06Z INFO  RustEDABoiler] Received event with sequence: 0, Id: 1
[2025-12-26T13:16:06Z INFO  RustEDABoiler] Processed order: 1
[2025-12-26T13:16:07Z INFO  RustEDABoiler] Total events published to ringbuffer: 2
[2025-12-26T13:16:07Z INFO  RustEDABoiler] Received event with sequence: 1, Id: 2
[2025-12-26T13:16:07Z ERROR RustEDABoiler] Error processing order: OmsHandlerError: code Invalid parameters, id 2
[2025-12-26T13:16:08Z INFO  RustEDABoiler] Total events published to ringbuffer: 3
[2025-12-26T13:16:08Z INFO  RustEDABoiler] Received event with sequence: 2, Id: 3
[2025-12-26T13:16:08Z INFO  RustEDABoiler] Processed order: 3
[2025-12-26T13:16:09Z INFO  RustEDABoiler] Total events published to ringbuffer: 4
[2025-12-26T13:16:09Z INFO  RustEDABoiler] Received event with sequence: 3, Id: 4
[2025-12-26T13:16:09Z ERROR RustEDABoiler] Error processing order: OmsHandlerError: code Invalid parameters, id 4
[2025-12-26T13:16:10Z INFO  RustEDABoiler] Total events published to ringbuffer: 5
[2025-12-26T13:16:10Z INFO  RustEDABoiler] Received event with sequence: 4, Id: 5
[2025-12-26T13:16:10Z INFO  RustEDABoiler] Processed order: 5
[2025-12-26T13:16:11Z INFO  RustEDABoiler] Received event with sequence: 5, Id: 6
[2025-12-26T13:16:11Z INFO  RustEDABoiler] Total events published to ringbuffer: 6
[2025-12-26T13:16:11Z ERROR RustEDABoiler] Error processing order: OmsHandlerError: code Invalid parameters, id 6
```

