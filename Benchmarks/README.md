# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |   Rust (`rust-wip`) |   Rust (`master-dev`) |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|--------------------:|----------------------:|--------------------:|-----------------:|
| copy1     |             36549.4 |                5052.4 |             19449.7 |          19261.8 |
| copy2     |             41046.5 |                5040.1 |             65471.4 |          65031.8 |
| math      |              6537.1 |                 951.5 |              6688.1 |           7156.8 |
| delay     |              1948.2 |                     - |              4703.0 |           4742.7 |
| karplus32 |                85.6 |                  85.0 |                78.0 |             80.9 |
| reverb    |                82.7 |                  81.9 |                86.5 |            105.3 |

Numbers are output throughput in MiB/s, i.e., higher is better.

Faust version: 2.26.0 (or the `rust-wip` branch)

Some observations:

- On `master-dev` Rust suffers some bounds checks (preventing auto-vectorization) on the extremely
  lightweight / trivial DSPs.
  This is solved on the `rust-wip` branch, which uses iterators instead.

- Without fast-math, the performance of Rust and C++ are basically the same.

  In the long term this could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
  (currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
