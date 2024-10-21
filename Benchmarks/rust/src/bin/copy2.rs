/* ------------------------------------------------------------
name: "copy2"
Code generated with Faust 2.75.12 (https://faust.grame.fr)
Compilation options: -a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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

use std::convert::TryInto;
mod ffi {
    use std::os::raw::c_float;
    // Conditionally compile the link attribute only on non-Windows platforms
    #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
    extern "C" {
        pub fn remainderf(from: c_float, to: c_float) -> c_float;
        pub fn rintf(val: c_float) -> c_float;
    }
}
fn remainder_f32(from: f32, to: f32) -> f32 {
    unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
    unsafe { ffi::rintf(val) }
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct Dsp {
    fSampleRate: i32,
}

impl Dsp {
    fn compute_arrays(&mut self, count: i32, inputs: &[&[F32]; 2], outputs: &mut [&mut [F32]; 2]) {
        let [inputs0, inputs1] = inputs;
        let inputs0 = inputs0[..count as usize].iter();
        let inputs1 = inputs1[..count as usize].iter();
        let [outputs0, outputs1] = outputs;
        let outputs0 = outputs0[..count as usize].iter_mut();
        let outputs1 = outputs1[..count as usize].iter_mut();
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            *output0 = *input0;
            *output1 = *input1;
        }
    }
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp { fSampleRate: 0 }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare(
            "compile_options",
            r"-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare("filename", r"copy2.dsp");
        m.declare("name", r"copy2");
    }

    fn get_sample_rate(&self) -> i32 {
        return self.fSampleRate;
    }
    fn get_num_inputs(&self) -> i32 {
        return 2;
    }
    fn get_num_outputs(&self) -> i32 {
        return 2;
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
        ui_interface.open_vertical_box("copy2");
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
        let input_array = inputs.split_at(2).0.try_into().expect("too few input buffers");
        let output_array = outputs.split_at_mut(2).0.try_into().expect("too few output buffers");
        self.compute_arrays(count, input_array, output_array);
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
