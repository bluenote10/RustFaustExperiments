/* ------------------------------------------------------------
name: "math"
Code generated with Faust 2.83.10 (https://faust.grame.fr)
Compilation options: -a ./architecture/benchmark.rs -lang rust -fpga-mem-th 4 -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unexpected_cfgs)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]

extern crate libm;

use std::env;
use std::path::PathBuf;

use faust_benchmarks::benchmark_runner::run_benchmark;
use faust_benchmarks::types::{FaustDsp, Meta, ParamIndex, UI};

type F32 = f32;

// Generated intrinsics:

// Generated class:

#[repr(C)]
pub struct Dsp {
    fSampleRate: i32,
}

pub type FaustFloat = F32;
#[cfg(not(target_arch = "wasm32"))] // Compile ffi bindings only on non-wasm targets
mod ffi {
    use std::os::raw::c_float;
    // Conditionally compile the link attribute only on non-Windows platforms
    #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
    unsafe extern "C" {
        pub fn remainderf(from: c_float, to: c_float) -> c_float;
        pub fn rintf(val: c_float) -> c_float;
    }
}
fn remainder_f32(from: f32, to: f32) -> f32 {
    #[cfg(not(target_arch = "wasm32"))] // non-wasm targets use ffi bindings
    unsafe {
        ffi::remainderf(from, to)
    }
    #[cfg(target_arch = "wasm32")] // wasm relies on libm
    libm::remainderf(from, to)
}
fn rint_f32(val: f32) -> f32 {
    #[cfg(not(target_arch = "wasm32"))] // non-wasm targets use ffi bindings
    unsafe {
        ffi::rintf(val)
    }
    #[cfg(target_arch = "wasm32")] // wasm relies on libm
    libm::rintf(val)
}

pub const FAUST_INPUTS: usize = 8;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 0;
pub const FAUST_PASSIVES: usize = 0;

impl Dsp {
    pub fn new() -> Dsp {
        Dsp { fSampleRate: 0 }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("compile_options", r"-a ./architecture/benchmark.rs -lang rust -fpga-mem-th 4 -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
        m.declare("filename", r"math.dsp");
        m.declare("name", r"math");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate as i32
    }

    pub fn class_init(sample_rate: i32) {
        // Obtaining locks on 0 static var(s)
    }
    pub fn instance_reset_params(&mut self) {}
    pub fn instance_clear(&mut self) {}
    pub fn instance_constants(&mut self, sample_rate: i32) {
        // Obtaining locks on 0 static var(s)
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
        ui_interface.open_vertical_box("math");
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

    pub fn compute(
        &mut self,
        count: usize,
        inputs: &[impl AsRef<[FaustFloat]>],
        outputs: &mut [impl AsMut<[FaustFloat]>],
    ) {
        // Obtaining locks on 0 static var(s)
        let [inputs0, inputs1, inputs2, inputs3, inputs4, inputs5, inputs6, inputs7, ..] = inputs.as_ref() else {
            panic!("wrong number of input buffers");
        };
        let inputs0 = inputs0.as_ref()[..count].iter();
        let inputs1 = inputs1.as_ref()[..count].iter();
        let inputs2 = inputs2.as_ref()[..count].iter();
        let inputs3 = inputs3.as_ref()[..count].iter();
        let inputs4 = inputs4.as_ref()[..count].iter();
        let inputs5 = inputs5.as_ref()[..count].iter();
        let inputs6 = inputs6.as_ref()[..count].iter();
        let inputs7 = inputs7.as_ref()[..count].iter();
        let [outputs0, ..] = outputs.as_mut() else {
            panic!("wrong number of output buffers");
        };
        let outputs0 = outputs0.as_mut()[..count].iter_mut();
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
