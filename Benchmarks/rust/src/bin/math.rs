/* ------------------------------------------------------------
name: "math"
Code generated with Faust 2.59.4 (https://faust.grame.fr)
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
#[repr(C)]
pub struct Dsp {
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp { fSampleRate: 0 }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare(
            "compile_options",
            "-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -single -ftz 0",
        );
        m.declare("filename", "math.dsp");
        m.declare("name", "math");
    }

    fn get_sample_rate(&self) -> i32 {
        return self.fSampleRate;
    }
    fn get_num_inputs(&self) -> i32 {
        return 8;
    }
    fn get_num_outputs(&self) -> i32 {
        return 1;
    }

    fn class_init(sample_rate: i32) {}
    fn instance_reset_params(&mut self) {}
    fn instance_clear(&mut self) {}
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
        ui_interface.open_vertical_box("math");
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
        let (inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7) =
            if let [inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, ..] = inputs {
                let inputs0 = inputs0[..count as usize].iter();
                let inputs1 = inputs1[..count as usize].iter();
                let inputs2 = inputs2[..count as usize].iter();
                let inputs3 = inputs3[..count as usize].iter();
                let inputs4 = inputs4[..count as usize].iter();
                let inputs5 = inputs5[..count as usize].iter();
                let inputs6 = inputs6[..count as usize].iter();
                let inputs7 = inputs7[..count as usize].iter();
                (inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7)
            } else {
                panic!("wrong number of inputs");
            };
        let (outputs0) = if let [outputs0, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            (outputs0)
        } else {
            panic!("wrong number of outputs");
        };
        let zipped_iterators = inputs0
            .zip(inputs1)
            .zip(inputs2)
            .zip(inputs3)
            .zip(inputs4)
            .zip(inputs5)
            .zip(inputs6)
            .zip(inputs7)
            .zip(outputs0);
        for ((((((((input0, input1), input2), input3), input4), input5), input6), input7), output0) in zipped_iterators
        {
            *output0 = (*input2 + *input3) * (*input0 + *input1) / ((*input6 + *input7) * (*input4 + *input5));
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
