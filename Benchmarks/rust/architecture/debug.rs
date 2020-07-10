/************************************************************************
************************************************************************
    FAUST Architecture File
    Copyright (C) 2017 GRAME, Centre National de Creation Musicale
    ---------------------------------------------------------------------

    This is sample code. This file is provided as an example of minimal
    FAUST architecture file. Redistribution and use in source and binary
    forms, with or without modification, in part or in full are permitted.
    In particular you can create a derived work of this FAUST architecture
    and distribute that work under terms of your choice.

    This sample code is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

extern crate libm;

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


fn main() {
    let SAMPLE_RATE = 44100;

    // Init dsp
    let mut dsp = Box::new(Dsp::new());
    dsp.init(SAMPLE_RATE as i32);

    // Prepare buffers
    let buffer_size = 64usize;
    let mut in_buffer0 = vec![0 as f32; buffer_size];
    let mut in_buffer1 = vec![0 as f32; buffer_size];
    let mut out_buffer0 = vec![0 as f32; buffer_size];
    let mut out_buffer1 = vec![0 as f32; buffer_size];

    for i in 0..buffer_size {
        in_buffer0[i] = 0.0;
    }
    for i in 0..buffer_size {
        in_buffer1[i] = 440.0;
    }

    // Compute
    dsp.compute(
        buffer_size as i32,
        &[&in_buffer0, &in_buffer1],
        &mut[&mut out_buffer0, &mut out_buffer1],
    );

    // Check
    println!("Input 0: {:?}", in_buffer0);
    println!("Input 1: {:?}", in_buffer1);
    println!("Output 0: {:?}", out_buffer0);
    println!("Output 1: {:?}", out_buffer1);
}
