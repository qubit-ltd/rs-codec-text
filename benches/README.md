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

## Recording a baseline

For each future baseline, record the metadata below. Preserve the raw Criterion
output at a durable location and record its path. If a value cannot be
retrieved, write `unknown` and explain why; do not infer it from the report's
commit date or the current checkout.

```text
Measurement date (UTC):
Source commit (full SHA):
Source working tree (clean/dirty; if dirty, identify saved diff/artifact):
Cargo.lock snapshot (artifact location and SHA-256):
Enabled features:
rustc -Vv:
Operating system/kernel:
CPU (model and logical CPU count):
Command:
Criterion sampling parameters (sample count, warm-up, measurement time):
Fixture size (logical UTF-8 bytes):
Results (point estimate and confidence interval for each benchmark):
Raw Criterion output (durable artifact location):
```

Report point estimates with their confidence intervals. A median alone does not
establish that a difference is statistically significant.
