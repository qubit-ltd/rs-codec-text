# Charset transcode benchmarks

`transcode.rs` measures the reusable charset engine layer over one fixed,
mixed-language fixture. Data preparation is outside the timed loop, and every
case reuses its output buffer. The UTF-8 window cases also validate the full
decoded character sequence and consumed/written totals before registering the
timed benchmark.

Both ordinary and windowed decoder cases consume each valid output slice with
`black_box(&output[..written])`. The timed loop does not traverse decoded
characters or compute a checksum; window reset/transcode/finish work remains
timed, while fixture preparation and content validation stay outside timing.

Run the complete benchmark with:

```bash
cargo bench --bench transcode
```

The benchmark reports logical UTF-8 fixture bytes as throughput so encode,
decode, and conversion cases remain comparable.
