/* ------------------------------------------------------------
name: "delay"
Code generated with Faust 2.58.13 (https://faust.grame.fr)
Compilation options: -a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -single -ftz 0
------------------------------------------------------------ */
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

extern crate libm;

use std::env;
use std::path::PathBuf;

use faust_benchmarks::benchmark_runner::run_benchmark;
use faust_benchmarks::types::{FaustDsp, Meta, ParamIndex, UI};

type F32 = f32;

// Generated intrinsics:

// Generated class:

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
pub struct Dsp {
    IOTA0: i32,
    fVec0: [F32; 2048],
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp {
            IOTA0: 0,
            fVec0: [0.0; 2048],
            fSampleRate: 0,
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare(
            "compile_options",
            "-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -single -ftz 0",
        );
        m.declare("filename", "delay.dsp");
        m.declare("name", "delay");
    }

    fn get_sample_rate(&self) -> i32 {
        return self.fSampleRate;
    }
    fn get_num_inputs(&self) -> i32 {
        return 1;
    }
    fn get_num_outputs(&self) -> i32 {
        return 1;
    }

    fn class_init(sample_rate: i32) {}
    fn instance_reset_params(&mut self) {}
    fn instance_clear(&mut self) {
        self.IOTA0 = 0;
        for l0 in 0..2048 {
            self.fVec0[(l0) as usize] = 0.0;
        }
    }
    fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
    }
    fn instance_init(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate);
        self.instance_reset_params();
        self.instance_clear();
    }
    fn init(&mut self, sample_rate: i32) {
        Dsp::class_init(sample_rate);
        self.instance_init(sample_rate);
    }

    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
        Self::build_user_interface_static(ui_interface);
    }

    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
        ui_interface.open_vertical_box("delay");
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        match param.0 {
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        match param.0 {
            _ => {}
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        let (inputs0) = if let [inputs0, ..] = inputs {
            let inputs0 = inputs0[..count as usize].iter();
            (inputs0)
        } else {
            panic!("wrong number of inputs");
        };
        let (outputs0) = if let [outputs0, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            (outputs0)
        } else {
            panic!("wrong number of outputs");
        };
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            self.fVec0[(self.IOTA0 & 2047) as usize] = *input0;
            *output0 = self.fVec0[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
        }
    }
}

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
