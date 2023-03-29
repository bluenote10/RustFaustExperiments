# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |    Rust |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|--------:|--------------------:|-----------------:|
| copy1     | 37723.6 |             23097.8 |          23036.6 |
| copy2     | 36241.8 |             52926.9 |          52878.0 |
| math      |  6697.5 |              7199.6 |           7314.6 |
| delay     | 19095.4 |              4922.1 |           4916.4 |
| karplus32 |    31.6 |                61.4 |             68.8 |
| reverb    |    53.5 |                88.2 |            109.2 |

Numbers are output throughput in MiB/s, i.e., higher is better.

Versions:
- Faust version: 2.58.13
- Rust version: 1.68.0
- g++ version: 11.3.0

Note about fastmath:
In the long term this could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
(currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
