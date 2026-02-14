/* ------------------------------------------------------------
name: "reverb"
Code generated with Faust 2.84.4 (https://faust.grame.fr)
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

use std::env;
use std::path::PathBuf;

use faust_benchmarks::benchmark_runner::run_benchmark;
use faust_benchmarks::types::{FaustDsp, Meta, ParamIndex, UI};

type F32 = f32;
type FaustFloat = f32;

// Generated intrinsics:

// Generated class:

#[repr(C)]
pub struct Dsp {
    fSampleRate: i32,
    fConst0: F32,
    fConst1: F32,
    fConst2: F32,
    fConst3: F32,
    fConst4: F32,
    fConst5: F32,
    fConst6: F32,
    fConst7: F32,
    fConst8: F32,
    fConst9: F32,
    fConst10: F32,
    fConst11: F32,
    fConst12: F32,
    fConst13: F32,
    fConst14: F32,
    fConst15: F32,
    fConst16: F32,
    fConst17: F32,
    fConst18: F32,
    fConst19: F32,
    fRec13: [F32; 2],
    fConst20: F32,
    fConst21: F32,
    fRec12: [F32; 2],
    IOTA0: i32,
    fVec0: [F32; 16384],
    fConst22: F32,
    iConst23: i32,
    fVec1: [F32; 16384],
    iConst24: i32,
    fVec2: [F32; 4096],
    iConst25: i32,
    fRec10: [F32; 2],
    fConst26: F32,
    fConst27: F32,
    fConst28: F32,
    fConst29: F32,
    fConst30: F32,
    fConst31: F32,
    fConst32: F32,
    fConst33: F32,
    fConst34: F32,
    fRec17: [F32; 2],
    fConst35: F32,
    fConst36: F32,
    fRec16: [F32; 2],
    fVec3: [F32; 16384],
    fConst37: F32,
    iConst38: i32,
    fVec4: [F32; 2048],
    iConst39: i32,
    fRec14: [F32; 2],
    fConst40: F32,
    fConst41: F32,
    fConst42: F32,
    fConst43: F32,
    fConst44: F32,
    fConst45: F32,
    fConst46: F32,
    fConst47: F32,
    fConst48: F32,
    fRec21: [F32; 2],
    fConst49: F32,
    fConst50: F32,
    fRec20: [F32; 2],
    fVec5: [F32; 16384],
    fConst51: F32,
    iConst52: i32,
    fVec6: [F32; 4096],
    iConst53: i32,
    fRec18: [F32; 2],
    fConst54: F32,
    fConst55: F32,
    fConst56: F32,
    fConst57: F32,
    fConst58: F32,
    fConst59: F32,
    fConst60: F32,
    fConst61: F32,
    fConst62: F32,
    fRec25: [F32; 2],
    fConst63: F32,
    fConst64: F32,
    fRec24: [F32; 2],
    fVec7: [F32; 16384],
    fConst65: F32,
    iConst66: i32,
    fVec8: [F32; 2048],
    iConst67: i32,
    fRec22: [F32; 2],
    fConst68: F32,
    fConst69: F32,
    fConst70: F32,
    fConst71: F32,
    fConst72: F32,
    fConst73: F32,
    fConst74: F32,
    fConst75: F32,
    fConst76: F32,
    fRec29: [F32; 2],
    fConst77: F32,
    fConst78: F32,
    fRec28: [F32; 2],
    fVec9: [F32; 32768],
    fConst79: F32,
    iConst80: i32,
    fVec10: [F32; 16384],
    fVec11: [F32; 4096],
    iConst81: i32,
    fRec26: [F32; 2],
    fConst82: F32,
    fConst83: F32,
    fConst84: F32,
    fConst85: F32,
    fConst86: F32,
    fConst87: F32,
    fConst88: F32,
    fConst89: F32,
    fConst90: F32,
    fRec33: [F32; 2],
    fConst91: F32,
    fConst92: F32,
    fRec32: [F32; 2],
    fVec12: [F32; 16384],
    fConst93: F32,
    iConst94: i32,
    fVec13: [F32; 4096],
    iConst95: i32,
    fRec30: [F32; 2],
    fConst96: F32,
    fConst97: F32,
    fConst98: F32,
    fConst99: F32,
    fConst100: F32,
    fConst101: F32,
    fConst102: F32,
    fConst103: F32,
    fConst104: F32,
    fRec37: [F32; 2],
    fConst105: F32,
    fConst106: F32,
    fRec36: [F32; 2],
    fVec14: [F32; 32768],
    fConst107: F32,
    iConst108: i32,
    fVec15: [F32; 4096],
    iConst109: i32,
    fRec34: [F32; 2],
    fConst110: F32,
    fConst111: F32,
    fConst112: F32,
    fConst113: F32,
    fConst114: F32,
    fConst115: F32,
    fConst116: F32,
    fConst117: F32,
    fConst118: F32,
    fRec41: [F32; 2],
    fConst119: F32,
    fConst120: F32,
    fRec40: [F32; 2],
    fVec16: [F32; 32768],
    fConst121: F32,
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
    fVslider0: FaustFloat,
    fRec42: [F32; 2],
    fVslider1: FaustFloat,
    fRec43: [F32; 2],
    fRec45: [F32; 3],
    fRec44: [F32; 3],
}

fn Dsp_faustpower2_f(value: F32) -> F32 {
    return value * value;
}
#[cfg(not(target_arch = "wasm32"))] // Compile ffi bindings only on non-wasm targets
mod ffi {
    use core::ffi::c_float;
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

pub const FAUST_INPUTS: usize = 2;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 2;
pub const FAUST_PASSIVES: usize = 0;

impl Dsp {
    pub fn new() -> Dsp {
        Dsp {
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fConst2: 0.0,
            fConst3: 0.0,
            fConst4: 0.0,
            fConst5: 0.0,
            fConst6: 0.0,
            fConst7: 0.0,
            fConst8: 0.0,
            fConst9: 0.0,
            fConst10: 0.0,
            fConst11: 0.0,
            fConst12: 0.0,
            fConst13: 0.0,
            fConst14: 0.0,
            fConst15: 0.0,
            fConst16: 0.0,
            fConst17: 0.0,
            fConst18: 0.0,
            fConst19: 0.0,
            fRec13: [0.0; 2],
            fConst20: 0.0,
            fConst21: 0.0,
            fRec12: [0.0; 2],
            IOTA0: 0,
            fVec0: [0.0; 16384],
            fConst22: 0.0,
            iConst23: 0,
            fVec1: [0.0; 16384],
            iConst24: 0,
            fVec2: [0.0; 4096],
            iConst25: 0,
            fRec10: [0.0; 2],
            fConst26: 0.0,
            fConst27: 0.0,
            fConst28: 0.0,
            fConst29: 0.0,
            fConst30: 0.0,
            fConst31: 0.0,
            fConst32: 0.0,
            fConst33: 0.0,
            fConst34: 0.0,
            fRec17: [0.0; 2],
            fConst35: 0.0,
            fConst36: 0.0,
            fRec16: [0.0; 2],
            fVec3: [0.0; 16384],
            fConst37: 0.0,
            iConst38: 0,
            fVec4: [0.0; 2048],
            iConst39: 0,
            fRec14: [0.0; 2],
            fConst40: 0.0,
            fConst41: 0.0,
            fConst42: 0.0,
            fConst43: 0.0,
            fConst44: 0.0,
            fConst45: 0.0,
            fConst46: 0.0,
            fConst47: 0.0,
            fConst48: 0.0,
            fRec21: [0.0; 2],
            fConst49: 0.0,
            fConst50: 0.0,
            fRec20: [0.0; 2],
            fVec5: [0.0; 16384],
            fConst51: 0.0,
            iConst52: 0,
            fVec6: [0.0; 4096],
            iConst53: 0,
            fRec18: [0.0; 2],
            fConst54: 0.0,
            fConst55: 0.0,
            fConst56: 0.0,
            fConst57: 0.0,
            fConst58: 0.0,
            fConst59: 0.0,
            fConst60: 0.0,
            fConst61: 0.0,
            fConst62: 0.0,
            fRec25: [0.0; 2],
            fConst63: 0.0,
            fConst64: 0.0,
            fRec24: [0.0; 2],
            fVec7: [0.0; 16384],
            fConst65: 0.0,
            iConst66: 0,
            fVec8: [0.0; 2048],
            iConst67: 0,
            fRec22: [0.0; 2],
            fConst68: 0.0,
            fConst69: 0.0,
            fConst70: 0.0,
            fConst71: 0.0,
            fConst72: 0.0,
            fConst73: 0.0,
            fConst74: 0.0,
            fConst75: 0.0,
            fConst76: 0.0,
            fRec29: [0.0; 2],
            fConst77: 0.0,
            fConst78: 0.0,
            fRec28: [0.0; 2],
            fVec9: [0.0; 32768],
            fConst79: 0.0,
            iConst80: 0,
            fVec10: [0.0; 16384],
            fVec11: [0.0; 4096],
            iConst81: 0,
            fRec26: [0.0; 2],
            fConst82: 0.0,
            fConst83: 0.0,
            fConst84: 0.0,
            fConst85: 0.0,
            fConst86: 0.0,
            fConst87: 0.0,
            fConst88: 0.0,
            fConst89: 0.0,
            fConst90: 0.0,
            fRec33: [0.0; 2],
            fConst91: 0.0,
            fConst92: 0.0,
            fRec32: [0.0; 2],
            fVec12: [0.0; 16384],
            fConst93: 0.0,
            iConst94: 0,
            fVec13: [0.0; 4096],
            iConst95: 0,
            fRec30: [0.0; 2],
            fConst96: 0.0,
            fConst97: 0.0,
            fConst98: 0.0,
            fConst99: 0.0,
            fConst100: 0.0,
            fConst101: 0.0,
            fConst102: 0.0,
            fConst103: 0.0,
            fConst104: 0.0,
            fRec37: [0.0; 2],
            fConst105: 0.0,
            fConst106: 0.0,
            fRec36: [0.0; 2],
            fVec14: [0.0; 32768],
            fConst107: 0.0,
            iConst108: 0,
            fVec15: [0.0; 4096],
            iConst109: 0,
            fRec34: [0.0; 2],
            fConst110: 0.0,
            fConst111: 0.0,
            fConst112: 0.0,
            fConst113: 0.0,
            fConst114: 0.0,
            fConst115: 0.0,
            fConst116: 0.0,
            fConst117: 0.0,
            fConst118: 0.0,
            fRec41: [0.0; 2],
            fConst119: 0.0,
            fConst120: 0.0,
            fRec40: [0.0; 2],
            fVec16: [0.0; 32768],
            fConst121: 0.0,
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
            fVslider0: 0.0,
            fRec42: [0.0; 2],
            fVslider1: 0.0,
            fRec43: [0.0; 2],
            fRec45: [0.0; 3],
            fRec44: [0.0; 3],
        }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("basics.lib/name", r"Faust Basic Element Library");
        m.declare("basics.lib/version", r"1.22.0");
        m.declare("compile_options", r"-a ./architecture/benchmark.rs -lang rust -fpga-mem-th 4 -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
        m.declare("delays.lib/name", r"Faust Delay Library");
        m.declare("delays.lib/version", r"1.2.0");
        m.declare("demos.lib/name", r"Faust Demos Library");
        m.declare("demos.lib/version", r"1.4.0");
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
        m.declare("filters.lib/version", r"1.7.1");
        m.declare("maths.lib/author", r"GRAME");
        m.declare("maths.lib/copyright", r"GRAME");
        m.declare("maths.lib/license", r"LGPL with exception");
        m.declare("maths.lib/name", r"Faust Math Library");
        m.declare("maths.lib/version", r"2.9.0");
        m.declare("name", r"reverb");
        m.declare("platform.lib/name", r"Generic Platform Library");
        m.declare("platform.lib/version", r"1.3.0");
        m.declare("reverbs.lib/name", r"Faust Reverb Library");
        m.declare("reverbs.lib/version", r"1.5.1");
        m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
        m.declare("routes.lib/name", r"Faust Signal Routing Library");
        m.declare("routes.lib/version", r"1.2.0");
        m.declare("signals.lib/name", r"Faust Signal Routing Library");
        m.declare("signals.lib/version", r"1.6.0");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate as i32
    }

    pub fn class_init(sample_rate: i32) {
        // Obtaining locks on 0 static var(s)
    }
    pub fn instance_reset_params(&mut self) {
        self.fVslider0 = (0.0) as FaustFloat;
        self.fVslider1 = (-6.0) as FaustFloat;
    }
    pub fn instance_clear(&mut self) {
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
    pub fn instance_constants(&mut self, sample_rate: i32) {
        // Obtaining locks on 0 static var(s)
        self.fSampleRate = sample_rate;
        self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
        self.fConst1 = 9424.778 / self.fConst0;
        self.fConst2 = (1.0 - self.fConst1) / (self.fConst1 + 1.0);
        self.fConst3 = F32::cos(self.fConst1) * (self.fConst2 + 1.0);
        self.fConst4 = 1979.2034 / self.fConst0;
        self.fConst5 = (1.0 - self.fConst4) / (self.fConst4 + 1.0);
        self.fConst6 = F32::cos(self.fConst4) * (self.fConst5 + 1.0);
        self.fConst7 = F32::floor(0.174713 * self.fConst0 + 0.5);
        self.fConst8 = self.fConst7 / self.fConst0;
        self.fConst9 = F32::exp(-(3.4538777 * self.fConst8));
        self.fConst10 = Dsp_faustpower2_f(self.fConst9);
        self.fConst11 = 1.0 - self.fConst10;
        self.fConst12 = F32::cos(37699.113 / self.fConst0);
        self.fConst13 = 1.0 - self.fConst12 * self.fConst10;
        self.fConst14 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst13) / Dsp_faustpower2_f(self.fConst11) + -1.0,
        ));
        self.fConst15 = self.fConst13 / self.fConst11;
        self.fConst16 = self.fConst15 - self.fConst14;
        self.fConst17 = 1.0 / F32::tan(628.31854 / self.fConst0);
        self.fConst18 = 1.0 - self.fConst17;
        self.fConst19 = 1.0 / (self.fConst17 + 1.0);
        self.fConst20 = F32::exp(-(2.3025851 * self.fConst8)) / self.fConst9 + -1.0;
        self.fConst21 = self.fConst9 * (self.fConst14 + (1.0 - self.fConst15));
        self.fConst22 = F32::floor(0.022904 * self.fConst0 + 0.5);
        self.iConst23 = (F32::min(8192.0, F32::max(0.0, self.fConst7 - self.fConst22))) as i32;
        self.iConst24 = (F32::min(8192.0, F32::max(0.0, 0.06 * self.fConst0))) as i32;
        self.iConst25 = (F32::min(2048.0, F32::max(0.0, self.fConst22 + -1.0))) as i32;
        self.fConst26 = F32::floor(0.153129 * self.fConst0 + 0.5);
        self.fConst27 = self.fConst26 / self.fConst0;
        self.fConst28 = F32::exp(-(3.4538777 * self.fConst27));
        self.fConst29 = Dsp_faustpower2_f(self.fConst28);
        self.fConst30 = 1.0 - self.fConst29;
        self.fConst31 = 1.0 - self.fConst12 * self.fConst29;
        self.fConst32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst31) / Dsp_faustpower2_f(self.fConst30) + -1.0,
        ));
        self.fConst33 = self.fConst31 / self.fConst30;
        self.fConst34 = self.fConst33 - self.fConst32;
        self.fConst35 = F32::exp(-(2.3025851 * self.fConst27)) / self.fConst28 + -1.0;
        self.fConst36 = self.fConst28 * (self.fConst32 + (1.0 - self.fConst33));
        self.fConst37 = F32::floor(0.020346 * self.fConst0 + 0.5);
        self.iConst38 = (F32::min(8192.0, F32::max(0.0, self.fConst26 - self.fConst37))) as i32;
        self.iConst39 = (F32::min(1024.0, F32::max(0.0, self.fConst37 + -1.0))) as i32;
        self.fConst40 = F32::floor(0.127837 * self.fConst0 + 0.5);
        self.fConst41 = self.fConst40 / self.fConst0;
        self.fConst42 = F32::exp(-(3.4538777 * self.fConst41));
        self.fConst43 = Dsp_faustpower2_f(self.fConst42);
        self.fConst44 = 1.0 - self.fConst43;
        self.fConst45 = 1.0 - self.fConst12 * self.fConst43;
        self.fConst46 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst45) / Dsp_faustpower2_f(self.fConst44) + -1.0,
        ));
        self.fConst47 = self.fConst45 / self.fConst44;
        self.fConst48 = self.fConst47 - self.fConst46;
        self.fConst49 = F32::exp(-(2.3025851 * self.fConst41)) / self.fConst42 + -1.0;
        self.fConst50 = self.fConst42 * (self.fConst46 + (1.0 - self.fConst47));
        self.fConst51 = F32::floor(0.031604 * self.fConst0 + 0.5);
        self.iConst52 = (F32::min(8192.0, F32::max(0.0, self.fConst40 - self.fConst51))) as i32;
        self.iConst53 = (F32::min(2048.0, F32::max(0.0, self.fConst51 + -1.0))) as i32;
        self.fConst54 = F32::floor(0.125 * self.fConst0 + 0.5);
        self.fConst55 = self.fConst54 / self.fConst0;
        self.fConst56 = F32::exp(-(3.4538777 * self.fConst55));
        self.fConst57 = Dsp_faustpower2_f(self.fConst56);
        self.fConst58 = 1.0 - self.fConst57;
        self.fConst59 = 1.0 - self.fConst12 * self.fConst57;
        self.fConst60 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst59) / Dsp_faustpower2_f(self.fConst58) + -1.0,
        ));
        self.fConst61 = self.fConst59 / self.fConst58;
        self.fConst62 = self.fConst61 - self.fConst60;
        self.fConst63 = F32::exp(-(2.3025851 * self.fConst55)) / self.fConst56 + -1.0;
        self.fConst64 = self.fConst56 * (self.fConst60 + (1.0 - self.fConst61));
        self.fConst65 = F32::floor(0.013458 * self.fConst0 + 0.5);
        self.iConst66 = (F32::min(8192.0, F32::max(0.0, self.fConst54 - self.fConst65))) as i32;
        self.iConst67 = (F32::min(1024.0, F32::max(0.0, self.fConst65 + -1.0))) as i32;
        self.fConst68 = F32::floor(0.210389 * self.fConst0 + 0.5);
        self.fConst69 = self.fConst68 / self.fConst0;
        self.fConst70 = F32::exp(-(3.4538777 * self.fConst69));
        self.fConst71 = Dsp_faustpower2_f(self.fConst70);
        self.fConst72 = 1.0 - self.fConst71;
        self.fConst73 = 1.0 - self.fConst12 * self.fConst71;
        self.fConst74 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst73) / Dsp_faustpower2_f(self.fConst72) + -1.0,
        ));
        self.fConst75 = self.fConst73 / self.fConst72;
        self.fConst76 = self.fConst75 - self.fConst74;
        self.fConst77 = F32::exp(-(2.3025851 * self.fConst69)) / self.fConst70 + -1.0;
        self.fConst78 = self.fConst70 * (self.fConst74 + (1.0 - self.fConst75));
        self.fConst79 = F32::floor(0.024421 * self.fConst0 + 0.5);
        self.iConst80 = (F32::min(16384.0, F32::max(0.0, self.fConst68 - self.fConst79))) as i32;
        self.iConst81 = (F32::min(2048.0, F32::max(0.0, self.fConst79 + -1.0))) as i32;
        self.fConst82 = F32::floor(0.192303 * self.fConst0 + 0.5);
        self.fConst83 = self.fConst82 / self.fConst0;
        self.fConst84 = F32::exp(-(3.4538777 * self.fConst83));
        self.fConst85 = Dsp_faustpower2_f(self.fConst84);
        self.fConst86 = 1.0 - self.fConst85;
        self.fConst87 = 1.0 - self.fConst12 * self.fConst85;
        self.fConst88 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst87) / Dsp_faustpower2_f(self.fConst86) + -1.0,
        ));
        self.fConst89 = self.fConst87 / self.fConst86;
        self.fConst90 = self.fConst89 - self.fConst88;
        self.fConst91 = F32::exp(-(2.3025851 * self.fConst83)) / self.fConst84 + -1.0;
        self.fConst92 = self.fConst84 * (self.fConst88 + (1.0 - self.fConst89));
        self.fConst93 = F32::floor(0.029291 * self.fConst0 + 0.5);
        self.iConst94 = (F32::min(8192.0, F32::max(0.0, self.fConst82 - self.fConst93))) as i32;
        self.iConst95 = (F32::min(2048.0, F32::max(0.0, self.fConst93 + -1.0))) as i32;
        self.fConst96 = F32::floor(0.256891 * self.fConst0 + 0.5);
        self.fConst97 = self.fConst96 / self.fConst0;
        self.fConst98 = F32::exp(-(3.4538777 * self.fConst97));
        self.fConst99 = Dsp_faustpower2_f(self.fConst98);
        self.fConst100 = 1.0 - self.fConst99;
        self.fConst101 = 1.0 - self.fConst12 * self.fConst99;
        self.fConst102 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst101) / Dsp_faustpower2_f(self.fConst100) + -1.0,
        ));
        self.fConst103 = self.fConst101 / self.fConst100;
        self.fConst104 = self.fConst103 - self.fConst102;
        self.fConst105 = F32::exp(-(2.3025851 * self.fConst97)) / self.fConst98 + -1.0;
        self.fConst106 = self.fConst98 * (self.fConst102 + (1.0 - self.fConst103));
        self.fConst107 = F32::floor(0.027333 * self.fConst0 + 0.5);
        self.iConst108 = (F32::min(16384.0, F32::max(0.0, self.fConst96 - self.fConst107))) as i32;
        self.iConst109 = (F32::min(2048.0, F32::max(0.0, self.fConst107 + -1.0))) as i32;
        self.fConst110 = F32::floor(0.219991 * self.fConst0 + 0.5);
        self.fConst111 = self.fConst110 / self.fConst0;
        self.fConst112 = F32::exp(-(3.4538777 * self.fConst111));
        self.fConst113 = Dsp_faustpower2_f(self.fConst112);
        self.fConst114 = 1.0 - self.fConst113;
        self.fConst115 = 1.0 - self.fConst12 * self.fConst113;
        self.fConst116 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(self.fConst115) / Dsp_faustpower2_f(self.fConst114) + -1.0,
        ));
        self.fConst117 = self.fConst115 / self.fConst114;
        self.fConst118 = self.fConst117 - self.fConst116;
        self.fConst119 = F32::exp(-(2.3025851 * self.fConst111)) / self.fConst112 + -1.0;
        self.fConst120 = self.fConst112 * (self.fConst116 + (1.0 - self.fConst117));
        self.fConst121 = F32::floor(0.019123 * self.fConst0 + 0.5);
        self.iConst122 = (F32::min(16384.0, F32::max(0.0, self.fConst110 - self.fConst121))) as i32;
        self.iConst123 = (F32::min(1024.0, F32::max(0.0, self.fConst121 + -1.0))) as i32;
        self.fConst124 = 44.1 / self.fConst0;
        self.fConst125 = 1.0 - self.fConst124;
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
        ui_interface.open_horizontal_box("Zita Light");
        ui_interface.declare(Some(ParamIndex(0)), "1", "");
        ui_interface.declare(Some(ParamIndex(0)), "style", "knob");
        ui_interface.declare(
            Some(ParamIndex(0)),
            "tooltip",
            "Ratio of dry and wet signal. -1 = fully wet, +1 = fully dry",
        );
        ui_interface.add_vertical_slider("Wet/Dry Mix", ParamIndex(0), 0.0, -1.0, 1.0, 0.01);
        ui_interface.declare(Some(ParamIndex(1)), "2", "");
        ui_interface.declare(Some(ParamIndex(1)), "style", "knob");
        ui_interface.declare(Some(ParamIndex(1)), "tooltip", "Output scale         factor");
        ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
        ui_interface.add_vertical_slider("Level", ParamIndex(1), -6.0, -7e+01, 4e+01, 0.1);
        ui_interface.close_box();
    }

    pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
        match param.0 {
            0 => Some(self.fVslider0),
            1 => Some(self.fVslider1),
            _ => None,
        }
    }

    pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
        match param.0 {
            0 => self.fVslider0 = value,
            1 => self.fVslider1 = value,
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
        let [inputs0, inputs1, ..] = inputs.as_ref() else {
            panic!("wrong number of input buffers");
        };
        let inputs0 = inputs0.as_ref()[..count].iter();
        let inputs1 = inputs1.as_ref()[..count].iter();
        let [outputs0, outputs1, ..] = outputs.as_mut() else {
            panic!("wrong number of output buffers");
        };
        let outputs0 = outputs0.as_mut()[..count].iter_mut();
        let outputs1 = outputs1.as_mut()[..count].iter_mut();
        let mut fSlow0: F32 = self.fConst124 * (self.fVslider0) as F32;
        let mut fSlow1: F32 = self.fConst124 * F32::powf(1e+01, 0.05 * (self.fVslider1) as F32);
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            let mut fTemp0: F32 = self.fConst3 * self.fRec0[1];
            let mut fTemp1: F32 = self.fConst6 * self.fRec1[1];
            self.fRec13[0] = -(self.fConst19 * (self.fConst18 * self.fRec13[1] - (self.fRec6[1] + self.fRec6[2])));
            self.fRec12[0] =
                self.fConst21 * (self.fRec6[1] + self.fConst20 * self.fRec13[0]) + self.fConst16 * self.fRec12[1];
            self.fVec0[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec12[0] + 1e-20;
            let mut fTemp2: F32 = (*input0) as F32;
            self.fVec1[(self.IOTA0 & 16383) as usize] = fTemp2;
            let mut fTemp3: F32 = 0.3 * self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize];
            let mut fTemp4: F32 = fTemp3
                + self.fVec0[((i32::wrapping_sub(self.IOTA0, self.iConst23)) & 16383) as usize]
                - 0.6 * self.fRec10[1];
            self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp4;
            self.fRec10[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst25)) & 4095) as usize];
            let mut fRec11: F32 = 0.6 * fTemp4;
            self.fRec17[0] = -(self.fConst19 * (self.fConst18 * self.fRec17[1] - (self.fRec2[1] + self.fRec2[2])));
            self.fRec16[0] =
                self.fConst36 * (self.fRec2[1] + self.fConst35 * self.fRec17[0]) + self.fConst34 * self.fRec16[1];
            self.fVec3[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec16[0] + 1e-20;
            let mut fTemp5: F32 = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst38)) & 16383) as usize]
                + fTemp3
                - 0.6 * self.fRec14[1];
            self.fVec4[(self.IOTA0 & 2047) as usize] = fTemp5;
            self.fRec14[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst39)) & 2047) as usize];
            let mut fRec15: F32 = 0.6 * fTemp5;
            let mut fTemp6: F32 = fRec15 + fRec11;
            self.fRec21[0] = -(self.fConst19 * (self.fConst18 * self.fRec21[1] - (self.fRec4[1] + self.fRec4[2])));
            self.fRec20[0] =
                self.fConst50 * (self.fRec4[1] + self.fConst49 * self.fRec21[0]) + self.fConst48 * self.fRec20[1];
            self.fVec5[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec20[0] + 1e-20;
            let mut fTemp7: F32 = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst52)) & 16383) as usize]
                - (fTemp3 + 0.6 * self.fRec18[1]);
            self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp7;
            self.fRec18[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst53)) & 4095) as usize];
            let mut fRec19: F32 = 0.6 * fTemp7;
            self.fRec25[0] = -(self.fConst19 * (self.fConst18 * self.fRec25[1] - (self.fRec8[1] + self.fRec8[2])));
            self.fRec24[0] =
                self.fConst64 * (self.fRec8[1] + self.fConst63 * self.fRec25[0]) + self.fConst62 * self.fRec24[1];
            self.fVec7[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec24[0] + 1e-20;
            let mut fTemp8: F32 = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst66)) & 16383) as usize]
                - (fTemp3 + 0.6 * self.fRec22[1]);
            self.fVec8[(self.IOTA0 & 2047) as usize] = fTemp8;
            self.fRec22[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst67)) & 2047) as usize];
            let mut fRec23: F32 = 0.6 * fTemp8;
            let mut fTemp9: F32 = fRec23 + fRec19 + fTemp6;
            self.fRec29[0] = -(self.fConst19 * (self.fConst18 * self.fRec29[1] - (self.fRec3[1] + self.fRec3[2])));
            self.fRec28[0] =
                self.fConst78 * (self.fRec3[1] + self.fConst77 * self.fRec29[0]) + self.fConst76 * self.fRec28[1];
            self.fVec9[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec28[0] + 1e-20;
            let mut fTemp10: F32 = (*input1) as F32;
            self.fVec10[(self.IOTA0 & 16383) as usize] = fTemp10;
            let mut fTemp11: F32 = 0.3 * self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize];
            let mut fTemp12: F32 = fTemp11
                + 0.6 * self.fRec26[1]
                + self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst80)) & 32767) as usize];
            self.fVec11[(self.IOTA0 & 4095) as usize] = fTemp12;
            self.fRec26[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst81)) & 4095) as usize];
            let mut fRec27: F32 = -(0.6 * fTemp12);
            self.fRec33[0] = -(self.fConst19 * (self.fConst18 * self.fRec33[1] - (self.fRec7[1] + self.fRec7[2])));
            self.fRec32[0] =
                self.fConst92 * (self.fRec7[1] + self.fConst91 * self.fRec33[0]) + self.fConst90 * self.fRec32[1];
            self.fVec12[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec32[0] + 1e-20;
            let mut fTemp13: F32 = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst94)) & 16383) as usize]
                + fTemp11
                + 0.6 * self.fRec30[1];
            self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp13;
            self.fRec30[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst95)) & 4095) as usize];
            let mut fRec31: F32 = -(0.6 * fTemp13);
            self.fRec37[0] = -(self.fConst19 * (self.fConst18 * self.fRec37[1] - (self.fRec5[1] + self.fRec5[2])));
            self.fRec36[0] =
                self.fConst106 * (self.fRec5[1] + self.fConst105 * self.fRec37[0]) + self.fConst104 * self.fRec36[1];
            self.fVec14[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec36[0] + 1e-20;
            let mut fTemp14: F32 =
                0.6 * self.fRec34[1] + self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst108)) & 32767) as usize];
            self.fVec15[(self.IOTA0 & 4095) as usize] = fTemp14 - fTemp11;
            self.fRec34[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst109)) & 4095) as usize];
            let mut fRec35: F32 = 0.6 * (fTemp11 - fTemp14);
            self.fRec41[0] = -(self.fConst19 * (self.fConst18 * self.fRec41[1] - (self.fRec9[1] + self.fRec9[2])));
            self.fRec40[0] =
                self.fConst120 * (self.fRec9[1] + self.fConst119 * self.fRec41[0]) + self.fConst118 * self.fRec40[1];
            self.fVec16[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec40[0] + 1e-20;
            let mut fTemp15: F32 =
                0.6 * self.fRec38[1] + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 32767) as usize];
            self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp15 - fTemp11;
            self.fRec38[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst123)) & 2047) as usize];
            let mut fRec39: F32 = 0.6 * (fTemp11 - fTemp15);
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
                + fTemp9;
            self.fRec3[0] = self.fRec22[1] + self.fRec18[1] + self.fRec10[1] + self.fRec14[1] + fTemp9
                - (self.fRec38[1]
                    + self.fRec34[1]
                    + self.fRec30[1]
                    + self.fRec26[1]
                    + fRec39
                    + fRec35
                    + fRec27
                    + fRec31);
            let mut fTemp16: F32 = fRec19 + fRec23;
            self.fRec4[0] =
                self.fRec30[1] + self.fRec26[1] + self.fRec10[1] + self.fRec14[1] + fRec31 + fRec27 + fTemp6
                    - (self.fRec38[1] + self.fRec34[1] + self.fRec22[1] + self.fRec18[1] + fRec39 + fRec35 + fTemp16);
            self.fRec5[0] =
                self.fRec38[1] + self.fRec34[1] + self.fRec10[1] + self.fRec14[1] + fRec39 + fRec35 + fTemp6
                    - (self.fRec30[1] + self.fRec26[1] + self.fRec22[1] + self.fRec18[1] + fRec31 + fRec27 + fTemp16);
            let mut fTemp17: F32 = fRec11 + fRec23;
            let mut fTemp18: F32 = fRec15 + fRec19;
            self.fRec6[0] =
                self.fRec34[1] + self.fRec26[1] + self.fRec18[1] + self.fRec14[1] + fRec35 + fRec27 + fTemp18
                    - (self.fRec38[1] + self.fRec30[1] + self.fRec22[1] + self.fRec10[1] + fRec39 + fRec31 + fTemp17);
            self.fRec7[0] =
                self.fRec38[1] + self.fRec30[1] + self.fRec18[1] + self.fRec14[1] + fRec39 + fRec31 + fTemp18
                    - (self.fRec34[1] + self.fRec26[1] + self.fRec22[1] + self.fRec10[1] + fRec35 + fRec27 + fTemp17);
            let mut fTemp19: F32 = fRec11 + fRec19;
            let mut fTemp20: F32 = fRec15 + fRec23;
            self.fRec8[0] =
                self.fRec38[1] + self.fRec26[1] + self.fRec22[1] + self.fRec14[1] + fRec39 + fRec27 + fTemp20
                    - (self.fRec34[1] + self.fRec30[1] + self.fRec18[1] + self.fRec10[1] + fRec35 + fRec31 + fTemp19);
            self.fRec9[0] =
                self.fRec34[1] + self.fRec30[1] + self.fRec22[1] + self.fRec14[1] + fRec35 + fRec31 + fTemp20
                    - (self.fRec38[1] + self.fRec26[1] + self.fRec18[1] + self.fRec10[1] + fRec39 + fRec27 + fTemp19);
            let mut fTemp21: F32 = 0.37 * (self.fRec3[0] + self.fRec4[0]);
            let mut fTemp22: F32 = fTemp21 + fTemp1;
            self.fRec1[0] = fTemp22 - self.fConst5 * self.fRec1[2];
            let mut fTemp23: F32 = self.fConst5 * self.fRec1[0];
            let mut fTemp24: F32 =
                0.5 * (fTemp23 + fTemp21 + self.fRec1[2] - fTemp1 + (self.fRec1[2] + fTemp23 - fTemp22));
            let mut fTemp25: F32 = fTemp24 + fTemp0;
            self.fRec0[0] = fTemp25 - self.fConst2 * self.fRec0[2];
            let mut fTemp26: F32 = self.fConst2 * self.fRec0[0];
            self.fRec42[0] = fSlow0 + self.fConst125 * self.fRec42[1];
            let mut fTemp27: F32 = self.fRec42[0] + 1.0;
            let mut fTemp28: F32 = 1.0 - 0.5 * fTemp27;
            self.fRec43[0] = fSlow1 + self.fConst125 * self.fRec43[1];
            *output0 = (0.5
                * self.fRec43[0]
                * (fTemp2 * fTemp27
                    + fTemp28 * (fTemp26 + fTemp24 + self.fRec0[2] - fTemp0 + (self.fRec0[2] + fTemp26 - fTemp25))))
                as FaustFloat;
            let mut fTemp29: F32 = self.fConst3 * self.fRec44[1];
            let mut fTemp30: F32 = self.fConst6 * self.fRec45[1];
            let mut fTemp31: F32 = 0.37 * (self.fRec3[0] - self.fRec4[0]);
            let mut fTemp32: F32 = fTemp31 + fTemp30;
            self.fRec45[0] = fTemp32 - self.fConst5 * self.fRec45[2];
            let mut fTemp33: F32 = self.fConst5 * self.fRec45[0];
            let mut fTemp34: F32 =
                0.5 * (fTemp33 + fTemp31 + self.fRec45[2] - fTemp30 + (self.fRec45[2] + fTemp33 - fTemp32));
            let mut fTemp35: F32 = fTemp34 + fTemp29;
            self.fRec44[0] = fTemp35 - self.fConst2 * self.fRec44[2];
            let mut fTemp36: F32 = self.fConst2 * self.fRec44[0];
            *output1 = (0.5
                * self.fRec43[0]
                * (fTemp10 * fTemp27
                    + fTemp28 * (fTemp36 + fTemp34 + self.fRec44[2] - fTemp29 + (self.fRec44[2] + fTemp36 - fTemp35))))
                as FaustFloat;
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
