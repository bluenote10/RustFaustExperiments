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
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = f32;

    fn new() -> Dsp {
        Dsp { fSampleRate: 0 }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("filename", "copy2.dsp");
        m.declare("name", "copy2");
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
    fn get_input_rate(&self, channel: i32) -> i32 {
        let mut rate: i32;
        match (channel) {
            0 => {
                rate = 1;
            }
            1 => {
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
            1 => {
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
        for i in 0..count {
            outputs[0][i as usize] = (inputs[0][i as usize] as f32);
            outputs[1][i as usize] = (inputs[1][i as usize] as f32);
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
