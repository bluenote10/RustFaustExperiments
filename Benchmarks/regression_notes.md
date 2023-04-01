# Regression notes

It looks like the Rust performance in particular of the karplus32 dsp regressed
over time.

Around Faust version 2.26.0 and the (early) `rust-wip` branch the throughput was ~85 MiB/s,
but now it is ~31 MiB/s.

Trying to replicate the old performance now seems tricky. Attempt at bisecting the regression:


## Faust versions

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

## Rust versions

Current Rust version is ~1.68.0.

The old benchmarks were run in mid 2020. Checking the old releases (https://github.com/rust-lang/rust/blob/master/RELEASES.md)
suggest that this must have been in the 1.40 or so.

Trying 1.43.0 in combination with the 2.26.0 Faust branch immediately reproduces the old 83.8 MiB/s.

Steps to try out old Rust versions:

```sh
rustup install 1.43.0
rustup toolchain list

# The CARGO_NET_GIT_FETCH_WITH_CLI was necessary for some reason, because
# otherwise I was getting an error related to fetching the libm dependency, see:
# https://github.com/rust-lang/cargo/issues/10303
RUSTUP_TOOLCHAIN=1.43.0 CARGO_NET_GIT_FETCH_WITH_CLI=true ./gen_and_run_all.sh
```


## Legacy code snippets

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
