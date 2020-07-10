#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(bare_trait_objects)]


extern crate libm;

use std::fs::File;
use std::io::Write;

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str) -> ();
}

pub trait UI<T> {
    // -- widget's layouts
    fn openTabBox(&mut self, label: &str) -> ();
    fn openHorizontalBox(&mut self, label: &str) -> ();
    fn openVerticalBox(&mut self, label: &str) -> ();
    fn closeBox(&mut self) -> ();

    // -- active widgets
    fn addButton(&mut self, label: &str, zone: &mut T) -> ();
    fn addCheckButton(&mut self, label: &str, zone: &mut T) -> ();
    fn addVerticalSlider(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T) -> ();
    fn addHorizontalSlider(&mut self, label: &str, zone: &mut T , init: T, min: T, max: T, step: T) -> ();
    fn addNumEntry(&mut self, label: &str, zone: &mut T, init: T, min: T, max: T, step: T) -> ();

    // -- passive widgets
    fn addHorizontalBargraph(&mut self, label: &str, zone: &mut T, min: T, max: T) -> ();
    fn addVerticalBargraph(&mut self, label: &str, zone: &mut T, min: T, max: T) -> ();

    // -- metadata declarations
    fn declare(&mut self, zone: &mut T, key: &str, value: &str) -> ();
}

// Generated intrinsics:
<<includeIntrinsic>>

// Generated class:
<<includeclass>>

const SAMPLE_RATE: i32 = 44100;

type FloatType = f32;

fn run_dsp(output_file: &mut File, num_samples: usize) {

    // Generation constants
    let buffer_size = 64usize;

    // Init dsp
    let mut dsp = Box::new(mydsp::new());
    dsp.init(SAMPLE_RATE);

    let num_inputs = dsp.getNumInputs() as usize;
    let num_outputs = dsp.getNumOutputs() as usize;

    // Prepare buffers
    let mut in_buffer = vec![vec![0 as FloatType; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0 as FloatType; buffer_size]; num_outputs];

    // Compute
    let mut num_samples_written = 0;
    while num_samples_written < num_samples {

        let buffer_size = buffer_size.min(num_samples - num_samples_written);

        // handle inputs
        for c in 0..num_inputs {
            for j in 0..buffer_size {
                let first_frame = num_samples_written == 0 && j == 0;
                in_buffer[c][j] = if first_frame { 1.0 } else { 0.0 };
            }
        }

        dsp.compute(
            buffer_size as i32,
            in_buffer.iter().map(|buffer| buffer.as_slice()).collect::<Vec<&[FloatType]>>().as_slice(),
            out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [FloatType]>>().as_mut_slice(),
        );

        // handle outputs
        for j in 0..buffer_size {
            for c in 0..num_outputs {
                if c > 0 {
                    write!(output_file, ";").unwrap();
                }
                write!(output_file, "{}", out_buffer[c][j]).unwrap();
            }
            writeln!(output_file).unwrap();
            num_samples_written += 1;
        }
    }
}

fn main() {
    let num_samples = 44100;

    // Open output file
    let mut output_file = File::create("/tmp/dsp_debug_output.txt").expect("Cannot create output file");

    run_dsp(&mut output_file, num_samples);
}
