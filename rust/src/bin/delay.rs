/* ------------------------------------------------------------
name: "delay"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
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

pub type FaustFloat = F32;
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

pub const FAUST_INPUTS: usize = 1;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 0;
pub const FAUST_PASSIVES: usize = 0;

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct Dsp {
    IOTA0: i32,
    fVec0: [F32; 2048],
    fSampleRate: i32,
}

impl Dsp {
    pub fn new() -> Dsp {
        Dsp {
            IOTA0: 0,
            fVec0: [0.0; 2048],
            fSampleRate: 0,
        }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare(
            "compile_options",
            r"-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare("filename", r"delay.dsp");
        m.declare("name", r"delay");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate as i32
    }

    pub fn class_init(sample_rate: i32) {}
    pub fn instance_reset_params(&mut self) {}
    pub fn instance_clear(&mut self) {
        self.IOTA0 = 0;
        for l0 in 0..2048 {
            self.fVec0[l0 as usize] = 0.0;
        }
    }
    pub fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
    }
    pub fn instance_init(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate);
        self.instance_reset_params();
        self.instance_clear();
    }
    pub fn init(&mut self, sample_rate: i32) {
        Dsp::class_init(sample_rate);
        self.instance_init(sample_rate);
    }

    pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
        Self::build_user_interface_static(ui_interface);
    }

    pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
        ui_interface.open_vertical_box("delay");
        ui_interface.close_box();
    }

    pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
        match param.0 {
            _ => None,
        }
    }

    pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
        match param.0 {
            _ => {}
        }
    }

    pub fn compute_arrays(&mut self, count: usize, inputs: &[&[FaustFloat]; 1], outputs: &mut [&mut [FaustFloat]; 1]) {
        let [inputs0] = inputs;
        let inputs0 = inputs0[..count].iter();
        let [outputs0] = outputs;
        let outputs0 = outputs0[..count].iter_mut();
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            self.fVec0[(self.IOTA0 & 2047) as usize] = *input0;
            *output0 = self.fVec0[((i32::wrapping_sub(self.IOTA0, 1024)) & 2047) as usize];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
        }
    }

    pub fn compute(&mut self, count: usize, inputs: &[&[FaustFloat]], outputs: &mut [&mut [FaustFloat]]) {
        let input_array = inputs.split_at(1).0.try_into().expect("too few input buffers");
        let output_array = outputs.split_at_mut(1).0.try_into().expect("too few output buffers");
        self.compute_arrays(count, input_array, output_array);
    }
}

impl FaustDsp for Dsp {
    type T = FaustFloat;
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }
    fn metadata(&self, m: &mut dyn Meta) {
        self.metadata(m)
    }
    fn get_sample_rate(&self) -> i32 {
        self.get_sample_rate()
    }
    fn get_num_inputs(&self) -> i32 {
        FAUST_INPUTS as i32
    }
    fn get_num_outputs(&self) -> i32 {
        FAUST_OUTPUTS as i32
    }
    fn class_init(sample_rate: i32)
    where
        Self: Sized,
    {
        Self::class_init(sample_rate);
    }
    fn instance_reset_params(&mut self) {
        self.instance_reset_params()
    }
    fn instance_clear(&mut self) {
        self.instance_clear()
    }
    fn instance_constants(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate)
    }
    fn instance_init(&mut self, sample_rate: i32) {
        self.instance_init(sample_rate)
    }
    fn init(&mut self, sample_rate: i32) {
        self.init(sample_rate)
    }
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
        self.build_user_interface(ui_interface)
    }
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized,
    {
        Self::build_user_interface_static(ui_interface);
    }
    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        self.get_param(param)
    }
    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        self.set_param(param, value)
    }
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        self.compute(count as usize, inputs, outputs)
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
