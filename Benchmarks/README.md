# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |    Rust |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|--------:|--------------------:|-----------------:|
| copy1     | 38431.7 |             23175.0 |          22867.3 |
| copy2     | 35446.9 |             53261.6 |          52886.2 |
| math      |  6722.6 |              7206.3 |           7306.8 |
| delay     | 18913.8 |              4845.9 |           4926.2 |
| karplus32 |    31.8 |                59.6 |             67.3 |
| reverb    |    53.9 |                87.0 |            110.6 |

Numbers are output throughput in MiB/s, i.e., higher is better.

Faust version: 2.56.1
Rust version: 1.68.0
g++ version: 11.3.0

Note about fastmath:
In the long term this could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
(currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
