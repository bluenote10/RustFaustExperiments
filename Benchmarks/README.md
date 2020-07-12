# Benchmarks

DSP throughput comparison of Rust vs C++:

|           |     Rust |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|---------:|--------------------:|-----------------:|
| copy1     | 5052.425 |           19449.679 |        19261.800 |
| copy2     | 5040.145 |           65471.413 |        65031.774 |
| math      |  951.463 |            6688.054 |         7156.849 |
| karplus32 |   80.531 |              77.956 |           80.897 |
| reverb    |   81.858 |              86.474 |          105.350 |

Numbers are output throughput in MB/s.

Some observations:

- Especially on the extremely lightweight / trivial DSPs, Rust suffers from bounds checks.
  On more complex DSPs like `karplus32` or `reverb` the overhead of bounds checks aren't
  too big though.

- On the `math` DSPs Rust fails to autovectorize, most likely due to bound checks as well.

- Another difference comes from avoiding fastmath optimizations. In the long term this
  could be solved by using [fastmath intrinsics](https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html)
  (currently a nightly feature) or [future higher-level solutions](https://github.com/rust-lang/rust/issues/21690).
