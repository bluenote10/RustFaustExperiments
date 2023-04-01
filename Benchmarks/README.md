# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |   Rust (latest) |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|----------------:|--------------------:|-----------------:|
| copy1     |         37723.6 |             23097.8 |          23036.6 |
| copy2     |         36241.8 |             52926.9 |          52878.0 |
| math      |          6697.5 |              7199.6 |           7314.6 |
| delay     |         19095.4 |              4922.1 |           4916.4 |
| karplus32 |            31.6 |                61.4 |             68.8 |
| reverb    |            53.5 |                88.2 |            109.2 |

Numbers are output throughput in MiB/s, i.e., higher is better.

Versions:
- Faust version: 2.58.13
- Rust version: 1.68.0
- g++ version: 11.3.0

**Note about fastmath:**
In the long term this could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
(currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).

## Performance regression

It looks like [Rust version 1.67.0](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1670-2023-01-26)
has lead to a performance regression. Comparing 1.66.0 and 1.67.0 directly:

|           |   Rust (1.67.0) |   Rust (1.66.0) |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|----------------:|----------------:|--------------------:|-----------------:|
| copy1     |         38558.3 |         38737.5 |             23097.8 |          23036.6 |
| copy2     |         35880.5 |         35690.7 |             52926.9 |          52878.0 |
| math      |          6644.2 |          6710.4 |              7199.6 |           7314.6 |
| delay     |         19124.8 |          9833.7 |              4922.1 |           4916.4 |
| karplus32 |            31.4 |            88.6 |                61.4 |             68.8 |
| reverb    |            54.2 |            90.2 |                88.2 |            109.2 |
