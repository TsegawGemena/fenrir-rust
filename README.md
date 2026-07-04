# Fenirer

**Edge telemetry and event stream decoder** for the FENR/FN2R wire formats used by
field gateways, audit collectors, and IoT batch-uplink pipelines.

Fenirer accepts arbitrary chunk sizes from TCP sockets, serial bridges, and UDP
datagram reassembly — the common case on constrained edge hosts where full frames
rarely arrive in one read.

## Use cases

- **Sensor gateways** batch ADC readings, GPS fixes, and GPIO events into FENR
  frames before uplink over LTE or LoRa backhaul.
- **Audit collectors** append tamper-evident FN2R records with device metadata
  and CRC32 checksums for compliance pipelines.
- **Replay tooling** indexes decoded batches via the catalog/seek API and writes
  FARC archives for offline analysis.

## Quick start

```rust
use fenirer::{decode_all, DecodeSession, StreamDecoder};

// Decode a complete buffer
let frames = decode_all(&wire_bytes)?;

// Incremental decode from a socket
let mut session = DecodeSession::new();
for chunk in read_chunks {
    for event in session.feed(&chunk)? {
        if let fenirer::DecodeEvent::Frame(frame) = event {
            process(&frame.payload);
        }
    }
}

// Stream wrapper with backpressure
let mut stream = StreamDecoder::new(4096);
let frames = stream.push(&chunk)?;
```

## Wire formats

| Version | Magic | Purpose |
|---------|-------|---------|
| v1 | `FENR` | Field Event Network Record — compact sensor/audit batches |
| v2 | `FN2R` | Field Network Record — v1 + optional metadata (device ID, flags) |

v1 layout (little-endian):

```text
[ "FENR" ][ u32 len ][ payload ][ u32 xor-checksum ]
```

## Build & test

```bash
cargo test
```

## Architecture

| Layer | Modules | Responsibility |
|-------|---------|----------------|
| Decode core | `session`, `parser`, `buffer`, `finalize` | Incremental frame parsing with pooled buffers |
| Wire | `wire`, `codec`, `encode`, `protocol` | v1/v2 framing, checksums, negotiation |
| Ingest | `stream`, `recovery`, `service` | Streaming decode, resync, multi-tenant routing |
| Storage | `catalog`, `archive` | Frame indexing, seek, FARC archives |
| Validation | `schema`, `filter`, `analysis` | Typed payload checks, filtering, histograms |
| Pipeline | `pipeline`, `transform`, `fsm` | Composable transforms and state-machine tooling |

## License

MIT
