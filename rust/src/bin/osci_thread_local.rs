/* ------------------------------------------------------------
name: "osci"
Code generated with Faust 2.81.0 (https://faust.grame.fr)
Compilation options: -a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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

use std::cell::RefCell;
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
    iVec1: [i32; 2],
    fSampleRate: i32,
    fConst0: F32,
    fConst1: F32,
    fRec1: [F32; 2],
    fVec2: [F32; 2],
    iRec2: [i32; 2],
    fConst2: F32,
    fRec3: [F32; 2],
    fConst3: F32,
    fConst4: F32,
    fConst5: F32,
}

pub type FaustFloat = F32;

pub struct DspSIG0 {
    iVec0: [i32; 2],
    iRec0: [i32; 2],
}

impl DspSIG0 {
    fn get_num_inputsDspSIG0(&self) -> i32 {
        return 0;
    }
    fn get_num_outputsDspSIG0(&self) -> i32 {
        return 1;
    }

    pub fn instance_initDspSIG0(&mut self, sample_rate: i32) {
        for l0 in 0..2 {
            self.iVec0[l0 as usize] = 0;
        }
        for l1 in 0..2 {
            self.iRec0[l1 as usize] = 0;
        }
    }

    pub fn fillDspSIG0(&mut self, count: i32, table: &mut [FaustFloat]) {
        for i1 in 0..count {
            self.iVec0[0] = 1;
            self.iRec0[0] = (i32::wrapping_add(self.iVec0[1], self.iRec0[1])) % 65536;
            table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec0[0]) as F32);
            self.iVec0[1] = self.iVec0[0];
            self.iRec0[1] = self.iRec0[0];
        }
    }
}

pub fn newDspSIG0() -> DspSIG0 {
    DspSIG0 {
        iVec0: [0; 2],
        iRec0: [0; 2],
    }
}
thread_local! {
    static ftbl0DspSIG0: RefCell<[F32; 65536]> = RefCell::new([0.0; 65536]);
}
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
    unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
    unsafe { ffi::rintf(val) }
}

pub const FAUST_INPUTS: usize = 2;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 0;
pub const FAUST_PASSIVES: usize = 0;

impl Dsp {
    pub fn new() -> Dsp {
        Dsp {
            iVec1: [0; 2],
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fRec1: [0.0; 2],
            fVec2: [0.0; 2],
            iRec2: [0; 2],
            fConst2: 0.0,
            fRec3: [0.0; 2],
            fConst3: 0.0,
            fConst4: 0.0,
            fConst5: 0.0,
        }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("basics.lib/name", r"Faust Basic Element Library");
        m.declare("basics.lib/version", r"1.21.0");
        m.declare(
            "compile_options",
            r"-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare(
            "envelopes.lib/adsr:author",
            r"Yann Orlarey and Andrey Bundin",
        );
        m.declare("envelopes.lib/author", r"GRAME");
        m.declare("envelopes.lib/copyright", r"GRAME");
        m.declare("envelopes.lib/license", r"LGPL with exception");
        m.declare("envelopes.lib/name", r"Faust Envelope Library");
        m.declare("envelopes.lib/version", r"1.3.0");
        m.declare("filename", r"osci.dsp");
        m.declare("maths.lib/author", r"GRAME");
        m.declare("maths.lib/copyright", r"GRAME");
        m.declare("maths.lib/license", r"LGPL with exception");
        m.declare("maths.lib/name", r"Faust Math Library");
        m.declare("maths.lib/version", r"2.8.1");
        m.declare("name", r"osci");
        m.declare("oscillators.lib/name", r"Faust Oscillator Library");
        m.declare("oscillators.lib/version", r"1.6.0");
        m.declare("platform.lib/name", r"Generic Platform Library");
        m.declare("platform.lib/version", r"1.3.0");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate as i32
    }

    pub fn class_init(sample_rate: i32) {
        let mut sig0: DspSIG0 = newDspSIG0();
        sig0.instance_initDspSIG0(sample_rate);
        ftbl0DspSIG0.with_borrow_mut(|data| sig0.fillDspSIG0(65536, data));
    }
    pub fn instance_reset_params(&mut self) {}
    pub fn instance_clear(&mut self) {
        for l2 in 0..2 {
            self.iVec1[l2 as usize] = 0;
        }
        for l3 in 0..2 {
            self.fRec1[l3 as usize] = 0.0;
        }
        for l4 in 0..2 {
            self.fVec2[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.iRec2[l5 as usize] = 0;
        }
        for l6 in 0..2 {
            self.fRec3[l6 as usize] = 0.0;
        }
    }
    pub fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
        self.fConst1 = 1.0 / self.fConst0;
        self.fConst2 = 1.0 / F32::max(1.0, 0.1 * self.fConst0);
        self.fConst3 = F32::max(1.0, 0.01 * self.fConst0);
        self.fConst4 = 1.0 / self.fConst3;
        self.fConst5 = 0.7 / self.fConst3;
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
        ui_interface.open_vertical_box("osci");
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
        let [inputs0, inputs1, ..] = inputs.as_ref() else {
            panic!("wrong number of input buffers");
        };
        let inputs0 = inputs0.as_ref()[..count].iter();
        let inputs1 = inputs1.as_ref()[..count].iter();
        let [outputs0, ..] = outputs.as_mut() else {
            panic!("wrong number of output buffers");
        };
        let outputs0 = outputs0.as_mut()[..count].iter_mut();
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0);
        ftbl0DspSIG0.with_borrow(|ftbl0DspSIG0_borrow| {
            for ((input0, input1), output0) in zipped_iterators {
                self.iVec1[0] = 1;
                let mut fTemp0: F32 = (if i32::wrapping_sub(1, self.iVec1[1]) != 0 {
                    0.0
                } else {
                    self.fRec1[1] + self.fConst1 * *input1
                });
                self.fRec1[0] = fTemp0 - F32::floor(fTemp0);
                let mut fTemp1: F32 = *input0;
                self.fVec2[0] = fTemp1;
                self.iRec2[0] = ((fTemp1 == 0.0) as i32) * (i32::wrapping_add(self.iRec2[1], 1));
                self.fRec3[0] =
                    fTemp1 + self.fRec3[1] * ((self.fVec2[1] >= fTemp1) as i32) as u32 as F32;
                *output0 = F32::max(
                    0.0,
                    F32::min(
                        self.fConst4 * self.fRec3[0],
                        F32::max(1.7 - self.fConst5 * self.fRec3[0], 0.3),
                    ) * (1.0 - self.fConst2 * (self.iRec2[0]) as F32),
                ) * ftbl0DspSIG0_borrow[(std::cmp::max(
                    0,
                    std::cmp::min((65536.0 * self.fRec1[0]) as i32, 65535),
                )) as usize];

                self.iVec1[1] = self.iVec1[0];
                self.fRec1[1] = self.fRec1[0];
                self.fVec2[1] = self.fVec2[0];
                self.iRec2[1] = self.iRec2[0];
                self.fRec3[1] = self.fRec3[0];
            }
        });
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
