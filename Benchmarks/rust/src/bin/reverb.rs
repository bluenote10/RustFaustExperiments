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

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
pub struct Dsp {
    fSampleRate: i32,
    fConst10: F32,
    fConst12: F32,
    fConst13: F32,
    fRec11: [F32; 2],
    fConst14: F32,
    fConst15: F32,
    fRec10: [F32; 2],
    IOTA0: i32,
    fVec0: [F32; 16384],
    iConst17: i32,
    fVec1: [F32; 16384],
    iConst18: i32,
    fVec2: [F32; 4096],
    iConst19: i32,
    fRec8: [F32; 2],
    fConst28: F32,
    fRec15: [F32; 2],
    fConst29: F32,
    fConst30: F32,
    fRec14: [F32; 2],
    fVec3: [F32; 16384],
    iConst32: i32,
    fVec4: [F32; 2048],
    iConst33: i32,
    fRec12: [F32; 2],
    fConst42: F32,
    fRec19: [F32; 2],
    fConst43: F32,
    fConst44: F32,
    fRec18: [F32; 2],
    fVec5: [F32; 16384],
    iConst46: i32,
    fVec6: [F32; 4096],
    iConst47: i32,
    fRec16: [F32; 2],
    fConst56: F32,
    fRec23: [F32; 2],
    fConst57: F32,
    fConst58: F32,
    fRec22: [F32; 2],
    fVec7: [F32; 16384],
    iConst60: i32,
    fVec8: [F32; 2048],
    iConst61: i32,
    fRec20: [F32; 2],
    fConst70: F32,
    fRec27: [F32; 2],
    fConst71: F32,
    fConst72: F32,
    fRec26: [F32; 2],
    fVec9: [F32; 32768],
    iConst74: i32,
    fVec10: [F32; 16384],
    fVec11: [F32; 4096],
    iConst75: i32,
    fRec24: [F32; 2],
    fConst84: F32,
    fRec31: [F32; 2],
    fConst85: F32,
    fConst86: F32,
    fRec30: [F32; 2],
    fVec12: [F32; 16384],
    iConst88: i32,
    fVec13: [F32; 4096],
    iConst89: i32,
    fRec28: [F32; 2],
    fConst98: F32,
    fRec35: [F32; 2],
    fConst99: F32,
    fConst100: F32,
    fRec34: [F32; 2],
    fVec14: [F32; 32768],
    iConst102: i32,
    fVec15: [F32; 4096],
    iConst103: i32,
    fRec32: [F32; 2],
    fConst112: F32,
    fRec39: [F32; 2],
    fConst113: F32,
    fConst114: F32,
    fRec38: [F32; 2],
    fVec16: [F32; 32768],
    iConst116: i32,
    fVec17: [F32; 2048],
    iConst117: i32,
    fRec36: [F32; 2],
    fRec0: [F32; 3],
    fRec1: [F32; 3],
    fRec2: [F32; 3],
    fRec3: [F32; 3],
    fRec4: [F32; 3],
    fRec5: [F32; 3],
    fRec6: [F32; 3],
    fRec7: [F32; 3],
    fConst119: F32,
    fConst120: F32,
    fRec40: [F32; 3],
    fConst122: F32,
    fConst123: F32,
    fRec41: [F32; 3],
    fConst124: F32,
    fConst125: F32,
    fVslider0: F32,
    fRec42: [F32; 2],
    fVslider1: F32,
    fRec43: [F32; 2],
    fRec44: [F32; 3],
    fRec45: [F32; 3],
}

impl FaustDsp for Dsp {
    type T = F32;

