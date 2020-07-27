#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

extern crate libm;

use faust_benchmarks::benchmark_runner::run_benchmark;
use faust_benchmarks::types::{FaustDsp, Meta, ParamIndex, UI};

// Generated intrinsics:

// Generated class:

pub struct Dsp {
    IOTA: i32,
    fVec0: [f32; 2048],
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = f32;

    fn new() -> Dsp {
        Dsp {
            IOTA: 0,
            fVec0: [0.0; 2048],
            fSampleRate: 0,
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
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
    fn get_input_rate(&self, channel: i32) -> i32 {
        let mut rate: i32;
        match (channel) {
            0 => {
                rate = 1;
            }
            _ => {
                rate = -1;
            }
        }
        return rate;
    }
    fn get_output_rate(&self, channel: i32) -> i32 {
        let mut rate: i32;
        match (channel) {
            0 => {
                rate = 1;
            }
            _ => {
                rate = -1;
            }
        }
        return rate;
    }

    fn class_init(sample_rate: i32) {}
    fn instance_reset_params(&mut self) {}
    fn instance_clear(&mut self) {
        self.IOTA = 0;
        for l0 in 0..2048 {
            self.fVec0[l0 as usize] = 0.0;
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
            self.fVec0[(self.IOTA & 2047) as usize] = (*input0 as f32);
            *output0 = (self.fVec0[((self.IOTA - 1024) & 2047) as usize] as f32);
            self.IOTA = (self.IOTA + 1);
        }
    }
}

const SAMPLE_RATE: i32 = 44100;

fn main() {
    println!("Size of DSP struct: {}", std::mem::size_of::<Dsp>());

    run_benchmark(
        || {
            let mut dsp = Dsp::new();
            dsp.init(SAMPLE_RATE);
            Box::new(dsp)
        },
        SAMPLE_RATE,
    );
}
