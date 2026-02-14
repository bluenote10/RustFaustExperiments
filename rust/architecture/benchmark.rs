#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unexpected_cfgs)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use std::env;
use std::path::PathBuf;

use faust_benchmarks::types::{FaustDsp, Meta, UI, ParamIndex};
use faust_benchmarks::benchmark_runner::run_benchmark;

type F32 = f32;
type FaustFloat = f32;

// Generated intrinsics:
<<includeIntrinsic>>

// Generated class:
<<includeclass>>

const SAMPLE_RATE: i32 = 44100;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let result_file = PathBuf::from(args[1].clone());

    println!("Size of DSP struct: {}", std::mem::size_of::<Dsp>());

    run_benchmark(
        || {
            let mut dsp = Dsp::new();
            dsp.init(SAMPLE_RATE);
            Box::new(dsp)
        },
        SAMPLE_RATE,
        &result_file,
    );
}