    fn new() -> Dsp {
        Dsp {
            fSampleRate: 0,
            fConst10: 0.0,
            fConst12: 0.0,
            fConst13: 0.0,
            fRec11: [0.0; 2],
            fConst14: 0.0,
            fConst15: 0.0,
            fRec10: [0.0; 2],
            IOTA0: 0,
            fVec0: [0.0; 16384],
            iConst17: 0,
            fVec1: [0.0; 16384],
            iConst18: 0,
            fVec2: [0.0; 4096],
            iConst19: 0,
            fRec8: [0.0; 2],
            fConst28: 0.0,
            fRec15: [0.0; 2],
            fConst29: 0.0,
            fConst30: 0.0,
            fRec14: [0.0; 2],
            fVec3: [0.0; 16384],
            iConst32: 0,
            fVec4: [0.0; 2048],
            iConst33: 0,
            fRec12: [0.0; 2],
            fConst42: 0.0,
            fRec19: [0.0; 2],
            fConst43: 0.0,
            fConst44: 0.0,
            fRec18: [0.0; 2],
            fVec5: [0.0; 16384],
            iConst46: 0,
            fVec6: [0.0; 4096],
            iConst47: 0,
            fRec16: [0.0; 2],
            fConst56: 0.0,
            fRec23: [0.0; 2],
            fConst57: 0.0,
            fConst58: 0.0,
            fRec22: [0.0; 2],
            fVec7: [0.0; 16384],
            iConst60: 0,
            fVec8: [0.0; 2048],
            iConst61: 0,
            fRec20: [0.0; 2],
            fConst70: 0.0,
            fRec27: [0.0; 2],
            fConst71: 0.0,
            fConst72: 0.0,
            fRec26: [0.0; 2],
            fVec9: [0.0; 32768],
            iConst74: 0,
            fVec10: [0.0; 16384],
            fVec11: [0.0; 4096],
            iConst75: 0,
            fRec24: [0.0; 2],
            fConst84: 0.0,
            fRec31: [0.0; 2],
            fConst85: 0.0,
            fConst86: 0.0,
            fRec30: [0.0; 2],
            fVec12: [0.0; 16384],
            iConst88: 0,
            fVec13: [0.0; 4096],
            iConst89: 0,
            fRec28: [0.0; 2],
            fConst98: 0.0,
            fRec35: [0.0; 2],
            fConst99: 0.0,
            fConst100: 0.0,
            fRec34: [0.0; 2],
            fVec14: [0.0; 32768],
            iConst102: 0,
            fVec15: [0.0; 4096],
            iConst103: 0,
            fRec32: [0.0; 2],
            fConst112: 0.0,
            fRec39: [0.0; 2],
            fConst113: 0.0,
            fConst114: 0.0,
            fRec38: [0.0; 2],
            fVec16: [0.0; 32768],
            iConst116: 0,
            fVec17: [0.0; 2048],
            iConst117: 0,
            fRec36: [0.0; 2],
            fRec0: [0.0; 3],
            fRec1: [0.0; 3],
            fRec2: [0.0; 3],
            fRec3: [0.0; 3],
            fRec4: [0.0; 3],
            fRec5: [0.0; 3],
            fRec6: [0.0; 3],
            fRec7: [0.0; 3],
            fConst119: 0.0,
            fConst120: 0.0,
            fRec40: [0.0; 3],
            fConst122: 0.0,
            fConst123: 0.0,
            fRec41: [0.0; 3],
            fConst124: 0.0,
            fConst125: 0.0,
            fVslider0: 0.0,
            fRec42: [0.0; 2],
            fVslider1: 0.0,
            fRec43: [0.0; 2],
            fRec44: [0.0; 3],
            fRec45: [0.0; 3],
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("basics.lib/name", "Faust Basic Element Library");
        m.declare("basics.lib/version", "0.9");
        m.declare("delays.lib/name", "Faust Delay Library");
        m.declare("delays.lib/version", "0.1");
        m.declare("demos.lib/name", "Faust Demos Library");
        m.declare("demos.lib/version", "0.1");
        m.declare("demos.lib/zita_light:author", "Julius O. Smith III");
        m.declare("demos.lib/zita_light:licence", "MIT");
        m.declare("filename", "reverb.dsp");
        m.declare("filters.lib/allpass_comb:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/allpass_comb:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/allpass_comb:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/fir:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/fir:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/iir:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/iir:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
        m.declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
        m.declare("filters.lib/lowpass:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/lowpass:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/lowpass:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/name", "Faust Filters Library");
        m.declare("filters.lib/peak_eq_rm:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/peak_eq_rm:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/peak_eq_rm:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/tf1:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/tf1:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf1:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/tf1s:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/tf1s:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf1s:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/tf2:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/tf2:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
        m.declare("filters.lib/version", "0.3");
        m.declare("maths.lib/author", "GRAME");
        m.declare("maths.lib/copyright", "GRAME");
        m.declare("maths.lib/license", "LGPL with exception");
        m.declare("maths.lib/name", "Faust Math Library");
        m.declare("maths.lib/version", "2.5");
        m.declare("name", "reverb");
        m.declare("platform.lib/name", "Generic Platform Library");
        m.declare("platform.lib/version", "0.3");
        m.declare("reverbs.lib/name", "Faust Reverb Library");
        m.declare("reverbs.lib/version", "0.2");
        m.declare("routes.lib/hadamard:author", "Remy Muller, revised by Romain Michon");
        m.declare("routes.lib/name", "Faust Signal Routing Library");
        m.declare("routes.lib/version", "0.2");
        m.declare("signals.lib/name", "Faust Signal Routing Library");
        m.declare("signals.lib/version", "0.3");
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
            self.fRec11[(l0) as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec10[(l1) as usize] = 0.0;
        }
        self.IOTA0 = 0;
        for l2 in 0..16384 {
            self.fVec0[(l2) as usize] = 0.0;
        }
        for l3 in 0..16384 {
            self.fVec1[(l3) as usize] = 0.0;
        }
        for l4 in 0..4096 {
            self.fVec2[(l4) as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fRec8[(l5) as usize] = 0.0;
        }
        for l6 in 0..2 {
            self.fRec15[(l6) as usize] = 0.0;
        }
        for l7 in 0..2 {
            self.fRec14[(l7) as usize] = 0.0;
        }
        for l8 in 0..16384 {
            self.fVec3[(l8) as usize] = 0.0;
        }
        for l9 in 0..2048 {
            self.fVec4[(l9) as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec12[(l10) as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fRec19[(l11) as usize] = 0.0;
        }
        for l12 in 0..2 {
            self.fRec18[(l12) as usize] = 0.0;
        }
        for l13 in 0..16384 {
            self.fVec5[(l13) as usize] = 0.0;
        }
        for l14 in 0..4096 {
            self.fVec6[(l14) as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec16[(l15) as usize] = 0.0;
        }
        for l16 in 0..2 {
            self.fRec23[(l16) as usize] = 0.0;
        }
        for l17 in 0..2 {
            self.fRec22[(l17) as usize] = 0.0;
        }
        for l18 in 0..16384 {
            self.fVec7[(l18) as usize] = 0.0;
        }
        for l19 in 0..2048 {
            self.fVec8[(l19) as usize] = 0.0;
        }
        for l20 in 0..2 {
            self.fRec20[(l20) as usize] = 0.0;
        }
        for l21 in 0..2 {
            self.fRec27[(l21) as usize] = 0.0;
        }
        for l22 in 0..2 {
            self.fRec26[(l22) as usize] = 0.0;
        }
        for l23 in 0..32768 {
            self.fVec9[(l23) as usize] = 0.0;
        }
        for l24 in 0..16384 {
            self.fVec10[(l24) as usize] = 0.0;
        }
        for l25 in 0..4096 {
            self.fVec11[(l25) as usize] = 0.0;
        }
        for l26 in 0..2 {
            self.fRec24[(l26) as usize] = 0.0;
        }
        for l27 in 0..2 {
            self.fRec31[(l27) as usize] = 0.0;
        }
        for l28 in 0..2 {
            self.fRec30[(l28) as usize] = 0.0;
        }
        for l29 in 0..16384 {
            self.fVec12[(l29) as usize] = 0.0;
        }
        for l30 in 0..4096 {
            self.fVec13[(l30) as usize] = 0.0;
        }
        for l31 in 0..2 {
            self.fRec28[(l31) as usize] = 0.0;
        }
        for l32 in 0..2 {
            self.fRec35[(l32) as usize] = 0.0;
        }
        for l33 in 0..2 {
            self.fRec34[(l33) as usize] = 0.0;
        }
        for l34 in 0..32768 {
            self.fVec14[(l34) as usize] = 0.0;
        }
        for l35 in 0..4096 {
            self.fVec15[(l35) as usize] = 0.0;
        }
        for l36 in 0..2 {
            self.fRec32[(l36) as usize] = 0.0;
        }
        for l37 in 0..2 {
            self.fRec39[(l37) as usize] = 0.0;
        }
        for l38 in 0..2 {
            self.fRec38[(l38) as usize] = 0.0;
        }
        for l39 in 0..32768 {
            self.fVec16[(l39) as usize] = 0.0;
        }
        for l40 in 0..2048 {
            self.fVec17[(l40) as usize] = 0.0;
        }
        for l41 in 0..2 {
            self.fRec36[(l41) as usize] = 0.0;
        }
        for l42 in 0..3 {
            self.fRec0[(l42) as usize] = 0.0;
        }
        for l43 in 0..3 {
            self.fRec1[(l43) as usize] = 0.0;
        }
        for l44 in 0..3 {
            self.fRec2[(l44) as usize] = 0.0;
        }
        for l45 in 0..3 {
            self.fRec3[(l45) as usize] = 0.0;
        }
        for l46 in 0..3 {
            self.fRec4[(l46) as usize] = 0.0;
        }
        for l47 in 0..3 {
            self.fRec5[(l47) as usize] = 0.0;
        }
        for l48 in 0..3 {
            self.fRec6[(l48) as usize] = 0.0;
        }
        for l49 in 0..3 {
            self.fRec7[(l49) as usize] = 0.0;
        }
        for l50 in 0..3 {
            self.fRec40[(l50) as usize] = 0.0;
        }
        for l51 in 0..3 {
            self.fRec41[(l51) as usize] = 0.0;
        }
        for l52 in 0..2 {
            self.fRec42[(l52) as usize] = 0.0;
        }
        for l53 in 0..2 {
            self.fRec43[(l53) as usize] = 0.0;
        }
        for l54 in 0..3 {
            self.fRec44[(l54) as usize] = 0.0;
        }
        for l55 in 0..3 {
            self.fRec45[(l55) as usize] = 0.0;
        }
    }
    fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        let mut fConst0: F32 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
        let mut fConst1: F32 = F32::floor(0.174713 * fConst0 + 0.5);
        let mut fConst2: F32 = (0.0 - 6.9077554 * fConst1) / fConst0;
        let mut fConst3: F32 = F32::exp(0.5 * fConst2);
        let mut fConst4: F32 = Dsp_faustpower2_f(fConst3);
        let mut fConst5: F32 = 1.0 - fConst4;
        let mut fConst6: F32 = F32::cos(37699.113 / fConst0);
        let mut fConst7: F32 = 1.0 - fConst6 * fConst4;
        let mut fConst8: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst7) / Dsp_faustpower2_f(fConst5) + -1.0,
        ));
        let mut fConst9: F32 = fConst7 / fConst5;
        self.fConst10 = fConst9 - fConst8;
        let mut fConst11: F32 = 1.0 / F32::tan(628.31854 / fConst0);
        self.fConst12 = 1.0 - fConst11;
        self.fConst13 = 1.0 / (fConst11 + 1.0);
        self.fConst14 = F32::exp(0.33333334 * fConst2) / fConst3 + -1.0;
        self.fConst15 = fConst3 * (fConst8 + (1.0 - fConst9));
        let mut fConst16: F32 = F32::floor(0.022904 * fConst0 + 0.5);
        self.iConst17 = ((F32::min(8192.0, F32::max(0.0, fConst1 - fConst16))) as i32);
        self.iConst18 = ((F32::min(8192.0, F32::max(0.0, 0.06 * fConst0))) as i32);
        self.iConst19 = ((F32::min(2048.0, F32::max(0.0, fConst16 + -1.0))) as i32);
        let mut fConst20: F32 = F32::floor(0.153129 * fConst0 + 0.5);
        let mut fConst21: F32 = (0.0 - 6.9077554 * fConst20) / fConst0;
        let mut fConst22: F32 = F32::exp(0.5 * fConst21);
        let mut fConst23: F32 = Dsp_faustpower2_f(fConst22);
        let mut fConst24: F32 = 1.0 - fConst23;
        let mut fConst25: F32 = 1.0 - fConst6 * fConst23;
        let mut fConst26: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst25) / Dsp_faustpower2_f(fConst24) + -1.0,
        ));
        let mut fConst27: F32 = fConst25 / fConst24;
        self.fConst28 = fConst27 - fConst26;
        self.fConst29 = F32::exp(0.33333334 * fConst21) / fConst22 + -1.0;
        self.fConst30 = fConst22 * (fConst26 + (1.0 - fConst27));
        let mut fConst31: F32 = F32::floor(0.020346 * fConst0 + 0.5);
        self.iConst32 = ((F32::min(8192.0, F32::max(0.0, fConst20 - fConst31))) as i32);
        self.iConst33 = ((F32::min(1024.0, F32::max(0.0, fConst31 + -1.0))) as i32);
        let mut fConst34: F32 = F32::floor(0.127837 * fConst0 + 0.5);
        let mut fConst35: F32 = (0.0 - 6.9077554 * fConst34) / fConst0;
        let mut fConst36: F32 = F32::exp(0.5 * fConst35);
        let mut fConst37: F32 = Dsp_faustpower2_f(fConst36);
        let mut fConst38: F32 = 1.0 - fConst37;
        let mut fConst39: F32 = 1.0 - fConst6 * fConst37;
        let mut fConst40: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst39) / Dsp_faustpower2_f(fConst38) + -1.0,
        ));
        let mut fConst41: F32 = fConst39 / fConst38;
        self.fConst42 = fConst41 - fConst40;
        self.fConst43 = F32::exp(0.33333334 * fConst35) / fConst36 + -1.0;
        self.fConst44 = fConst36 * (fConst40 + (1.0 - fConst41));
        let mut fConst45: F32 = F32::floor(0.031604 * fConst0 + 0.5);
        self.iConst46 = ((F32::min(8192.0, F32::max(0.0, fConst34 - fConst45))) as i32);
        self.iConst47 = ((F32::min(2048.0, F32::max(0.0, fConst45 + -1.0))) as i32);
        let mut fConst48: F32 = F32::floor(0.125 * fConst0 + 0.5);
        let mut fConst49: F32 = (0.0 - 6.9077554 * fConst48) / fConst0;
        let mut fConst50: F32 = F32::exp(0.5 * fConst49);
        let mut fConst51: F32 = Dsp_faustpower2_f(fConst50);
        let mut fConst52: F32 = 1.0 - fConst51;
        let mut fConst53: F32 = 1.0 - fConst6 * fConst51;
        let mut fConst54: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst53) / Dsp_faustpower2_f(fConst52) + -1.0,
        ));
        let mut fConst55: F32 = fConst53 / fConst52;
        self.fConst56 = fConst55 - fConst54;
        self.fConst57 = F32::exp(0.33333334 * fConst49) / fConst50 + -1.0;
        self.fConst58 = fConst50 * (fConst54 + (1.0 - fConst55));
        let mut fConst59: F32 = F32::floor(0.013458 * fConst0 + 0.5);
        self.iConst60 = ((F32::min(8192.0, F32::max(0.0, fConst48 - fConst59))) as i32);
        self.iConst61 = ((F32::min(1024.0, F32::max(0.0, fConst59 + -1.0))) as i32);
        let mut fConst62: F32 = F32::floor(0.210389 * fConst0 + 0.5);
        let mut fConst63: F32 = (0.0 - 6.9077554 * fConst62) / fConst0;
        let mut fConst64: F32 = F32::exp(0.5 * fConst63);
        let mut fConst65: F32 = Dsp_faustpower2_f(fConst64);
        let mut fConst66: F32 = 1.0 - fConst65;
        let mut fConst67: F32 = 1.0 - fConst6 * fConst65;
        let mut fConst68: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst67) / Dsp_faustpower2_f(fConst66) + -1.0,
        ));
        let mut fConst69: F32 = fConst67 / fConst66;
        self.fConst70 = fConst69 - fConst68;
        self.fConst71 = F32::exp(0.33333334 * fConst63) / fConst64 + -1.0;
        self.fConst72 = fConst64 * (fConst68 + (1.0 - fConst69));
        let mut fConst73: F32 = F32::floor(0.024421 * fConst0 + 0.5);
        self.iConst74 = ((F32::min(16384.0, F32::max(0.0, fConst62 - fConst73))) as i32);
        self.iConst75 = ((F32::min(2048.0, F32::max(0.0, fConst73 + -1.0))) as i32);
        let mut fConst76: F32 = F32::floor(0.192303 * fConst0 + 0.5);
        let mut fConst77: F32 = (0.0 - 6.9077554 * fConst76) / fConst0;
        let mut fConst78: F32 = F32::exp(0.5 * fConst77);
        let mut fConst79: F32 = Dsp_faustpower2_f(fConst78);
        let mut fConst80: F32 = 1.0 - fConst79;
        let mut fConst81: F32 = 1.0 - fConst6 * fConst79;
        let mut fConst82: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst81) / Dsp_faustpower2_f(fConst80) + -1.0,
        ));
        let mut fConst83: F32 = fConst81 / fConst80;
        self.fConst84 = fConst83 - fConst82;
        self.fConst85 = F32::exp(0.33333334 * fConst77) / fConst78 + -1.0;
        self.fConst86 = fConst78 * (fConst82 + (1.0 - fConst83));
        let mut fConst87: F32 = F32::floor(0.029291 * fConst0 + 0.5);
        self.iConst88 = ((F32::min(8192.0, F32::max(0.0, fConst76 - fConst87))) as i32);
        self.iConst89 = ((F32::min(2048.0, F32::max(0.0, fConst87 + -1.0))) as i32);
        let mut fConst90: F32 = F32::floor(0.256891 * fConst0 + 0.5);
        let mut fConst91: F32 = (0.0 - 6.9077554 * fConst90) / fConst0;
        let mut fConst92: F32 = F32::exp(0.5 * fConst91);
        let mut fConst93: F32 = Dsp_faustpower2_f(fConst92);
        let mut fConst94: F32 = 1.0 - fConst93;
        let mut fConst95: F32 = 1.0 - fConst6 * fConst93;
        let mut fConst96: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst95) / Dsp_faustpower2_f(fConst94) + -1.0,
        ));
        let mut fConst97: F32 = fConst95 / fConst94;
        self.fConst98 = fConst97 - fConst96;
        self.fConst99 = F32::exp(0.33333334 * fConst91) / fConst92 + -1.0;
        self.fConst100 = fConst92 * (fConst96 + (1.0 - fConst97));
        let mut fConst101: F32 = F32::floor(0.027333 * fConst0 + 0.5);
        self.iConst102 = ((F32::min(16384.0, F32::max(0.0, fConst90 - fConst101))) as i32);
        self.iConst103 = ((F32::min(2048.0, F32::max(0.0, fConst101 + -1.0))) as i32);
        let mut fConst104: F32 = F32::floor(0.219991 * fConst0 + 0.5);
        let mut fConst105: F32 = (0.0 - 6.9077554 * fConst104) / fConst0;
        let mut fConst106: F32 = F32::exp(0.5 * fConst105);
        let mut fConst107: F32 = Dsp_faustpower2_f(fConst106);
        let mut fConst108: F32 = 1.0 - fConst107;
        let mut fConst109: F32 = 1.0 - fConst6 * fConst107;
        let mut fConst110: F32 = F32::sqrt(F32::max(
            0.0,
            Dsp_faustpower2_f(fConst109) / Dsp_faustpower2_f(fConst108) + -1.0,
        ));
        let mut fConst111: F32 = fConst109 / fConst108;
        self.fConst112 = fConst111 - fConst110;
        self.fConst113 = F32::exp(0.33333334 * fConst105) / fConst106 + -1.0;
        self.fConst114 = fConst106 * (fConst110 + (1.0 - fConst111));
        let mut fConst115: F32 = F32::floor(0.019123 * fConst0 + 0.5);
        self.iConst116 = ((F32::min(16384.0, F32::max(0.0, fConst104 - fConst115))) as i32);
        self.iConst117 = ((F32::min(1024.0, F32::max(0.0, fConst115 + -1.0))) as i32);
        let mut fConst118: F32 = 1979.2034 / fConst0;
        self.fConst119 = (1.0 - fConst118) / (fConst118 + 1.0);
        self.fConst120 = 0.0 - F32::cos(fConst118) * (self.fConst119 + 1.0);
        let mut fConst121: F32 = 9424.778 / fConst0;
        self.fConst122 = (1.0 - fConst121) / (fConst121 + 1.0);
        self.fConst123 = 0.0 - F32::cos(fConst121) * (self.fConst122 + 1.0);
        self.fConst124 = 44.1 / fConst0;
        self.fConst125 = 1.0 - self.fConst124;
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
        let mut fSlow0: F32 = self.fConst124 * self.fVslider0;
        let mut fSlow1: F32 = self.fConst124 * F32::powf(1e+01, 0.05 * self.fVslider1);
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            self.fRec11[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec11[1] - (self.fRec4[1] + self.fRec4[2]));
            self.fRec10[0] =
                self.fConst15 * (self.fRec4[1] + self.fConst14 * self.fRec11[0]) + self.fConst10 * self.fRec10[1];
            self.fVec0[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec10[0] + 1e-20;
            let mut fTemp0: F32 = *input0;
            self.fVec1[(self.IOTA0 & 16383) as usize] = fTemp0;
            let mut fTemp1: F32 = 0.3 * self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst18)) & 16383) as usize];
            let mut fTemp2: F32 = fTemp1
                + self.fVec0[((i32::wrapping_sub(self.IOTA0, self.iConst17)) & 16383) as usize]
                - 0.6 * self.fRec8[1];
            self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp2;
            self.fRec8[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst19)) & 4095) as usize];
            let mut fRec9: F32 = 0.6 * fTemp2;
            self.fRec15[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec15[1] - (self.fRec0[1] + self.fRec0[2]));
            self.fRec14[0] =
                self.fConst30 * (self.fRec0[1] + self.fConst29 * self.fRec15[0]) + self.fConst28 * self.fRec14[1];
            self.fVec3[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec14[0] + 1e-20;
            let mut fTemp3: F32 = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst32)) & 16383) as usize]
                + fTemp1
                - 0.6 * self.fRec12[1];
            self.fVec4[(self.IOTA0 & 2047) as usize] = fTemp3;
            self.fRec12[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst33)) & 2047) as usize];
            let mut fRec13: F32 = 0.6 * fTemp3;
            let mut fTemp4: F32 = fRec13 + fRec9;
            self.fRec19[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec19[1] - (self.fRec2[1] + self.fRec2[2]));
            self.fRec18[0] =
                self.fConst44 * (self.fRec2[1] + self.fConst43 * self.fRec19[0]) + self.fConst42 * self.fRec18[1];
            self.fVec5[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec18[0] + 1e-20;
            let mut fTemp5: F32 = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst46)) & 16383) as usize]
                - (fTemp1 + 0.6 * self.fRec16[1]);
            self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp5;
            self.fRec16[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst47)) & 4095) as usize];
            let mut fRec17: F32 = 0.6 * fTemp5;
            self.fRec23[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec23[1] - (self.fRec6[1] + self.fRec6[2]));
            self.fRec22[0] =
                self.fConst58 * (self.fRec6[1] + self.fConst57 * self.fRec23[0]) + self.fConst56 * self.fRec22[1];
            self.fVec7[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec22[0] + 1e-20;
            let mut fTemp6: F32 = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst60)) & 16383) as usize]
                - (fTemp1 + 0.6 * self.fRec20[1]);
            self.fVec8[(self.IOTA0 & 2047) as usize] = fTemp6;
            self.fRec20[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst61)) & 2047) as usize];
            let mut fRec21: F32 = 0.6 * fTemp6;
            let mut fTemp7: F32 = fRec21 + fRec17 + fTemp4;
            self.fRec27[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec27[1] - (self.fRec1[1] + self.fRec1[2]));
            self.fRec26[0] =
                self.fConst72 * (self.fRec1[1] + self.fConst71 * self.fRec27[0]) + self.fConst70 * self.fRec26[1];
            self.fVec9[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec26[0] + 1e-20;
            let mut fTemp8: F32 = *input1;
            self.fVec10[(self.IOTA0 & 16383) as usize] = fTemp8;
            let mut fTemp9: F32 = 0.3 * self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst18)) & 16383) as usize];
            let mut fTemp10: F32 = fTemp9
                + 0.6 * self.fRec24[1]
                + self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst74)) & 32767) as usize];
            self.fVec11[(self.IOTA0 & 4095) as usize] = fTemp10;
            self.fRec24[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst75)) & 4095) as usize];
            let mut fRec25: F32 = 0.0 - 0.6 * fTemp10;
            self.fRec31[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec31[1] - (self.fRec5[1] + self.fRec5[2]));
            self.fRec30[0] =
                self.fConst86 * (self.fRec5[1] + self.fConst85 * self.fRec31[0]) + self.fConst84 * self.fRec30[1];
            self.fVec12[(self.IOTA0 & 16383) as usize] = 0.35355338 * self.fRec30[0] + 1e-20;
            let mut fTemp11: F32 = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst88)) & 16383) as usize]
                + fTemp9
                + 0.6 * self.fRec28[1];
            self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp11;
            self.fRec28[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst89)) & 4095) as usize];
            let mut fRec29: F32 = 0.0 - 0.6 * fTemp11;
            self.fRec35[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec35[1] - (self.fRec3[1] + self.fRec3[2]));
            self.fRec34[0] =
                self.fConst100 * (self.fRec3[1] + self.fConst99 * self.fRec35[0]) + self.fConst98 * self.fRec34[1];
            self.fVec14[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec34[0] + 1e-20;
            let mut fTemp12: F32 = 0.6 * self.fRec32[1]
                + self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst102)) & 32767) as usize]
                - fTemp9;
            self.fVec15[(self.IOTA0 & 4095) as usize] = fTemp12;
            self.fRec32[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst103)) & 4095) as usize];
            let mut fRec33: F32 = 0.0 - 0.6 * fTemp12;
            self.fRec39[0] = 0.0 - self.fConst13 * (self.fConst12 * self.fRec39[1] - (self.fRec7[1] + self.fRec7[2]));
            self.fRec38[0] =
                self.fConst114 * (self.fRec7[1] + self.fConst113 * self.fRec39[0]) + self.fConst112 * self.fRec38[1];
            self.fVec16[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec38[0] + 1e-20;
            let mut fTemp13: F32 = 0.6 * self.fRec36[1]
                + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 32767) as usize]
                - fTemp9;
            self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp13;
            self.fRec36[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 2047) as usize];
            let mut fRec37: F32 = 0.0 - 0.6 * fTemp13;
            self.fRec0[0] = self.fRec36[1]
                + self.fRec32[1]
                + self.fRec28[1]
                + self.fRec24[1]
                + self.fRec20[1]
                + self.fRec16[1]
                + self.fRec8[1]
                + self.fRec12[1]
                + fRec37
                + fRec33
                + fRec29
                + fRec25
                + fTemp7;
            self.fRec1[0] = self.fRec20[1] + self.fRec16[1] + self.fRec8[1] + self.fRec12[1] + fTemp7
                - (self.fRec36[1]
                    + self.fRec32[1]
                    + self.fRec28[1]
                    + self.fRec24[1]
                    + fRec37
                    + fRec33
                    + fRec25
                    + fRec29);
            let mut fTemp14: F32 = fRec17 + fRec21;
            self.fRec2[0] = self.fRec28[1] + self.fRec24[1] + self.fRec8[1] + self.fRec12[1] + fRec29 + fRec25 + fTemp4
                - (self.fRec36[1] + self.fRec32[1] + self.fRec20[1] + self.fRec16[1] + fRec37 + fRec33 + fTemp14);
            self.fRec3[0] = self.fRec36[1] + self.fRec32[1] + self.fRec8[1] + self.fRec12[1] + fRec37 + fRec33 + fTemp4
                - (self.fRec28[1] + self.fRec24[1] + self.fRec20[1] + self.fRec16[1] + fRec29 + fRec25 + fTemp14);
            let mut fTemp15: F32 = fRec9 + fRec21;
            let mut fTemp16: F32 = fRec13 + fRec17;
            self.fRec4[0] =
                self.fRec32[1] + self.fRec24[1] + self.fRec16[1] + self.fRec12[1] + fRec33 + fRec25 + fTemp16
                    - (self.fRec36[1] + self.fRec28[1] + self.fRec20[1] + self.fRec8[1] + fRec37 + fRec29 + fTemp15);
            self.fRec5[0] =
                self.fRec36[1] + self.fRec28[1] + self.fRec16[1] + self.fRec12[1] + fRec37 + fRec29 + fTemp16
                    - (self.fRec32[1] + self.fRec24[1] + self.fRec20[1] + self.fRec8[1] + fRec33 + fRec25 + fTemp15);
            let mut fTemp17: F32 = fRec9 + fRec17;
            let mut fTemp18: F32 = fRec13 + fRec21;
            self.fRec6[0] =
                self.fRec36[1] + self.fRec24[1] + self.fRec20[1] + self.fRec12[1] + fRec37 + fRec25 + fTemp18
                    - (self.fRec32[1] + self.fRec28[1] + self.fRec16[1] + self.fRec8[1] + fRec33 + fRec29 + fTemp17);
            self.fRec7[0] =
                self.fRec32[1] + self.fRec28[1] + self.fRec20[1] + self.fRec12[1] + fRec33 + fRec29 + fTemp18
                    - (self.fRec36[1] + self.fRec24[1] + self.fRec16[1] + self.fRec8[1] + fRec37 + fRec25 + fTemp17);
            let mut fTemp19: F32 = 0.37 * (self.fRec1[0] + self.fRec2[0]);
            let mut fTemp20: F32 = self.fConst120 * self.fRec40[1];
            self.fRec40[0] = fTemp19 - (fTemp20 + self.fConst119 * self.fRec40[2]);
            let mut fTemp21: F32 = self.fConst119 * self.fRec40[0];
            let mut fTemp22: F32 =
                0.5 * (fTemp21 + self.fRec40[2] + fTemp19 + fTemp20 + (fTemp21 + fTemp20 + self.fRec40[2] - fTemp19));
            let mut fTemp23: F32 = self.fConst123 * self.fRec41[1];
            self.fRec41[0] = fTemp22 - (fTemp23 + self.fConst122 * self.fRec41[2]);
            let mut fTemp24: F32 = self.fConst122 * self.fRec41[0];
            self.fRec42[0] = fSlow0 + self.fConst125 * self.fRec42[1];
            let mut fTemp25: F32 = self.fRec42[0] + 1.0;
            let mut fTemp26: F32 = 1.0 - 0.5 * fTemp25;
            self.fRec43[0] = fSlow1 + self.fConst125 * self.fRec43[1];
            *output0 = 0.5
                * self.fRec43[0]
                * (fTemp0 * fTemp25
                    + fTemp26
                        * (fTemp24
                            + self.fRec41[2]
                            + fTemp22
                            + fTemp23
                            + (fTemp24 + fTemp23 + self.fRec41[2] - fTemp22)));
            let mut fTemp27: F32 = 0.37 * (self.fRec1[0] - self.fRec2[0]);
            let mut fTemp28: F32 = self.fConst120 * self.fRec44[1];
            self.fRec44[0] = fTemp27 - (fTemp28 + self.fConst119 * self.fRec44[2]);
            let mut fTemp29: F32 = self.fConst119 * self.fRec44[0];
            let mut fTemp30: F32 =
                0.5 * (fTemp29 + self.fRec44[2] + fTemp27 + fTemp28 + (fTemp29 + fTemp28 + self.fRec44[2] - fTemp27));
            let mut fTemp31: F32 = self.fConst123 * self.fRec45[1];
            self.fRec45[0] = fTemp30 - (fTemp31 + self.fConst122 * self.fRec45[2]);
            let mut fTemp32: F32 = self.fConst122 * self.fRec45[0];
            *output1 = 0.5
                * self.fRec43[0]
                * (fTemp8 * fTemp25
                    + fTemp26
                        * (fTemp32
                            + self.fRec45[2]
                            + fTemp30
                            + fTemp31
                            + (fTemp32 + fTemp31 + self.fRec45[2] - fTemp30)));
            self.fRec11[1] = self.fRec11[0];
            self.fRec10[1] = self.fRec10[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.fRec8[1] = self.fRec8[0];
            self.fRec15[1] = self.fRec15[0];
            self.fRec14[1] = self.fRec14[0];
            self.fRec12[1] = self.fRec12[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            self.fRec16[1] = self.fRec16[0];
            self.fRec23[1] = self.fRec23[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec20[1] = self.fRec20[0];
            self.fRec27[1] = self.fRec27[0];
            self.fRec26[1] = self.fRec26[0];
            self.fRec24[1] = self.fRec24[0];
            self.fRec31[1] = self.fRec31[0];
            self.fRec30[1] = self.fRec30[0];
            self.fRec28[1] = self.fRec28[0];
            self.fRec35[1] = self.fRec35[0];
            self.fRec34[1] = self.fRec34[0];
            self.fRec32[1] = self.fRec32[0];
            self.fRec39[1] = self.fRec39[0];
            self.fRec38[1] = self.fRec38[0];
            self.fRec36[1] = self.fRec36[0];
            self.fRec0[2] = self.fRec0[1];
            self.fRec0[1] = self.fRec0[0];
            self.fRec1[2] = self.fRec1[1];
            self.fRec1[1] = self.fRec1[0];
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
            self.fRec40[2] = self.fRec40[1];
            self.fRec40[1] = self.fRec40[0];
            self.fRec41[2] = self.fRec41[1];
            self.fRec41[1] = self.fRec41[0];
            self.fRec42[1] = self.fRec42[0];
            self.fRec43[1] = self.fRec43[0];
            self.fRec44[2] = self.fRec44[1];
            self.fRec44[1] = self.fRec44[0];
            self.fRec45[2] = self.fRec45[1];
            self.fRec45[1] = self.fRec45[0];
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
