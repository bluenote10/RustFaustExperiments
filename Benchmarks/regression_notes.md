# Regression notes

## Regression 1

After switching to [Rust version 1.67.0](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1670-2023-01-26)
there was a performance regression in the benchmarks.

This issue has now been solved, for details see:
- [Rust internals thread](https://internals.rust-lang.org/t/unexpected-3-x-performance-regression-starting-with-rust-version-1-67/18724/4)
- [Deleted question on SO](https://stackoverflow.com/questions/75905096/what-might-have-caused-performance-regression-in-rust-version-1-67)

Below is the stuff collected during the investigation. Now outdated, but keeping for reference.


### Summary

Comparing 1.66.0 and 1.67.0 directly:

|           |   Rust (1.67.0) |   Rust (1.66.0) |   C++ (no fastmath) |   C++ (fastmath) |
|:----------|----------------:|----------------:|--------------------:|-----------------:|
| copy1     |         38558.3 |         38737.5 |             23097.8 |          23036.6 |
| copy2     |         35880.5 |         35690.7 |             52926.9 |          52878.0 |
| math      |          6644.2 |          6710.4 |              7199.6 |           7314.6 |
| delay     |         19124.8 |          9833.7 |              4922.1 |           4916.4 |
| karplus32 |            31.4 |            88.6 |                61.4 |             68.8 |
| reverb    |            54.2 |            90.2 |                88.2 |            109.2 |


### Investigation

It looks like the Rust performance in particular of the karplus32 dsp regressed
over time.

Around Faust version 2.26.0 and the (early) `rust-wip` branch the throughput was ~85 MiB/s,
but now it is ~31 MiB/s.

Trying to replicate the old performance now seems tricky.


#### Faust versions

Attempt at bisecting the regression on Faust side:

```
Faust versions:

tag: 2.41.1 => 30.9 MiB/s
tag: 2.33.1 => 30.8 MiB/s
tag: 2.30.5 => 31.5 MiB/s
tag: 2.27.2 => 33.2 MiB/s
tag: 2.27.1 => 31.1 MiB/s
branch: rust-wip, sha: 2599686ce => 30.8 MiB/s   (that was the last commit on that branch)
sha: 32846af52 => 32.8 MiB/s (to replicate version 2.26.0, which doesn't have a tag)
```

On Faust side, versions 2.30.5 and earlier need a manual fixing of the JSONUI.h header to make
them actually compile (just add `#include <limits>`, no clue how that officially compiled back
then).

On first glance it looks like all Faust versions now end up in the 30 MiB/s area, no matter what.


#### Rust versions

Current Rust version is ~1.68.0.

The old benchmarks were run in mid 2020. Checking the old releases (https://github.com/rust-lang/rust/blob/master/RELEASES.md)
suggest that this must have been in the 1.40 or so.

Trying 1.43.0 in combination with the 2.26.0 Faust branch immediately reproduces the old 83.8 MiB/s.

Switching back to the latest Faust version also produces 86.7 MiB/s. So bisecting:

```
1.68.0 => 31.6 MiB/s
1.67.0 => 30.9 MiB/s
1.66.0 => 88.5 MiB/s
1.65.0 => 87.7 MiB/s
1.62.0 => 83.9 MiB/s
1.56.0 => 85.6 MiB/s
1.50.0 => 86.1 MiB/s
1.43.0 => 86.7 MiB/s
```

Conclusion: The regression seems to be introduced in Rust version 1.67.

Steps to try out old Rust versions:

```sh
# EDIT: Installing the version manually is actually not needed, because using
# RUSTUP_TOOLCHAIN=... will implicitly install it. But cleaning up afterwards
# could still make sense, because the ad-hoc installed versions seem to be kept.
rustup install 1.43.0
rustup toolchain list

# The CARGO_NET_GIT_FETCH_WITH_CLI was necessary for some reason, because
# otherwise I was getting an error related to fetching the libm dependency, see:
# https://github.com/rust-lang/cargo/issues/10303
RUSTUP_TOOLCHAIN=1.43.0 CARGO_NET_GIT_FETCH_WITH_CLI=true ./gen_and_run_all.sh

# When done with experiments, it makes sense to clean up:
rustup uninstall 1.43.0
```

# Regression 2

After updating

- Faust version: 2.72.11 -> 2.75.12
- Rust version: 1.77.0 -> 1.81.0

The throughput of the "delay" benchmark dropped from 12820.8 to 5499.7.

Result of a small investigation: Since the codegen has almost not changed,
it was likely that it is a Rust version thing.

Bisecting indicates it was a regression from 1.77.2 to 1.78.0:

```sh
# This runs at ~12.8 GB/sec
RUSTUP_TOOLCHAIN=1.77.2 cargo run --bin delay --release -- rust.json
# This runs at ~5.5 GB/sec
RUSTUP_TOOLCHAIN=1.78.0 cargo run --bin delay --release -- rust.json
```

Also, there seems to be no difference in using the Faust codegen from 2.72.11 vs 2.75.12.


# Legacy code snippets

On Rust side, compiling older versions requires to switch out various versions of the trait
definitions:

```rust
// Some variants also require to adapt benchmark_runners.rs where clause to:
// F: Fn() -> Box<dyn FaustDsp<Sample = FloatType>>

// Should work for rust-wip
pub trait FaustDsp {
    type T;

    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]);

    fn get_input_rate(&self, channel: i32) -> i32;
    fn get_output_rate(&self, channel: i32) -> i32;
    fn instance_reset_params(&mut self);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized;

    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
}

// Should work for 2.26.0
pub trait FaustDsp {
    type Sample;

    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&mut self, ui_interface: &mut dyn UI<Self::Sample>);
    fn compute(
        &mut self,
        count: i32,
        inputs: &[&[Self::Sample]],
        outputs: &mut [&mut [Self::Sample]],
    );

    fn get_input_rate(&self, channel: i32) -> i32;
    fn get_output_rate(&self, channel: i32) -> i32;
    fn instance_reset_user_interface(&mut self);
}


// Should work for rust-wip
pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}

// Should work for 2.26.0, and versions that require the old &mut parameter passing.
pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: &mut f32);
    fn add_check_button(&mut self, label: &str, param: &mut f32);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: &mut f32,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: &mut f32,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: &mut f32, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: &mut f32, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: &mut f32, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: &mut f32, key: &str, value: &str);
}

```
