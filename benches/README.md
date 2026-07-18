# Charset transcode benchmarks

`transcode.rs` measures the reusable charset engine layer over one fixed,
mixed-language fixture. Data preparation is outside the timed loop, and every
case reuses its output buffer.

Run the complete benchmark with:

```bash
cargo bench --bench transcode
```

The benchmark reports logical UTF-8 fixture bytes as throughput so encode,
decode, and conversion cases remain comparable.
