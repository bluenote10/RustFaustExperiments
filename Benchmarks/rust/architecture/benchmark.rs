#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

extern crate libm;

use faust_benchmarks::types::{FaustDsp, Meta, UI, ParamIndex};
use faust_benchmarks::benchmark_runner::run_benchmark;

// Generated intrinsics:
<<includeIntrinsic>>

// Generated class:
<<includeclass>>

const SAMPLE_RATE: i32 = 44100;

fn main() {
    println!("Size of DSP struct: {}", std::mem::size_of::<Dsp>());

    run_benchmark(|| {
        let mut dsp = Dsp::new();
        dsp.init(SAMPLE_RATE);
        Box::new(dsp)
    }, SAMPLE_RATE);
}
