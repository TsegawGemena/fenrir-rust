# Fenirer

Incremental streaming frame decoder with buffer pooling, zero-copy views, and
multi-protocol wire support (~87 KB source).

## Wire formats

**v1**
```text
[ "FENR" ][ u32 len LE ][ payload ][ u32 xor-checksum LE ]
```

**v2** (optional metadata block, selectable checksum)
```text
[ "FN2R" ][ u32 len LE ][ MD metadata? ][ payload ][ checksum LE ]
```

## Build

```bash
cargo test
cargo test --features buggy   # stale-view regression path
```

## Fuzzing

```bash
# requires nightly + cargo-fuzz, or build harness directly:
cargo check -p fenirer-fuzz

# with cargo-fuzz installed:
cargo fuzz run decode_fuzzer
cargo fuzz run chunked_fuzzer
```

Harnesses live under `fuzz/`:
- `fuzz/fenirer_fuzzer.rs` — single-chunk incremental decode
- `fuzz/fuzz_targets/decode_fuzzer.rs` — decode_all + session feed
- `fuzz/fuzz_targets/chunked_fuzzer.rs` — fragmented input across chunk boundaries

## Modules

- `buffer` — pool, shared buffers, views, arena, slab, region registry
- `parser` — stage machine, validation, checkpoints
- `session` — incremental feed, pipeline, metrics, config
- `finalize` — single buffer release point
- `wire` — v1/v2 frame builders, metadata, flags
- `codec` — XOR + CRC32 checksums
- `encode` — encoder, streaming/batch encoding
- `recovery` — resync scanner, skip policies
- `protocol` — v1/v2 negotiation
- `util` — byte helpers, reader
- `invariants` — debug lifetime checks
- `fuzz/` — libfuzzer harnesses for chunked decode

## Chunked-input bug

See **[docs/BUG_ANALYSIS.md](docs/BUG_ANALYSIS.md)** for the full root-cause trace,
ownership flow, bug classification, and systemic fix.

With `--features buggy`, the session returns pooled memory while parser views still
alias it. Default build pins views via `Arc`, splits payload/trailer buffers, and
enforces invariants in `src/invariants.rs`.

## License

MIT
