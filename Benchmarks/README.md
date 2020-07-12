# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |   Rust |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|-------:|--------------------:|-----------------:|
| copy1     | 5052.4 |             19449.7 |          19261.8 |
| copy2     | 5040.1 |             65471.4 |          65031.8 |
| math      |  951.5 |              6688.1 |           7156.8 |
| karplus32 |   80.5 |                78.0 |             80.9 |
| reverb    |   81.9 |                86.5 |            105.3 |

Numbers are output throughput in MB/s.

Some observations:

- Especially on the extremely lightweight / trivial DSPs, Rust suffers from bounds checks.
  On more complex DSPs like `karplus32` or `reverb` the overhead of bounds checks aren't
  too big though.

- On the `math` DSPs Rust fails to autovectorize, most likely due to bound checks as well.

- Another difference comes from avoiding fastmath optimizations. In the long term this
  could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
  (currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
