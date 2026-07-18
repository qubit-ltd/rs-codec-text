# Charset transcode baseline — 2026-07-18

Environment: Linux 6.17, Intel Core i5-9600K (6 CPUs), Rust 1.94.0.
Criterion used 20 samples, a 2-second warm-up, and a 5-second
measurement window. Times below are median point estimates for one fixture of
174,080 logical UTF-8 bytes; throughput remains available in Criterion's
generated report.

| Operation | Median |
| --- | ---: |
| encode UTF-8 | 196.801 µs |
| encode UTF-16 (`u16`) | 202.033 µs |
| encode UTF-32 (`u32`) | 15.665 µs |
| decode UTF-8 | 702.918 µs |
| decode UTF-16 (`u16`) | 344.641 µs |
| decode UTF-32 (`u32`) | 109.046 µs |
| UTF-8 → UTF-16 | 1.336 ms |
| UTF-16 → UTF-8 | 1.203 ms |
| UTF-8 → UTF-32 | 1.014 ms |
| UTF-32 → UTF-8 | 1.179 ms |

Command:

```shell
cargo bench --bench transcode -- --noplot
```
