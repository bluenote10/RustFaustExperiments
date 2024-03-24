/* ------------------------------------------------------------
author: "Grame"
copyright: "(c)GRAME 2006"
license: "BSD"
name: "karplus32"
version: "1.0"
Code generated with Faust 2.72.11 (https://faust.grame.fr)
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

mod ffi {
    use std::os::raw::c_float;
    #[link(name = "m")]
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
    fHslider0: F32,
    iRec1: [i32; 2],
    fHslider1: F32,
    fButton0: F32,
    fVec0: [F32; 2],
    fRec2: [F32; 2],
    fHslider2: F32,
    IOTA0: i32,
    fVec1: [F32; 8192],
    fHslider3: F32,
    fHslider4: F32,
    fRec0: [F32; 3],
    fHslider5: F32,
    fVec2: [F32; 8192],
    fRec3: [F32; 3],
    fVec3: [F32; 8192],
    fRec4: [F32; 3],
    fVec4: [F32; 8192],
    fRec5: [F32; 3],
    fVec5: [F32; 8192],
    fRec6: [F32; 3],
    fVec6: [F32; 8192],
    fRec7: [F32; 3],
    fVec7: [F32; 8192],
    fRec8: [F32; 3],
    fVec8: [F32; 8192],
    fRec9: [F32; 3],
    fVec9: [F32; 8192],
    fRec10: [F32; 3],
    fVec10: [F32; 8192],
    fRec11: [F32; 3],
    fVec11: [F32; 8192],
    fRec12: [F32; 3],
    fVec12: [F32; 8192],
    fRec13: [F32; 3],
    fVec13: [F32; 4096],
    fRec14: [F32; 3],
    fVec14: [F32; 4096],
    fRec15: [F32; 3],
    fVec15: [F32; 2048],
    fRec16: [F32; 3],
    fVec16: [F32; 512],
    fRec17: [F32; 3],
    fHslider6: F32,
    fVec17: [F32; 8192],
    fRec18: [F32; 3],
    fVec18: [F32; 8192],
    fRec19: [F32; 3],
    fVec19: [F32; 8192],
    fRec20: [F32; 3],
    fVec20: [F32; 8192],
    fRec21: [F32; 3],
    fVec21: [F32; 8192],
    fRec22: [F32; 3],
    fVec22: [F32; 8192],
    fRec23: [F32; 3],
    fVec23: [F32; 8192],
    fRec24: [F32; 3],
    fVec24: [F32; 8192],
    fRec25: [F32; 3],
    fVec25: [F32; 8192],
    fRec26: [F32; 3],
    fVec26: [F32; 8192],
    fRec27: [F32; 3],
    fVec27: [F32; 8192],
    fRec28: [F32; 3],
    fVec28: [F32; 8192],
    fRec29: [F32; 3],
    fVec29: [F32; 4096],
    fRec30: [F32; 3],
    fVec30: [F32; 4096],
    fRec31: [F32; 3],
    fVec31: [F32; 2048],
    fRec32: [F32; 3],
    fVec32: [F32; 1024],
    fRec33: [F32; 3],
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp {
            fHslider0: 0.0,
            iRec1: [0; 2],
            fHslider1: 0.0,
            fButton0: 0.0,
            fVec0: [0.0; 2],
            fRec2: [0.0; 2],
            fHslider2: 0.0,
            IOTA0: 0,
            fVec1: [0.0; 8192],
            fHslider3: 0.0,
            fHslider4: 0.0,
            fRec0: [0.0; 3],
            fHslider5: 0.0,
            fVec2: [0.0; 8192],
            fRec3: [0.0; 3],
            fVec3: [0.0; 8192],
            fRec4: [0.0; 3],
            fVec4: [0.0; 8192],
            fRec5: [0.0; 3],
            fVec5: [0.0; 8192],
            fRec6: [0.0; 3],
            fVec6: [0.0; 8192],
            fRec7: [0.0; 3],
            fVec7: [0.0; 8192],
            fRec8: [0.0; 3],
            fVec8: [0.0; 8192],
            fRec9: [0.0; 3],
            fVec9: [0.0; 8192],
            fRec10: [0.0; 3],
            fVec10: [0.0; 8192],
            fRec11: [0.0; 3],
            fVec11: [0.0; 8192],
            fRec12: [0.0; 3],
            fVec12: [0.0; 8192],
            fRec13: [0.0; 3],
            fVec13: [0.0; 4096],
            fRec14: [0.0; 3],
            fVec14: [0.0; 4096],
            fRec15: [0.0; 3],
            fVec15: [0.0; 2048],
            fRec16: [0.0; 3],
            fVec16: [0.0; 512],
            fRec17: [0.0; 3],
            fHslider6: 0.0,
            fVec17: [0.0; 8192],
            fRec18: [0.0; 3],
            fVec18: [0.0; 8192],
            fRec19: [0.0; 3],
            fVec19: [0.0; 8192],
            fRec20: [0.0; 3],
            fVec20: [0.0; 8192],
            fRec21: [0.0; 3],
            fVec21: [0.0; 8192],
            fRec22: [0.0; 3],
            fVec22: [0.0; 8192],
            fRec23: [0.0; 3],
            fVec23: [0.0; 8192],
            fRec24: [0.0; 3],
            fVec24: [0.0; 8192],
            fRec25: [0.0; 3],
            fVec25: [0.0; 8192],
            fRec26: [0.0; 3],
            fVec26: [0.0; 8192],
            fRec27: [0.0; 3],
            fVec27: [0.0; 8192],
            fRec28: [0.0; 3],
            fVec28: [0.0; 8192],
            fRec29: [0.0; 3],
            fVec29: [0.0; 4096],
            fRec30: [0.0; 3],
            fVec30: [0.0; 4096],
            fRec31: [0.0; 3],
            fVec31: [0.0; 2048],
            fRec32: [0.0; 3],
            fVec32: [0.0; 1024],
            fRec33: [0.0; 3],
            fSampleRate: 0,
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("author", r"Grame");
        m.declare(
            "compile_options",
            r"-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare("copyright", r"(c)GRAME 2006");
        m.declare("delays.lib/name", r"Faust Delay Library");
        m.declare("delays.lib/version", r"1.1.0");
        m.declare("filename", r"karplus32.dsp");
        m.declare("license", r"BSD");
        m.declare("name", r"karplus32");
        m.declare("noises.lib/name", r"Faust Noise Generator Library");
        m.declare("noises.lib/version", r"1.4.1");
        m.declare("version", r"1.0");
    }

    fn get_sample_rate(&self) -> i32 {
        return self.fSampleRate;
    }
    fn get_num_inputs(&self) -> i32 {
        return 0;
    }
    fn get_num_outputs(&self) -> i32 {
        return 2;
    }

    fn class_init(sample_rate: i32) {}
    fn instance_reset_params(&mut self) {
        self.fHslider0 = 0.1;
        self.fHslider1 = 128.0;
        self.fButton0 = 0.0;
        self.fHslider2 = 0.5;
        self.fHslider3 = 32.0;
        self.fHslider4 = 128.0;
        self.fHslider5 = 1.0;
        self.fHslider6 = 0.5;
    }
    fn instance_clear(&mut self) {
        for l0 in 0..2 {
            self.iRec1[l0 as usize] = 0;
        }
        for l1 in 0..2 {
            self.fVec0[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.fRec2[l2 as usize] = 0.0;
        }
        self.IOTA0 = 0;
        for l3 in 0..8192 {
            self.fVec1[l3 as usize] = 0.0;
        }
        for l4 in 0..3 {
            self.fRec0[l4 as usize] = 0.0;
        }
        for l5 in 0..8192 {
            self.fVec2[l5 as usize] = 0.0;
        }
        for l6 in 0..3 {
            self.fRec3[l6 as usize] = 0.0;
        }
        for l7 in 0..8192 {
            self.fVec3[l7 as usize] = 0.0;
        }
        for l8 in 0..3 {
            self.fRec4[l8 as usize] = 0.0;
        }
        for l9 in 0..8192 {
            self.fVec4[l9 as usize] = 0.0;
        }
        for l10 in 0..3 {
            self.fRec5[l10 as usize] = 0.0;
        }
        for l11 in 0..8192 {
            self.fVec5[l11 as usize] = 0.0;
        }
        for l12 in 0..3 {
            self.fRec6[l12 as usize] = 0.0;
        }
        for l13 in 0..8192 {
            self.fVec6[l13 as usize] = 0.0;
        }
        for l14 in 0..3 {
            self.fRec7[l14 as usize] = 0.0;
        }
        for l15 in 0..8192 {
            self.fVec7[l15 as usize] = 0.0;
        }
        for l16 in 0..3 {
            self.fRec8[l16 as usize] = 0.0;
        }
        for l17 in 0..8192 {
            self.fVec8[l17 as usize] = 0.0;
        }
        for l18 in 0..3 {
            self.fRec9[l18 as usize] = 0.0;
        }
        for l19 in 0..8192 {
            self.fVec9[l19 as usize] = 0.0;
        }
        for l20 in 0..3 {
            self.fRec10[l20 as usize] = 0.0;
        }
        for l21 in 0..8192 {
            self.fVec10[l21 as usize] = 0.0;
        }
        for l22 in 0..3 {
            self.fRec11[l22 as usize] = 0.0;
        }
        for l23 in 0..8192 {
            self.fVec11[l23 as usize] = 0.0;
        }
        for l24 in 0..3 {
            self.fRec12[l24 as usize] = 0.0;
        }
        for l25 in 0..8192 {
            self.fVec12[l25 as usize] = 0.0;
        }
        for l26 in 0..3 {
            self.fRec13[l26 as usize] = 0.0;
        }
        for l27 in 0..4096 {
            self.fVec13[l27 as usize] = 0.0;
        }
        for l28 in 0..3 {
            self.fRec14[l28 as usize] = 0.0;
        }
        for l29 in 0..4096 {
            self.fVec14[l29 as usize] = 0.0;
        }
        for l30 in 0..3 {
            self.fRec15[l30 as usize] = 0.0;
        }
        for l31 in 0..2048 {
            self.fVec15[l31 as usize] = 0.0;
        }
        for l32 in 0..3 {
            self.fRec16[l32 as usize] = 0.0;
        }
        for l33 in 0..512 {
            self.fVec16[l33 as usize] = 0.0;
        }
        for l34 in 0..3 {
            self.fRec17[l34 as usize] = 0.0;
        }
        for l35 in 0..8192 {
            self.fVec17[l35 as usize] = 0.0;
        }
        for l36 in 0..3 {
            self.fRec18[l36 as usize] = 0.0;
        }
        for l37 in 0..8192 {
            self.fVec18[l37 as usize] = 0.0;
        }
        for l38 in 0..3 {
            self.fRec19[l38 as usize] = 0.0;
        }
        for l39 in 0..8192 {
            self.fVec19[l39 as usize] = 0.0;
        }
        for l40 in 0..3 {
            self.fRec20[l40 as usize] = 0.0;
        }
        for l41 in 0..8192 {
            self.fVec20[l41 as usize] = 0.0;
        }
        for l42 in 0..3 {
            self.fRec21[l42 as usize] = 0.0;
        }
        for l43 in 0..8192 {
            self.fVec21[l43 as usize] = 0.0;
        }
        for l44 in 0..3 {
            self.fRec22[l44 as usize] = 0.0;
        }
        for l45 in 0..8192 {
            self.fVec22[l45 as usize] = 0.0;
        }
        for l46 in 0..3 {
            self.fRec23[l46 as usize] = 0.0;
        }
        for l47 in 0..8192 {
            self.fVec23[l47 as usize] = 0.0;
        }
        for l48 in 0..3 {
            self.fRec24[l48 as usize] = 0.0;
        }
        for l49 in 0..8192 {
            self.fVec24[l49 as usize] = 0.0;
        }
        for l50 in 0..3 {
            self.fRec25[l50 as usize] = 0.0;
        }
        for l51 in 0..8192 {
            self.fVec25[l51 as usize] = 0.0;
        }
        for l52 in 0..3 {
            self.fRec26[l52 as usize] = 0.0;
        }
        for l53 in 0..8192 {
            self.fVec26[l53 as usize] = 0.0;
        }
        for l54 in 0..3 {
            self.fRec27[l54 as usize] = 0.0;
        }
        for l55 in 0..8192 {
            self.fVec27[l55 as usize] = 0.0;
        }
        for l56 in 0..3 {
            self.fRec28[l56 as usize] = 0.0;
        }
        for l57 in 0..8192 {
            self.fVec28[l57 as usize] = 0.0;
        }
        for l58 in 0..3 {
            self.fRec29[l58 as usize] = 0.0;
        }
        for l59 in 0..4096 {
            self.fVec29[l59 as usize] = 0.0;
        }
        for l60 in 0..3 {
            self.fRec30[l60 as usize] = 0.0;
        }
        for l61 in 0..4096 {
            self.fVec30[l61 as usize] = 0.0;
        }
        for l62 in 0..3 {
            self.fRec31[l62 as usize] = 0.0;
        }
        for l63 in 0..2048 {
            self.fVec31[l63 as usize] = 0.0;
        }
        for l64 in 0..3 {
            self.fRec32[l64 as usize] = 0.0;
        }
        for l65 in 0..1024 {
            self.fVec32[l65 as usize] = 0.0;
        }
        for l66 in 0..3 {
            self.fRec33[l66 as usize] = 0.0;
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
        ui_interface.open_vertical_box("karplus32");
        ui_interface.open_vertical_box("excitator");
        ui_interface.add_horizontal_slider("excitation (samples)", ParamIndex(0), 128.0, 2.0, 512.0, 1.0);
        ui_interface.add_button("play", ParamIndex(1));
        ui_interface.close_box();
        ui_interface.open_vertical_box("noise generator");
        ui_interface.add_horizontal_slider("level", ParamIndex(2), 0.5, 0.0, 1.0, 0.1);
        ui_interface.close_box();
        ui_interface.add_horizontal_slider("output volume", ParamIndex(3), 0.5, 0.0, 1.0, 0.1);
        ui_interface.open_vertical_box("resonator x32");
        ui_interface.add_horizontal_slider("attenuation", ParamIndex(4), 0.1, 0.0, 1.0, 0.01);
        ui_interface.add_horizontal_slider("detune", ParamIndex(5), 32.0, 0.0, 512.0, 1.0);
        ui_interface.add_horizontal_slider("duration (samples)", ParamIndex(6), 128.0, 2.0, 512.0, 1.0);
        ui_interface.add_horizontal_slider("polyphony", ParamIndex(7), 1.0, 0.0, 32.0, 1.0);
        ui_interface.close_box();
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        match param.0 {
            1 => Some(self.fButton0),
            4 => Some(self.fHslider0),
            0 => Some(self.fHslider1),
            2 => Some(self.fHslider2),
            5 => Some(self.fHslider3),
            6 => Some(self.fHslider4),
            7 => Some(self.fHslider5),
            3 => Some(self.fHslider6),
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        match param.0 {
            1 => self.fButton0 = value,
            4 => self.fHslider0 = value,
            0 => self.fHslider1 = value,
            2 => self.fHslider2 = value,
            5 => self.fHslider3 = value,
            6 => self.fHslider4 = value,
            7 => self.fHslider5 = value,
            3 => self.fHslider6 = value,
            _ => {}
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            let outputs1 = outputs1[..count as usize].iter_mut();
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let mut fSlow0: F32 = 0.5 * (1.0 - self.fHslider0);
        let mut fSlow1: F32 = 1.0 / self.fHslider1;
        let mut fSlow2: F32 = self.fButton0;
        let mut fSlow3: F32 = 4.656613e-10 * self.fHslider2;
        let mut fSlow4: F32 = self.fHslider3;
        let mut fSlow5: F32 = self.fHslider4;
        let mut iSlow6: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 3e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow7: F32 = self.fHslider5;
        let mut fSlow8: F32 = ((fSlow7 > 3e+01) as i32) as u32 as F32;
        let mut iSlow9: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 28.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow10: F32 = ((fSlow7 > 28.0) as i32) as u32 as F32;
        let mut iSlow11: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 26.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow12: F32 = ((fSlow7 > 26.0) as i32) as u32 as F32;
        let mut iSlow13: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 24.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow14: F32 = ((fSlow7 > 24.0) as i32) as u32 as F32;
        let mut iSlow15: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 22.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow16: F32 = ((fSlow7 > 22.0) as i32) as u32 as F32;
        let mut iSlow17: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 2e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow18: F32 = ((fSlow7 > 2e+01) as i32) as u32 as F32;
        let mut iSlow19: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 18.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow20: F32 = ((fSlow7 > 18.0) as i32) as u32 as F32;
        let mut iSlow21: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 16.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow22: F32 = ((fSlow7 > 16.0) as i32) as u32 as F32;
        let mut iSlow23: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 14.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow24: F32 = ((fSlow7 > 14.0) as i32) as u32 as F32;
        let mut iSlow25: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 12.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow26: F32 = ((fSlow7 > 12.0) as i32) as u32 as F32;
        let mut iSlow27: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 1e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow28: F32 = ((fSlow7 > 1e+01) as i32) as u32 as F32;
        let mut iSlow29: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 8.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow30: F32 = ((fSlow7 > 8.0) as i32) as u32 as F32;
        let mut iSlow31: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 6.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow32: F32 = ((fSlow7 > 6.0) as i32) as u32 as F32;
        let mut iSlow33: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 4.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow34: F32 = ((fSlow7 > 4.0) as i32) as u32 as F32;
        let mut iSlow35: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 2.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow36: F32 = ((fSlow7 > 2.0) as i32) as u32 as F32;
        let mut iSlow37: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + -1.5))) as i32;
        let mut fSlow38: F32 = ((fSlow7 > 0.0) as i32) as u32 as F32;
        let mut fSlow39: F32 = self.fHslider6;
        let mut iSlow40: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 31.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow41: F32 = ((fSlow7 > 31.0) as i32) as u32 as F32;
        let mut iSlow42: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 29.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow43: F32 = ((fSlow7 > 29.0) as i32) as u32 as F32;
        let mut iSlow44: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 27.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow45: F32 = ((fSlow7 > 27.0) as i32) as u32 as F32;
        let mut iSlow46: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 25.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow47: F32 = ((fSlow7 > 25.0) as i32) as u32 as F32;
        let mut iSlow48: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 23.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow49: F32 = ((fSlow7 > 23.0) as i32) as u32 as F32;
        let mut iSlow50: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 21.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow51: F32 = ((fSlow7 > 21.0) as i32) as u32 as F32;
        let mut iSlow52: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 19.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow53: F32 = ((fSlow7 > 19.0) as i32) as u32 as F32;
        let mut iSlow54: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 17.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow55: F32 = ((fSlow7 > 17.0) as i32) as u32 as F32;
        let mut iSlow56: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 15.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow57: F32 = ((fSlow7 > 15.0) as i32) as u32 as F32;
        let mut iSlow58: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 13.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow59: F32 = ((fSlow7 > 13.0) as i32) as u32 as F32;
        let mut iSlow60: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 11.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow61: F32 = ((fSlow7 > 11.0) as i32) as u32 as F32;
        let mut iSlow62: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 9.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow63: F32 = ((fSlow7 > 9.0) as i32) as u32 as F32;
        let mut iSlow64: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 7.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow65: F32 = ((fSlow7 > 7.0) as i32) as u32 as F32;
        let mut iSlow66: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 5.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow67: F32 = ((fSlow7 > 5.0) as i32) as u32 as F32;
        let mut iSlow68: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + 3.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow69: F32 = ((fSlow7 > 3.0) as i32) as u32 as F32;
        let mut iSlow70: i32 = (F32::min(4096.0, F32::max(0.0, fSlow4 + fSlow5 + -1.5))) as i32;
        let mut fSlow71: F32 = ((fSlow7 > 1.0) as i32) as u32 as F32;
        let zipped_iterators = outputs0.zip(outputs1);
        for (output0, output1) in zipped_iterators {
            self.iRec1[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec1[1]), 12345);
            self.fVec0[0] = fSlow2;
            self.fRec2[0] = self.fRec2[1] + (((fSlow2 - self.fVec0[1]) > 0.0) as i32) as u32 as F32
                - fSlow1 * ((self.fRec2[1] > 0.0) as i32) as u32 as F32;
            let mut fTemp0: F32 =
                fSlow3 * (((self.fRec2[0] > 0.0) as i32) as u32 as F32 + 1.5258789e-05) * (self.iRec1[0]) as F32;
            self.fVec1[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec0[1] + self.fRec0[2]);
            self.fRec0[0] = self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow6)) & 8191) as usize];
            self.fVec2[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec3[1] + self.fRec3[2]);
            self.fRec3[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow9)) & 8191) as usize];
            self.fVec3[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec4[1] + self.fRec4[2]);
            self.fRec4[0] = self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow11)) & 8191) as usize];
            self.fVec4[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec5[1] + self.fRec5[2]);
            self.fRec5[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow13)) & 8191) as usize];
            self.fVec5[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec6[1] + self.fRec6[2]);
            self.fRec6[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow15)) & 8191) as usize];
            self.fVec6[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec7[1] + self.fRec7[2]);
            self.fRec7[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow17)) & 8191) as usize];
            self.fVec7[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec8[1] + self.fRec8[2]);
            self.fRec8[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 8191) as usize];
            self.fVec8[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec9[1] + self.fRec9[2]);
            self.fRec9[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 8191) as usize];
            self.fVec9[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec10[1] + self.fRec10[2]);
            self.fRec10[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 8191) as usize];
            self.fVec10[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec11[1] + self.fRec11[2]);
            self.fRec11[0] = self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 8191) as usize];
            self.fVec11[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec12[1] + self.fRec12[2]);
            self.fRec12[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 8191) as usize];
            self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec13[1] + self.fRec13[2]);
            self.fRec13[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 8191) as usize];
            self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp0 + fSlow0 * (self.fRec14[1] + self.fRec14[2]);
            self.fRec14[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 4095) as usize];
            self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp0 + fSlow0 * (self.fRec15[1] + self.fRec15[2]);
            self.fRec15[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize];
            self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp0 + fSlow0 * (self.fRec16[1] + self.fRec16[2]);
            self.fRec16[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 2047) as usize];
            self.fVec16[(self.IOTA0 & 511) as usize] = fSlow0 * (self.fRec17[1] + self.fRec17[2]) + fTemp0;
            self.fRec17[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 511) as usize];
            *output0 = fSlow39
                * (fSlow38 * self.fRec17[0]
                    + fSlow36 * self.fRec16[0]
                    + fSlow34 * self.fRec15[0]
                    + fSlow32 * self.fRec14[0]
                    + fSlow30 * self.fRec13[0]
                    + fSlow28 * self.fRec12[0]
                    + fSlow26 * self.fRec11[0]
                    + fSlow24 * self.fRec10[0]
                    + fSlow22 * self.fRec9[0]
                    + fSlow20 * self.fRec8[0]
                    + fSlow18 * self.fRec7[0]
                    + fSlow16 * self.fRec6[0]
                    + fSlow14 * self.fRec5[0]
                    + fSlow12 * self.fRec4[0]
                    + fSlow10 * self.fRec3[0]
                    + fSlow8 * self.fRec0[0]);
            self.fVec17[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec18[1] + self.fRec18[2]);
            self.fRec18[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 8191) as usize];
            self.fVec18[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec19[1] + self.fRec19[2]);
            self.fRec19[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 8191) as usize];
            self.fVec19[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec20[1] + self.fRec20[2]);
            self.fRec20[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 8191) as usize];
            self.fVec20[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec21[1] + self.fRec21[2]);
            self.fRec21[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize];
            self.fVec21[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec22[1] + self.fRec22[2]);
            self.fRec22[0] = self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 8191) as usize];
            self.fVec22[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec23[1] + self.fRec23[2]);
            self.fRec23[0] = self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 8191) as usize];
            self.fVec23[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec24[1] + self.fRec24[2]);
            self.fRec24[0] = self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 8191) as usize];
            self.fVec24[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec25[1] + self.fRec25[2]);
            self.fRec25[0] = self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 8191) as usize];
            self.fVec25[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec26[1] + self.fRec26[2]);
            self.fRec26[0] = self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 8191) as usize];
            self.fVec26[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec27[1] + self.fRec27[2]);
            self.fRec27[0] = self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 8191) as usize];
            self.fVec27[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec28[1] + self.fRec28[2]);
            self.fRec28[0] = self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 8191) as usize];
            self.fVec28[(self.IOTA0 & 8191) as usize] = fTemp0 + fSlow0 * (self.fRec29[1] + self.fRec29[2]);
            self.fRec29[0] = self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 8191) as usize];
            self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp0 + fSlow0 * (self.fRec30[1] + self.fRec30[2]);
            self.fRec30[0] = self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 4095) as usize];
            self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp0 + fSlow0 * (self.fRec31[1] + self.fRec31[2]);
            self.fRec31[0] = self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 4095) as usize];
            self.fVec31[(self.IOTA0 & 2047) as usize] = fTemp0 + fSlow0 * (self.fRec32[1] + self.fRec32[2]);
            self.fRec32[0] = self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 2047) as usize];
            self.fVec32[(self.IOTA0 & 1023) as usize] = fTemp0 + fSlow0 * (self.fRec33[1] + self.fRec33[2]);
            self.fRec33[0] = self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 1023) as usize];
            *output1 = fSlow39
                * (fSlow71 * self.fRec33[0]
                    + fSlow69 * self.fRec32[0]
                    + fSlow67 * self.fRec31[0]
                    + fSlow65 * self.fRec30[0]
                    + fSlow63 * self.fRec29[0]
                    + fSlow61 * self.fRec28[0]
                    + fSlow59 * self.fRec27[0]
                    + fSlow57 * self.fRec26[0]
                    + fSlow55 * self.fRec25[0]
                    + fSlow53 * self.fRec24[0]
                    + fSlow51 * self.fRec23[0]
                    + fSlow49 * self.fRec22[0]
                    + fSlow47 * self.fRec21[0]
                    + fSlow45 * self.fRec20[0]
                    + fSlow43 * self.fRec19[0]
                    + fSlow41 * self.fRec18[0]);
            self.iRec1[1] = self.iRec1[0];
            self.fVec0[1] = self.fVec0[0];
            self.fRec2[1] = self.fRec2[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.fRec0[2] = self.fRec0[1];
            self.fRec0[1] = self.fRec0[0];
            self.fRec3[2] = self.fRec3[1];
            self.fRec3[1] = self.fRec3[0];
            self.fRec4[2] = self.fRec4[1];
            self.fRec4[1] = self.fRec4[0];
            self.fRec5[2] = self.fRec5[1];
            self.fRec5[1] = self.fRec5[0];
            self.fRec6[2] = self.fRec6[1];
            self.fRec6[1] = self.fRec6[0];
            self.fRec7[2] = self.fRec7[1];
            self.fRec7[1] = self.fRec7[0];
            self.fRec8[2] = self.fRec8[1];
            self.fRec8[1] = self.fRec8[0];
            self.fRec9[2] = self.fRec9[1];
            self.fRec9[1] = self.fRec9[0];
            self.fRec10[2] = self.fRec10[1];
            self.fRec10[1] = self.fRec10[0];
            self.fRec11[2] = self.fRec11[1];
            self.fRec11[1] = self.fRec11[0];
            self.fRec12[2] = self.fRec12[1];
            self.fRec12[1] = self.fRec12[0];
            self.fRec13[2] = self.fRec13[1];
            self.fRec13[1] = self.fRec13[0];
            self.fRec14[2] = self.fRec14[1];
            self.fRec14[1] = self.fRec14[0];
            self.fRec15[2] = self.fRec15[1];
            self.fRec15[1] = self.fRec15[0];
            self.fRec16[2] = self.fRec16[1];
            self.fRec16[1] = self.fRec16[0];
            self.fRec17[2] = self.fRec17[1];
            self.fRec17[1] = self.fRec17[0];
            self.fRec18[2] = self.fRec18[1];
            self.fRec18[1] = self.fRec18[0];
            self.fRec19[2] = self.fRec19[1];
            self.fRec19[1] = self.fRec19[0];
            self.fRec20[2] = self.fRec20[1];
            self.fRec20[1] = self.fRec20[0];
            self.fRec21[2] = self.fRec21[1];
            self.fRec21[1] = self.fRec21[0];
            self.fRec22[2] = self.fRec22[1];
            self.fRec22[1] = self.fRec22[0];
            self.fRec23[2] = self.fRec23[1];
            self.fRec23[1] = self.fRec23[0];
            self.fRec24[2] = self.fRec24[1];
            self.fRec24[1] = self.fRec24[0];
            self.fRec25[2] = self.fRec25[1];
            self.fRec25[1] = self.fRec25[0];
            self.fRec26[2] = self.fRec26[1];
            self.fRec26[1] = self.fRec26[0];
            self.fRec27[2] = self.fRec27[1];
            self.fRec27[1] = self.fRec27[0];
            self.fRec28[2] = self.fRec28[1];
            self.fRec28[1] = self.fRec28[0];
            self.fRec29[2] = self.fRec29[1];
            self.fRec29[1] = self.fRec29[0];
            self.fRec30[2] = self.fRec30[1];
            self.fRec30[1] = self.fRec30[0];
            self.fRec31[2] = self.fRec31[1];
            self.fRec31[1] = self.fRec31[0];
            self.fRec32[2] = self.fRec32[1];
            self.fRec32[1] = self.fRec32[0];
            self.fRec33[2] = self.fRec33[1];
            self.fRec33[1] = self.fRec33[0];
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
