/* ------------------------------------------------------------
name: "reverb"
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

fn Dsp_faustpower2_f(value: F32) -> F32 {
    return value * value;
}
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
    fSampleRate: i32,
    fConst2: F32,
    fConst3: F32,
    fConst5: F32,
    fConst6: F32,
    fConst16: F32,
    fConst18: F32,
    fConst19: F32,
    fRec13: [F32; 2],
    fConst20: F32,
    fConst21: F32,
    fRec12: [F32; 2],
    IOTA0: i32,
    fVec0: [F32; 16384],
    iConst23: i32,
    fVec1: [F32; 16384],
    iConst24: i32,
    fVec2: [F32; 4096],
    iConst25: i32,
    fRec10: [F32; 2],
    fConst34: F32,
    fRec17: [F32; 2],
    fConst35: F32,
    fConst36: F32,
    fRec16: [F32; 2],
    fVec3: [F32; 16384],
    iConst38: i32,
    fVec4: [F32; 2048],
    iConst39: i32,
    fRec14: [F32; 2],
    fConst48: F32,
    fRec21: [F32; 2],
    fConst49: F32,
    fConst50: F32,
    fRec20: [F32; 2],
    fVec5: [F32; 16384],
    iConst52: i32,
    fVec6: [F32; 4096],
    iConst53: i32,
    fRec18: [F32; 2],
    fConst62: F32,
    fRec25: [F32; 2],
    fConst63: F32,
    fConst64: F32,
    fRec24: [F32; 2],
    fVec7: [F32; 16384],
    iConst66: i32,
    fVec8: [F32; 2048],
    iConst67: i32,
    fRec22: [F32; 2],
    fConst76: F32,
    fRec29: [F32; 2],
    fConst77: F32,
    fConst78: F32,
    fRec28: [F32; 2],
    fVec9: [F32; 32768],
    iConst80: i32,
    fVec10: [F32; 16384],
    fVec11: [F32; 4096],
    iConst81: i32,
    fRec26: [F32; 2],
    fConst90: F32,
    fRec33: [F32; 2],
    fConst91: F32,
    fConst92: F32,
    fRec32: [F32; 2],
    fVec12: [F32; 16384],
    iConst94: i32,
    fVec13: [F32; 4096],
    iConst95: i32,
    fRec30: [F32; 2],
    fConst104: F32,
    fRec37: [F32; 2],
    fConst105: F32,
    fConst106: F32,
    fRec36: [F32; 2],
    fVec14: [F32; 32768],
    iConst108: i32,
    fVec15: [F32; 4096],
    iConst109: i32,
    fRec34: [F32; 2],
    fConst118: F32,
    fRec41: [F32; 2],
    fConst119: F32,
    fConst120: F32,
    fRec40: [F32; 2],
    fVec16: [F32; 32768],
    iConst122: i32,
    fVec17: [F32; 2048],
    iConst123: i32,
    fRec38: [F32; 2],
    fRec2: [F32; 3],
    fRec3: [F32; 3],
    fRec4: [F32; 3],
    fRec5: [F32; 3],
    fRec6: [F32; 3],
    fRec7: [F32; 3],
    fRec8: [F32; 3],
    fRec9: [F32; 3],
    fRec1: [F32; 3],
    fRec0: [F32; 3],
    fConst124: F32,
    fConst125: F32,
    fConst126: F32,
    fConst127: F32,
    fVslider0: F32,
    fRec42: [F32; 2],
    fVslider1: F32,
    fRec43: [F32; 2],
    fRec45: [F32; 3],
    fRec44: [F32; 3],
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp {
            fSampleRate: 0,
            fConst2: 0.0,
            fConst3: 0.0,
            fConst5: 0.0,
            fConst6: 0.0,
            fConst16: 0.0,
            fConst18: 0.0,
            fConst19: 0.0,
            fRec13: [0.0; 2],
            fConst20: 0.0,
            fConst21: 0.0,
            fRec12: [0.0; 2],
            IOTA0: 0,
            fVec0: [0.0; 16384],
            iConst23: 0,
            fVec1: [0.0; 16384],
            iConst24: 0,
            fVec2: [0.0; 4096],
            iConst25: 0,
            fRec10: [0.0; 2],
            fConst34: 0.0,
            fRec17: [0.0; 2],
            fConst35: 0.0,
            fConst36: 0.0,
            fRec16: [0.0; 2],
            fVec3: [0.0; 16384],
            iConst38: 0,
            fVec4: [0.0; 2048],
            iConst39: 0,
            fRec14: [0.0; 2],
            fConst48: 0.0,
            fRec21: [0.0; 2],
            fConst49: 0.0,
            fConst50: 0.0,
            fRec20: [0.0; 2],
            fVec5: [0.0; 16384],
            iConst52: 0,
            fVec6: [0.0; 4096],
            iConst53: 0,
            fRec18: [0.0; 2],
            fConst62: 0.0,
            fRec25: [0.0; 2],
            fConst63: 0.0,
            fConst64: 0.0,
            fRec24: [0.0; 2],
            fVec7: [0.0; 16384],
            iConst66: 0,
            fVec8: [0.0; 2048],
            iConst67: 0,
            fRec22: [0.0; 2],
            fConst76: 0.0,
            fRec29: [0.0; 2],
            fConst77: 0.0,
            fConst78: 0.0,
            fRec28: [0.0; 2],
            fVec9: [0.0; 32768],
            iConst80: 0,
            fVec10: [0.0; 16384],
            fVec11: [0.0; 4096],
            iConst81: 0,
            fRec26: [0.0; 2],
            fConst90: 0.0,
            fRec33: [0.0; 2],
            fConst91: 0.0,
            fConst92: 0.0,
            fRec32: [0.0; 2],
            fVec12: [0.0; 16384],
            iConst94: 0,
            fVec13: [0.0; 4096],
            iConst95: 0,
            fRec30: [0.0; 2],
            fConst104: 0.0,
            fRec37: [0.0; 2],
            fConst105: 0.0,
            fConst106: 0.0,
            fRec36: [0.0; 2],
            fVec14: [0.0; 32768],
            iConst108: 0,
            fVec15: [0.0; 4096],
            iConst109: 0,
            fRec34: [0.0; 2],
            fConst118: 0.0,
            fRec41: [0.0; 2],
            fConst119: 0.0,
            fConst120: 0.0,
            fRec40: [0.0; 2],
            fVec16: [0.0; 32768],
            iConst122: 0,
            fVec17: [0.0; 2048],
            iConst123: 0,
            fRec38: [0.0; 2],
            fRec2: [0.0; 3],
            fRec3: [0.0; 3],
            fRec4: [0.0; 3],
            fRec5: [0.0; 3],
            fRec6: [0.0; 3],
            fRec7: [0.0; 3],
            fRec8: [0.0; 3],
            fRec9: [0.0; 3],
            fRec1: [0.0; 3],
            fRec0: [0.0; 3],
            fConst124: 0.0,
            fConst125: 0.0,
            fConst126: 0.0,
            fConst127: 0.0,
            fVslider0: 0.0,
            fRec42: [0.0; 2],
            fVslider1: 0.0,
            fRec43: [0.0; 2],
            fRec45: [0.0; 3],
            fRec44: [0.0; 3],
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("basics.lib/name", r"Faust Basic Element Library");
        m.declare(
            "basics.lib/tabulateNd",
            r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>",
        );
        m.declare("basics.lib/version", r"1.15.0");
        m.declare(
            "compile_options",
            r"-a ./architecture/benchmark.rs -lang rust -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare("delays.lib/name", r"Faust Delay Library");
        m.declare("delays.lib/version", r"1.1.0");
        m.declare("demos.lib/name", r"Faust Demos Library");
        m.declare("demos.lib/version", r"1.1.1");
        m.declare("demos.lib/zita_light:author", r"Julius O. Smith III");
        m.declare("demos.lib/zita_light:licence", r"MIT");
        m.declare("filename", r"reverb.dsp");
        m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/allpass_comb:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/fir:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/fir:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/iir:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/iir:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
        m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/lowpass:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/name", r"Faust Filters Library");
        m.declare("filters.lib/peak_eq_rm:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/peak_eq_rm:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/peak_eq_rm:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/tf1:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/tf1s:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
        m.declare(
            "filters.lib/tf2:copyright",
            r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
        m.declare("filters.lib/version", r"1.3.0");
        m.declare("maths.lib/author", r"GRAME");
        m.declare("maths.lib/copyright", r"GRAME");
        m.declare("maths.lib/license", r"LGPL with exception");
        m.declare("maths.lib/name", r"Faust Math Library");
        m.declare("maths.lib/version", r"2.8.0");
        m.declare("name", r"reverb");
        m.declare("platform.lib/name", r"Generic Platform Library");
        m.declare("platform.lib/version", r"1.3.0");
        m.declare("reverbs.lib/name", r"Faust Reverb Library");
        m.declare("reverbs.lib/version", r"1.2.1");
        m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
        m.declare("routes.lib/name", r"Faust Signal Routing Library");
        m.declare("routes.lib/version", r"1.2.0");
        m.declare("signals.lib/name", r"Faust Signal Routing Library");
        m.declare("signals.lib/version", r"1.5.0");
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
    fn instance_reset_params(&mut self) {
        self.fVslider0 = 0.0;
        self.fVslider1 = -6.0;
    }
    fn instance_clear(&mut self) {
        for l0 in 0..2 {
            self.fRec13[l0 as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec12[l1 as usize] = 0.0;
        }
        self.IOTA0 = 0;
        for l2 in 0..16384 {
            self.fVec0[l2 as usize] = 0.0;
        }
        for l3 in 0..16384 {
            self.fVec1[l3 as usize] = 0.0;
        }
        for l4 in 0..4096 {
            self.fVec2[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fRec10[l5 as usize] = 0.0;
        }
        for l6 in 0..2 {
            self.fRec17[l6 as usize] = 0.0;
        }
        for l7 in 0..2 {
            self.fRec16[l7 as usize] = 0.0;
        }
        for l8 in 0..16384 {
            self.fVec3[l8 as usize] = 0.0;
        }
        for l9 in 0..2048 {
            self.fVec4[l9 as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec14[l10 as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fRec21[l11 as usize] = 0.0;
        }
        for l12 in 0..2 {
            self.fRec20[l12 as usize] = 0.0;
        }
        for l13 in 0..16384 {
            self.fVec5[l13 as usize] = 0.0;
        }
        for l14 in 0..4096 {
            self.fVec6[l14 as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec18[l15 as usize] = 0.0;
        }
        for l16 in 0..2 {
            self.fRec25[l16 as usize] = 0.0;
        }
        for l17 in 0..2 {
            self.fRec24[l17 as usize] = 0.0;
        }
        for l18 in 0..16384 {
            self.fVec7[l18 as usize] = 0.0;
        }
        for l19 in 0..2048 {
            self.fVec8[l19 as usize] = 0.0;
        }
        for l20 in 0..2 {
            self.fRec22[l20 as usize] = 0.0;
        }
        for l21 in 0..2 {
            self.fRec29[l21 as usize] = 0.0;
        }
        for l22 in 0..2 {
            self.fRec28[l22 as usize] = 0.0;
        }
        for l23 in 0..32768 {
            self.fVec9[l23 as usize] = 0.0;
        }
        for l24 in 0..16384 {
            self.fVec10[l24 as usize] = 0.0;
        }
        for l25 in 0..4096 {
            self.fVec11[l25 as usize] = 0.0;
        }
        for l26 in 0..2 {
            self.fRec26[l26 as usize] = 0.0;
        }
        for l27 in 0..2 {
            self.fRec33[l27 as usize] = 0.0;
        }
        for l28 in 0..2 {
            self.fRec32[l28 as usize] = 0.0;
        }
        for l29 in 0..16384 {
            self.fVec12[l29 as usize] = 0.0;
        }
        for l30 in 0..4096 {
            self.fVec13[l30 as usize] = 0.0;
        }
        for l31 in 0..2 {
            self.fRec30[l31 as usize] = 0.0;
        }
        for l32 in 0..2 {
            self.fRec37[l32 as usize] = 0.0;
        }
        for l33 in 0..2 {
            self.fRec36[l33 as usize] = 0.0;
        }
        for l34 in 0..32768 {
            self.fVec14[l34 as usize] = 0.0;
        }
        for l35 in 0..4096 {
            self.fVec15[l35 as usize] = 0.0;
        }
        for l36 in 0..2 {
            self.fRec34[l36 as usize] = 0.0;
        }
        for l37 in 0..2 {
            self.fRec41[l37 as usize] = 0.0;
        }
        for l38 in 0..2 {
            self.fRec40[l38 as usize] = 0.0;
        }
        for l39 in 0..32768 {
            self.fVec16[l39 as usize] = 0.0;
        }
        for l40 in 0..2048 {
            self.fVec17[l40 as usize] = 0.0;
        }
        for l41 in 0..2 {
            self.fRec38[l41 as usize] = 0.0;
        }
        for l42 in 0..3 {
            self.fRec2[l42 as usize] = 0.0;
        }
        for l43 in 0..3 {
            self.fRec3[l43 as usize] = 0.0;
        }
        for l44 in 0..3 {
            self.fRec4[l44 as usize] = 0.0;
        }
        for l45 in 0..3 {
            self.fRec5[l45 as usize] = 0.0;
        }
        for l46 in 0..3 {
            self.fRec6[l46 as usize] = 0.0;
        }
        for l47 in 0..3 {
            self.fRec7[l47 as usize] = 0.0;
        }
        for l48 in 0..3 {
            self.fRec8[l48 as usize] = 0.0;
        }
        for l49 in 0..3 {
            self.fRec9[l49 as usize] = 0.0;
        }
        for l50 in 0..3 {
            self.fRec1[l50 as usize] = 0.0;
        }
        for l51 in 0..3 {
            self.fRec0[l51 as usize] = 0.0;
        }
        for l52 in 0..2 {
            self.fRec42[l52 as usize] = 0.0;
        }
        for l53 in 0..2 {
            self.fRec43[l53 as usize] = 0.0;
        }
        for l54 in 0..3 {
            self.fRec45[l54 as usize] = 0.0;
        }
        for l55 in 0..3 {
            self.fRec44[l55 as usize] = 0.0;
        }
    }
    fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
        let mut fConst1: F32 = 9424.778 / fConst0;
        self.fConst2 = (1.0 - fConst1) / (fConst1 + 1.0);
        self.fConst3 = F32::cos(fConst1) * (self.fConst2 + 1.0);
        let mut fConst4: F32 = 1979.2034 / fConst0;
        self.fConst5 = (1.0 - fConst4) / (fConst4 + 1.0);
        self.fConst6 = F32::cos(fConst4) * (self.fConst5 + 1.0);
        let mut fConst7: F32 = F32::floor(0.174713 * fConst0 + 0.5);
        let mut fConst8: F32 = fConst7 / fConst0;
        let mut fConst9: F32 = F32::exp(-(3.4538777 * fConst8));
        let mut fConst10: F32 = Dsp_faustpower2_f(fConst9);
        let mut fConst11: F32 = 1.0 - fConst10;
        let mut fConst12: F32 = F32::cos(37699.113 / fConst0);
        let mut fConst13: F32 = 1.0 - fConst12 * fConst10;
        let mut fConst14: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst13) / Dsp_faustpower2_f(fConst11) + -1.0,
        ));
        let mut fConst15: F32 = fConst13 / fConst11;
        self.fConst16 = fConst15 - fConst14;
        let mut fConst17: F32 = 1.0 / F32::tan(628.31854 / fConst0);
        self.fConst18 = 1.0 - fConst17;
        self.fConst19 = 1.0 / (fConst17 + 1.0);
        self.fConst20 = F32::exp(-(2.3025851 * fConst8)) / fConst9 + -1.0;
        self.fConst21 = fConst9 * (fConst14 + (1.0 - fConst15));
        let mut fConst22: F32 = F32::floor(0.022904 * fConst0 + 0.5);
        self.iConst23 = (F32::min(8192.0, F32::max(0.0, fConst7 - fConst22))) as i32;
        self.iConst24 = (F32::min(8192.0, F32::max(0.0, 0.06 * fConst0))) as i32;
        self.iConst25 = (F32::min(2048.0, F32::max(0.0, fConst22 + -1.0))) as i32;
        let mut fConst26: F32 = F32::floor(0.153129 * fConst0 + 0.5);
        let mut fConst27: F32 = fConst26 / fConst0;
        let mut fConst28: F32 = F32::exp(-(3.4538777 * fConst27));
        let mut fConst29: F32 = Dsp_faustpower2_f(fConst28);
        let mut fConst30: F32 = 1.0 - fConst29;
        let mut fConst31: F32 = 1.0 - fConst12 * fConst29;
        let mut fConst32: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst31) / Dsp_faustpower2_f(fConst30) + -1.0,
        ));
        let mut fConst33: F32 = fConst31 / fConst30;
        self.fConst34 = fConst33 - fConst32;
        self.fConst35 = F32::exp(-(2.3025851 * fConst27)) / fConst28 + -1.0;
        self.fConst36 = fConst28 * (fConst32 + (1.0 - fConst33));
        let mut fConst37: F32 = F32::floor(0.020346 * fConst0 + 0.5);
        self.iConst38 = (F32::min(8192.0, F32::max(0.0, fConst26 - fConst37))) as i32;
        self.iConst39 = (F32::min(1024.0, F32::max(0.0, fConst37 + -1.0))) as i32;
        let mut fConst40: F32 = F32::floor(0.127837 * fConst0 + 0.5);
        let mut fConst41: F32 = fConst40 / fConst0;
        let mut fConst42: F32 = F32::exp(-(3.4538777 * fConst41));
        let mut fConst43: F32 = Dsp_faustpower2_f(fConst42);
        let mut fConst44: F32 = 1.0 - fConst43;
        let mut fConst45: F32 = 1.0 - fConst12 * fConst43;
        let mut fConst46: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst45) / Dsp_faustpower2_f(fConst44) + -1.0,
        ));
        let mut fConst47: F32 = fConst45 / fConst44;
        self.fConst48 = fConst47 - fConst46;
        self.fConst49 = F32::exp(-(2.3025851 * fConst41)) / fConst42 + -1.0;
        self.fConst50 = fConst42 * (fConst46 + (1.0 - fConst47));
        let mut fConst51: F32 = F32::floor(0.031604 * fConst0 + 0.5);
        self.iConst52 = (F32::min(8192.0, F32::max(0.0, fConst40 - fConst51))) as i32;
        self.iConst53 = (F32::min(2048.0, F32::max(0.0, fConst51 + -1.0))) as i32;
        let mut fConst54: F32 = F32::floor(0.125 * fConst0 + 0.5);
        let mut fConst55: F32 = fConst54 / fConst0;
        let mut fConst56: F32 = F32::exp(-(3.4538777 * fConst55));
        let mut fConst57: F32 = Dsp_faustpower2_f(fConst56);
        let mut fConst58: F32 = 1.0 - fConst57;
        let mut fConst59: F32 = 1.0 - fConst12 * fConst57;
        let mut fConst60: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst59) / Dsp_faustpower2_f(fConst58) + -1.0,
        ));
        let mut fConst61: F32 = fConst59 / fConst58;
        self.fConst62 = fConst61 - fConst60;
        self.fConst63 = F32::exp(-(2.3025851 * fConst55)) / fConst56 + -1.0;
        self.fConst64 = fConst56 * (fConst60 + (1.0 - fConst61));
        let mut fConst65: F32 = F32::floor(0.013458 * fConst0 + 0.5);
        self.iConst66 = (F32::min(8192.0, F32::max(0.0, fConst54 - fConst65))) as i32;
        self.iConst67 = (F32::min(1024.0, F32::max(0.0, fConst65 + -1.0))) as i32;
        let mut fConst68: F32 = F32::floor(0.210389 * fConst0 + 0.5);
        let mut fConst69: F32 = fConst68 / fConst0;
        let mut fConst70: F32 = F32::exp(-(3.4538777 * fConst69));
        let mut fConst71: F32 = Dsp_faustpower2_f(fConst70);
        let mut fConst72: F32 = 1.0 - fConst71;
        let mut fConst73: F32 = 1.0 - fConst12 * fConst71;
        let mut fConst74: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst73) / Dsp_faustpower2_f(fConst72) + -1.0,
        ));
        let mut fConst75: F32 = fConst73 / fConst72;
        self.fConst76 = fConst75 - fConst74;
        self.fConst77 = F32::exp(-(2.3025851 * fConst69)) / fConst70 + -1.0;
        self.fConst78 = fConst70 * (fConst74 + (1.0 - fConst75));
        let mut fConst79: F32 = F32::floor(0.024421 * fConst0 + 0.5);
        self.iConst80 = (F32::min(16384.0, F32::max(0.0, fConst68 - fConst79))) as i32;
        self.iConst81 = (F32::min(2048.0, F32::max(0.0, fConst79 + -1.0))) as i32;
        let mut fConst82: F32 = F32::floor(0.192303 * fConst0 + 0.5);
        let mut fConst83: F32 = fConst82 / fConst0;
        let mut fConst84: F32 = F32::exp(-(3.4538777 * fConst83));
        let mut fConst85: F32 = Dsp_faustpower2_f(fConst84);
        let mut fConst86: F32 = 1.0 - fConst85;
        let mut fConst87: F32 = 1.0 - fConst12 * fConst85;
        let mut fConst88: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst87) / Dsp_faustpower2_f(fConst86) + -1.0,
        ));
        let mut fConst89: F32 = fConst87 / fConst86;
        self.fConst90 = fConst89 - fConst88;
        self.fConst91 = F32::exp(-(2.3025851 * fConst83)) / fConst84 + -1.0;
        self.fConst92 = fConst84 * (fConst88 + (1.0 - fConst89));
        let mut fConst93: F32 = F32::floor(0.029291 * fConst0 + 0.5);
        self.iConst94 = (F32::min(8192.0, F32::max(0.0, fConst82 - fConst93))) as i32;
        self.iConst95 = (F32::min(2048.0, F32::max(0.0, fConst93 + -1.0))) as i32;
        let mut fConst96: F32 = F32::floor(0.256891 * fConst0 + 0.5);
        let mut fConst97: F32 = fConst96 / fConst0;
        let mut fConst98: F32 = F32::exp(-(3.4538777 * fConst97));
        let mut fConst99: F32 = Dsp_faustpower2_f(fConst98);
        let mut fConst100: F32 = 1.0 - fConst99;
        let mut fConst101: F32 = 1.0 - fConst12 * fConst99;
        let mut fConst102: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst101) / Dsp_faustpower2_f(fConst100) + -1.0,
        ));
        let mut fConst103: F32 = fConst101 / fConst100;
        self.fConst104 = fConst103 - fConst102;
        self.fConst105 = F32::exp(-(2.3025851 * fConst97)) / fConst98 + -1.0;
        self.fConst106 = fConst98 * (fConst102 + (1.0 - fConst103));
        let mut fConst107: F32 = F32::floor(0.027333 * fConst0 + 0.5);
        self.iConst108 = (F32::min(16384.0, F32::max(0.0, fConst96 - fConst107))) as i32;
        self.iConst109 = (F32::min(2048.0, F32::max(0.0, fConst107 + -1.0))) as i32;
        let mut fConst110: F32 = F32::floor(0.219991 * fConst0 + 0.5);
        let mut fConst111: F32 = fConst110 / fConst0;
        let mut fConst112: F32 = F32::exp(-(3.4538777 * fConst111));
        let mut fConst113: F32 = Dsp_faustpower2_f(fConst112);
        let mut fConst114: F32 = 1.0 - fConst113;
        let mut fConst115: F32 = 1.0 - fConst12 * fConst113;
        let mut fConst116: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst115) / Dsp_faustpower2_f(fConst114) + -1.0,
        ));
        let mut fConst117: F32 = fConst115 / fConst114;
        self.fConst118 = fConst117 - fConst116;
        self.fConst119 = F32::exp(-(2.3025851 * fConst111)) / fConst112 + -1.0;
        self.fConst120 = fConst112 * (fConst116 + (1.0 - fConst117));
        let mut fConst121: F32 = F32::floor(0.019123 * fConst0 + 0.5);
        self.iConst122 = (F32::min(16384.0, F32::max(0.0, fConst110 - fConst121))) as i32;
        self.iConst123 = (F32::min(1024.0, F32::max(0.0, fConst121 + -1.0))) as i32;
        self.fConst124 = 2.0 * self.fConst3;
        self.fConst125 = 2.0 * self.fConst2;
        self.fConst126 = 44.1 / fConst0;
        self.fConst127 = 1.0 - self.fConst126;
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
        ui_interface.open_horizontal_box("Zita Light");
        ui_interface.declare(Some(ParamIndex(0)), "1", "");
        ui_interface.declare(Some(ParamIndex(0)), "style", "knob");
        ui_interface.declare(Some(ParamIndex(0)), "tooltip", "-1 = dry, 1 = wet");
        ui_interface.add_vertical_slider("Dry/Wet Mix", ParamIndex(0), 0.0, -1.0, 1.0, 0.01);
        ui_interface.declare(Some(ParamIndex(1)), "2", "");
        ui_interface.declare(Some(ParamIndex(1)), "style", "knob");
        ui_interface.declare(Some(ParamIndex(1)), "tooltip", "Output scale         factor");
        ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
        ui_interface.add_vertical_slider("Level", ParamIndex(1), -6.0, -7e+01, 4e+01, 0.1);
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        match param.0 {
            0 => Some(self.fVslider0),
            1 => Some(self.fVslider1),
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        match param.0 {
            0 => self.fVslider0 = value,
            1 => self.fVslider1 = value,
            _ => {}
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
            let inputs0 = inputs0[..count as usize].iter();
            let inputs1 = inputs1[..count as usize].iter();
            (inputs0, inputs1)
        } else {
            panic!("wrong number of inputs");
        };
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            let outputs1 = outputs1[..count as usize].iter_mut();
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let mut fSlow0: F32 = self.fConst126 * self.fVslider0;
        let mut fSlow1: F32 = self.fConst126 * F32::powf(1e+01, 0.05 * self.fVslider1);
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            let mut fTemp0: F32 = self.fConst6 * self.fRec1[1];
            self.fRec13[0] = -(self.fConst19 * (self.fConst18 * self.fRec13[1] - (self.fRec6[1] + self.fRec6[2])));
            self.fRec12[0] =
                self.fConst21 * (self.fRec6[1] + self.fConst20 * self.fRec13[0]) + self.fConst16 * self.fRec12[1];
            self.fVec0[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec12[0] + 1e-20;
            let mut fTemp1: F32 = *input0;
            self.fVec1[(self.IOTA0 & 16383) as usize] = fTemp1;
            let mut fTemp2: F32 = 0.3 * self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize];
            let mut fTemp3: F32 = fTemp2
                + self.fVec0[((i32::wrapping_sub(self.IOTA0, self.iConst23)) & 16383) as usize]
                - 0.6 * self.fRec10[1];
            self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp3;
            self.fRec10[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst25)) & 4095) as usize];
            let mut fRec11: F32 = 0.6 * fTemp3;
            self.fRec17[0] = -(self.fConst19 * (self.fConst18 * self.fRec17[1] - (self.fRec2[1] + self.fRec2[2])));
            self.fRec16[0] =
                self.fConst36 * (self.fRec2[1] + self.fConst35 * self.fRec17[0]) + self.fConst34 * self.fRec16[1];
            self.fVec3[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec16[0] + 1e-20;
            let mut fTemp4: F32 = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst38)) & 16383) as usize]
                + fTemp2
                - 0.6 * self.fRec14[1];
            self.fVec4[(self.IOTA0 & 2047) as usize] = fTemp4;
            self.fRec14[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst39)) & 2047) as usize];
            let mut fRec15: F32 = 0.6 * fTemp4;
            let mut fTemp5: F32 = fRec15 + fRec11;
            self.fRec21[0] = -(self.fConst19 * (self.fConst18 * self.fRec21[1] - (self.fRec4[1] + self.fRec4[2])));
            self.fRec20[0] =
                self.fConst50 * (self.fRec4[1] + self.fConst49 * self.fRec21[0]) + self.fConst48 * self.fRec20[1];
            self.fVec5[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec20[0] + 1e-20;
            let mut fTemp6: F32 = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst52)) & 16383) as usize]
                - (fTemp2 + 0.6 * self.fRec18[1]);
            self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp6;
            self.fRec18[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst53)) & 4095) as usize];
            let mut fRec19: F32 = 0.6 * fTemp6;
            self.fRec25[0] = -(self.fConst19 * (self.fConst18 * self.fRec25[1] - (self.fRec8[1] + self.fRec8[2])));
            self.fRec24[0] =
                self.fConst64 * (self.fRec8[1] + self.fConst63 * self.fRec25[0]) + self.fConst62 * self.fRec24[1];
            self.fVec7[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec24[0] + 1e-20;
            let mut fTemp7: F32 = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst66)) & 16383) as usize]
                - (fTemp2 + 0.6 * self.fRec22[1]);
            self.fVec8[(self.IOTA0 & 2047) as usize] = fTemp7;
            self.fRec22[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst67)) & 2047) as usize];
            let mut fRec23: F32 = 0.6 * fTemp7;
            let mut fTemp8: F32 = fRec23 + fRec19 + fTemp5;
            self.fRec29[0] = -(self.fConst19 * (self.fConst18 * self.fRec29[1] - (self.fRec3[1] + self.fRec3[2])));
            self.fRec28[0] =
                self.fConst78 * (self.fRec3[1] + self.fConst77 * self.fRec29[0]) + self.fConst76 * self.fRec28[1];
            self.fVec9[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec28[0] + 1e-20;
            let mut fTemp9: F32 = *input1;
            self.fVec10[(self.IOTA0 & 16383) as usize] = fTemp9;
            let mut fTemp10: F32 = 0.3 * self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize];
            let mut fTemp11: F32 = fTemp10
                + 0.6 * self.fRec26[1]
                + self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst80)) & 32767) as usize];
            self.fVec11[(self.IOTA0 & 4095) as usize] = fTemp11;
            self.fRec26[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst81)) & 4095) as usize];
            let mut fRec27: F32 = -(0.6 * fTemp11);
            self.fRec33[0] = -(self.fConst19 * (self.fConst18 * self.fRec33[1] - (self.fRec7[1] + self.fRec7[2])));
            self.fRec32[0] =
                self.fConst92 * (self.fRec7[1] + self.fConst91 * self.fRec33[0]) + self.fConst90 * self.fRec32[1];
            self.fVec12[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec32[0] + 1e-20;
            let mut fTemp12: F32 = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst94)) & 16383) as usize]
                + fTemp10
                + 0.6 * self.fRec30[1];
            self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp12;
            self.fRec30[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst95)) & 4095) as usize];
            let mut fRec31: F32 = -(0.6 * fTemp12);
            self.fRec37[0] = -(self.fConst19 * (self.fConst18 * self.fRec37[1] - (self.fRec5[1] + self.fRec5[2])));
            self.fRec36[0] =
                self.fConst106 * (self.fRec5[1] + self.fConst105 * self.fRec37[0]) + self.fConst104 * self.fRec36[1];
            self.fVec14[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec36[0] + 1e-20;
            let mut fTemp13: F32 =
                0.6 * self.fRec34[1] + self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst108)) & 32767) as usize];
            self.fVec15[(self.IOTA0 & 4095) as usize] = fTemp13 - fTemp10;
            self.fRec34[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst109)) & 4095) as usize];
            let mut fRec35: F32 = 0.6 * (fTemp10 - fTemp13);
            self.fRec41[0] = -(self.fConst19 * (self.fConst18 * self.fRec41[1] - (self.fRec9[1] + self.fRec9[2])));
            self.fRec40[0] =
                self.fConst120 * (self.fRec9[1] + self.fConst119 * self.fRec41[0]) + self.fConst118 * self.fRec40[1];
            self.fVec16[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec40[0] + 1e-20;
            let mut fTemp14: F32 =
                0.6 * self.fRec38[1] + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 32767) as usize];
            self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp14 - fTemp10;
            self.fRec38[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst123)) & 2047) as usize];
            let mut fRec39: F32 = 0.6 * (fTemp10 - fTemp14);
            self.fRec2[0] = self.fRec38[1]
                + self.fRec34[1]
                + self.fRec30[1]
                + self.fRec26[1]
                + self.fRec22[1]
                + self.fRec18[1]
                + self.fRec10[1]
                + self.fRec14[1]
                + fRec39
                + fRec35
                + fRec31
                + fRec27
                + fTemp8;
            self.fRec3[0] = self.fRec22[1] + self.fRec18[1] + self.fRec10[1] + self.fRec14[1] + fTemp8
                - (self.fRec38[1]
                    + self.fRec34[1]
                    + self.fRec30[1]
                    + self.fRec26[1]
                    + fRec39
                    + fRec35
                    + fRec27
                    + fRec31);
            let mut fTemp15: F32 = fRec19 + fRec23;
            self.fRec4[0] =
                self.fRec30[1] + self.fRec26[1] + self.fRec10[1] + self.fRec14[1] + fRec31 + fRec27 + fTemp5
                    - (self.fRec38[1] + self.fRec34[1] + self.fRec22[1] + self.fRec18[1] + fRec39 + fRec35 + fTemp15);
            self.fRec5[0] =
                self.fRec38[1] + self.fRec34[1] + self.fRec10[1] + self.fRec14[1] + fRec39 + fRec35 + fTemp5
                    - (self.fRec30[1] + self.fRec26[1] + self.fRec22[1] + self.fRec18[1] + fRec31 + fRec27 + fTemp15);
            let mut fTemp16: F32 = fRec11 + fRec23;
            let mut fTemp17: F32 = fRec15 + fRec19;
            self.fRec6[0] =
                self.fRec34[1] + self.fRec26[1] + self.fRec18[1] + self.fRec14[1] + fRec35 + fRec27 + fTemp17
                    - (self.fRec38[1] + self.fRec30[1] + self.fRec22[1] + self.fRec10[1] + fRec39 + fRec31 + fTemp16);
            self.fRec7[0] =
                self.fRec38[1] + self.fRec30[1] + self.fRec18[1] + self.fRec14[1] + fRec39 + fRec31 + fTemp17
                    - (self.fRec34[1] + self.fRec26[1] + self.fRec22[1] + self.fRec10[1] + fRec35 + fRec27 + fTemp16);
            let mut fTemp18: F32 = fRec11 + fRec19;
            let mut fTemp19: F32 = fRec15 + fRec23;
            self.fRec8[0] =
                self.fRec38[1] + self.fRec26[1] + self.fRec22[1] + self.fRec14[1] + fRec39 + fRec27 + fTemp19
                    - (self.fRec34[1] + self.fRec30[1] + self.fRec18[1] + self.fRec10[1] + fRec35 + fRec31 + fTemp18);
            self.fRec9[0] =
                self.fRec34[1] + self.fRec30[1] + self.fRec22[1] + self.fRec14[1] + fRec35 + fRec31 + fTemp19
                    - (self.fRec38[1] + self.fRec26[1] + self.fRec18[1] + self.fRec10[1] + fRec39 + fRec27 + fTemp18);
            let mut fTemp20: F32 = 0.37 * (self.fRec3[0] + self.fRec4[0]);
            let mut fTemp21: F32 = fTemp20 + fTemp0;
            self.fRec1[0] = fTemp21 - self.fConst5 * self.fRec1[2];
            let mut fTemp22: F32 = self.fConst5 * self.fRec1[0];
            self.fRec0[0] = 0.5 * (fTemp22 + fTemp20 + self.fRec1[2] - fTemp0 + (self.fRec1[2] + fTemp22 - fTemp21))
                + self.fConst3 * self.fRec0[1]
                - self.fConst2 * self.fRec0[2];
            self.fRec42[0] = fSlow0 + self.fConst127 * self.fRec42[1];
            let mut fTemp23: F32 = self.fRec42[0] + 1.0;
            let mut fTemp24: F32 = 1.0 - 0.5 * fTemp23;
            self.fRec43[0] = fSlow1 + self.fConst127 * self.fRec43[1];
            *output0 = 0.5
                * self.fRec43[0]
                * (fTemp1 * fTemp23
                    + fTemp24
                        * (2.0 * self.fRec0[2] + self.fConst125 * self.fRec0[0] - self.fConst124 * self.fRec0[1]));
            let mut fTemp25: F32 = self.fConst6 * self.fRec45[1];
            let mut fTemp26: F32 = 0.37 * (self.fRec3[0] - self.fRec4[0]);
            let mut fTemp27: F32 = fTemp26 + fTemp25;
            self.fRec45[0] = fTemp27 - self.fConst5 * self.fRec45[2];
            let mut fTemp28: F32 = self.fConst5 * self.fRec45[0];
            self.fRec44[0] = 0.5
                * (fTemp28 + fTemp26 + self.fRec45[2] - fTemp25 + (self.fRec45[2] + fTemp28 - fTemp27))
                + self.fConst3 * self.fRec44[1]
                - self.fConst2 * self.fRec44[2];
            *output1 = 0.5
                * self.fRec43[0]
                * (fTemp9 * fTemp23
                    + fTemp24
                        * (2.0 * self.fRec44[2] + self.fConst125 * self.fRec44[0] - self.fConst124 * self.fRec44[1]));
            self.fRec13[1] = self.fRec13[0];
            self.fRec12[1] = self.fRec12[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.fRec10[1] = self.fRec10[0];
            self.fRec17[1] = self.fRec17[0];
            self.fRec16[1] = self.fRec16[0];
            self.fRec14[1] = self.fRec14[0];
            self.fRec21[1] = self.fRec21[0];
            self.fRec20[1] = self.fRec20[0];
            self.fRec18[1] = self.fRec18[0];
            self.fRec25[1] = self.fRec25[0];
            self.fRec24[1] = self.fRec24[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec29[1] = self.fRec29[0];
            self.fRec28[1] = self.fRec28[0];
            self.fRec26[1] = self.fRec26[0];
            self.fRec33[1] = self.fRec33[0];
            self.fRec32[1] = self.fRec32[0];
            self.fRec30[1] = self.fRec30[0];
            self.fRec37[1] = self.fRec37[0];
            self.fRec36[1] = self.fRec36[0];
            self.fRec34[1] = self.fRec34[0];
            self.fRec41[1] = self.fRec41[0];
            self.fRec40[1] = self.fRec40[0];
            self.fRec38[1] = self.fRec38[0];
            self.fRec2[2] = self.fRec2[1];
            self.fRec2[1] = self.fRec2[0];
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
            self.fRec1[2] = self.fRec1[1];
            self.fRec1[1] = self.fRec1[0];
            self.fRec0[2] = self.fRec0[1];
            self.fRec0[1] = self.fRec0[0];
            self.fRec42[1] = self.fRec42[0];
            self.fRec43[1] = self.fRec43[0];
            self.fRec45[2] = self.fRec45[1];
            self.fRec45[1] = self.fRec45[0];
            self.fRec44[2] = self.fRec44[1];
            self.fRec44[1] = self.fRec44[0];
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
