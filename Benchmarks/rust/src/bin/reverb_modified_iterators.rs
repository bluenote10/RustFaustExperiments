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

fn Dsp_faustpower2_f(value: f32) -> f32 {
    return (value * value);
}
pub struct Dsp {
    fVslider0: f32,
    fRec0: [f32; 2],
    IOTA: i32,
    fVec0: [f32; 16384],
    fVslider1: f32,
    fRec1: [f32; 2],
    fSampleRate: i32,
    fConst0: f32,
    fConst1: f32,
    fConst2: f32,
    fConst3: f32,
    fConst4: f32,
    fConst5: f32,
    fConst6: f32,
    fConst7: f32,
    fConst8: f32,
    fConst9: f32,
    fConst10: f32,
    fConst11: f32,
    fConst12: f32,
    fConst13: f32,
    fConst14: f32,
    fConst15: f32,
    fConst16: f32,
    fConst17: f32,
    fConst18: f32,
    fConst19: f32,
    fRec15: [f32; 2],
    fRec14: [f32; 2],
    fVec1: [f32; 32768],
    fConst20: f32,
    iConst21: i32,
    fVec2: [f32; 16384],
    iConst22: i32,
    fVec3: [f32; 2048],
    iConst23: i32,
    fRec12: [f32; 2],
    fConst24: f32,
    fConst25: f32,
    fConst26: f32,
    fConst27: f32,
    fConst28: f32,
    fConst29: f32,
    fConst30: f32,
    fConst31: f32,
    fConst32: f32,
    fConst33: f32,
    fConst34: f32,
    fRec19: [f32; 2],
    fRec18: [f32; 2],
    fVec4: [f32; 32768],
    fConst35: f32,
    iConst36: i32,
    fVec5: [f32; 4096],
    iConst37: i32,
    fRec16: [f32; 2],
    fConst38: f32,
    fConst39: f32,
    fConst40: f32,
    fConst41: f32,
    fConst42: f32,
    fConst43: f32,
    fConst44: f32,
    fConst45: f32,
    fConst46: f32,
    fConst47: f32,
    fConst48: f32,
    fRec23: [f32; 2],
    fRec22: [f32; 2],
    fVec6: [f32; 16384],
    fConst49: f32,
    iConst50: i32,
    fVec7: [f32; 4096],
    iConst51: i32,
    fRec20: [f32; 2],
    fConst52: f32,
    fConst53: f32,
    fConst54: f32,
    fConst55: f32,
    fConst56: f32,
    fConst57: f32,
    fConst58: f32,
    fConst59: f32,
    fConst60: f32,
    fConst61: f32,
    fConst62: f32,
    fRec27: [f32; 2],
    fRec26: [f32; 2],
    fVec8: [f32; 32768],
    fConst63: f32,
    iConst64: i32,
    fVec9: [f32; 4096],
    iConst65: i32,
    fRec24: [f32; 2],
    fConst66: f32,
    fConst67: f32,
    fConst68: f32,
    fConst69: f32,
    fConst70: f32,
    fConst71: f32,
    fConst72: f32,
    fConst73: f32,
    fConst74: f32,
    fConst75: f32,
    fConst76: f32,
    fRec31: [f32; 2],
    fRec30: [f32; 2],
    fVec10: [f32; 16384],
    fConst77: f32,
    iConst78: i32,
    fVec11: [f32; 2048],
    iConst79: i32,
    fRec28: [f32; 2],
    fConst80: f32,
    fConst81: f32,
    fConst82: f32,
    fConst83: f32,
    fConst84: f32,
    fConst85: f32,
    fConst86: f32,
    fConst87: f32,
    fConst88: f32,
    fConst89: f32,
    fConst90: f32,
    fRec35: [f32; 2],
    fRec34: [f32; 2],
    fVec12: [f32; 16384],
    fConst91: f32,
    iConst92: i32,
    fVec13: [f32; 4096],
    iConst93: i32,
    fRec32: [f32; 2],
    fConst94: f32,
    fConst95: f32,
    fConst96: f32,
    fConst97: f32,
    fConst98: f32,
    fConst99: f32,
    fConst100: f32,
    fConst101: f32,
    fConst102: f32,
    fConst103: f32,
    fConst104: f32,
    fRec39: [f32; 2],
    fRec38: [f32; 2],
    fVec14: [f32; 16384],
    fConst105: f32,
    iConst106: i32,
    fVec15: [f32; 4096],
    iConst107: i32,
    fRec36: [f32; 2],
    fConst108: f32,
    fConst109: f32,
    fConst110: f32,
    fConst111: f32,
    fConst112: f32,
    fConst113: f32,
    fConst114: f32,
    fConst115: f32,
    fConst116: f32,
    fConst117: f32,
    fConst118: f32,
    fRec43: [f32; 2],
    fRec42: [f32; 2],
    fVec16: [f32; 16384],
    fConst119: f32,
    iConst120: i32,
    fVec17: [f32; 2048],
    iConst121: i32,
    fRec40: [f32; 2],
    fRec4: [f32; 3],
    fRec5: [f32; 3],
    fRec6: [f32; 3],
    fRec7: [f32; 3],
    fRec8: [f32; 3],
    fRec9: [f32; 3],
    fRec10: [f32; 3],
    fRec11: [f32; 3],
    fConst122: f32,
    fRec3: [f32; 3],
    fConst123: f32,
    fRec2: [f32; 3],
    fRec45: [f32; 3],
    fRec44: [f32; 3],
}

impl FaustDsp for Dsp {
    type T = f32;

    fn new() -> Dsp {
        Dsp {
            fVslider0: 0.0,
            fRec0: [0.0; 2],
            IOTA: 0,
            fVec0: [0.0; 16384],
            fVslider1: 0.0,
            fRec1: [0.0; 2],
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
            fRec15: [0.0; 2],
            fRec14: [0.0; 2],
            fVec1: [0.0; 32768],
            fConst20: 0.0,
            iConst21: 0,
            fVec2: [0.0; 16384],
            iConst22: 0,
            fVec3: [0.0; 2048],
            iConst23: 0,
            fRec12: [0.0; 2],
            fConst24: 0.0,
            fConst25: 0.0,
            fConst26: 0.0,
            fConst27: 0.0,
            fConst28: 0.0,
            fConst29: 0.0,
            fConst30: 0.0,
            fConst31: 0.0,
            fConst32: 0.0,
            fConst33: 0.0,
            fConst34: 0.0,
            fRec19: [0.0; 2],
            fRec18: [0.0; 2],
            fVec4: [0.0; 32768],
            fConst35: 0.0,
            iConst36: 0,
            fVec5: [0.0; 4096],
            iConst37: 0,
            fRec16: [0.0; 2],
            fConst38: 0.0,
            fConst39: 0.0,
            fConst40: 0.0,
            fConst41: 0.0,
            fConst42: 0.0,
            fConst43: 0.0,
            fConst44: 0.0,
            fConst45: 0.0,
            fConst46: 0.0,
            fConst47: 0.0,
            fConst48: 0.0,
            fRec23: [0.0; 2],
            fRec22: [0.0; 2],
            fVec6: [0.0; 16384],
            fConst49: 0.0,
            iConst50: 0,
            fVec7: [0.0; 4096],
            iConst51: 0,
            fRec20: [0.0; 2],
            fConst52: 0.0,
            fConst53: 0.0,
            fConst54: 0.0,
            fConst55: 0.0,
            fConst56: 0.0,
            fConst57: 0.0,
            fConst58: 0.0,
            fConst59: 0.0,
            fConst60: 0.0,
            fConst61: 0.0,
            fConst62: 0.0,
            fRec27: [0.0; 2],
            fRec26: [0.0; 2],
            fVec8: [0.0; 32768],
            fConst63: 0.0,
            iConst64: 0,
            fVec9: [0.0; 4096],
            iConst65: 0,
            fRec24: [0.0; 2],
            fConst66: 0.0,
            fConst67: 0.0,
            fConst68: 0.0,
            fConst69: 0.0,
            fConst70: 0.0,
            fConst71: 0.0,
            fConst72: 0.0,
            fConst73: 0.0,
            fConst74: 0.0,
            fConst75: 0.0,
            fConst76: 0.0,
            fRec31: [0.0; 2],
            fRec30: [0.0; 2],
            fVec10: [0.0; 16384],
            fConst77: 0.0,
            iConst78: 0,
            fVec11: [0.0; 2048],
            iConst79: 0,
            fRec28: [0.0; 2],
            fConst80: 0.0,
            fConst81: 0.0,
            fConst82: 0.0,
            fConst83: 0.0,
            fConst84: 0.0,
            fConst85: 0.0,
            fConst86: 0.0,
            fConst87: 0.0,
            fConst88: 0.0,
            fConst89: 0.0,
            fConst90: 0.0,
            fRec35: [0.0; 2],
            fRec34: [0.0; 2],
            fVec12: [0.0; 16384],
            fConst91: 0.0,
            iConst92: 0,
            fVec13: [0.0; 4096],
            iConst93: 0,
            fRec32: [0.0; 2],
            fConst94: 0.0,
            fConst95: 0.0,
            fConst96: 0.0,
            fConst97: 0.0,
            fConst98: 0.0,
            fConst99: 0.0,
            fConst100: 0.0,
            fConst101: 0.0,
            fConst102: 0.0,
            fConst103: 0.0,
            fConst104: 0.0,
            fRec39: [0.0; 2],
            fRec38: [0.0; 2],
            fVec14: [0.0; 16384],
            fConst105: 0.0,
            iConst106: 0,
            fVec15: [0.0; 4096],
            iConst107: 0,
            fRec36: [0.0; 2],
            fConst108: 0.0,
            fConst109: 0.0,
            fConst110: 0.0,
            fConst111: 0.0,
            fConst112: 0.0,
            fConst113: 0.0,
            fConst114: 0.0,
            fConst115: 0.0,
            fConst116: 0.0,
            fConst117: 0.0,
            fConst118: 0.0,
            fRec43: [0.0; 2],
            fRec42: [0.0; 2],
            fVec16: [0.0; 16384],
            fConst119: 0.0,
            iConst120: 0,
            fVec17: [0.0; 2048],
            iConst121: 0,
            fRec40: [0.0; 2],
            fRec4: [0.0; 3],
            fRec5: [0.0; 3],
            fRec6: [0.0; 3],
            fRec7: [0.0; 3],
            fRec8: [0.0; 3],
            fRec9: [0.0; 3],
            fRec10: [0.0; 3],
            fRec11: [0.0; 3],
            fConst122: 0.0,
            fRec3: [0.0; 3],
            fConst123: 0.0,
            fRec2: [0.0; 3],
            fRec45: [0.0; 3],
            fRec44: [0.0; 3],
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("basics.lib/name", "Faust Basic Element Library");
        m.declare("basics.lib/version", "0.1");
        m.declare("delays.lib/name", "Faust Delay Library");
        m.declare("delays.lib/version", "0.1");
        m.declare("filename", "reverb.dsp");
        m.declare("filters.lib/allpass_comb:author", "Julius O. Smith III");
        m.declare(
            "filters.lib/allpass_comb:copyright",
            "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>",
        );
        m.declare(
            "filters.lib/allpass_comb:license",
            "MIT-style STK-4.3 license",
        );
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
        m.declare(
            "filters.lib/lowpass0_highpass1",
            "MIT-style STK-4.3 license",
        );
        m.declare(
            "filters.lib/lowpass0_highpass1:author",
            "Julius O. Smith III",
        );
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
        m.declare(
            "filters.lib/peak_eq_rm:license",
            "MIT-style STK-4.3 license",
        );
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
        m.declare("maths.lib/author", "GRAME");
        m.declare("maths.lib/copyright", "GRAME");
        m.declare("maths.lib/license", "LGPL with exception");
        m.declare("maths.lib/name", "Faust Math Library");
        m.declare("maths.lib/version", "2.3");
        m.declare("name", "reverb");
        m.declare("platform.lib/name", "Generic Platform Library");
        m.declare("platform.lib/version", "0.1");
        m.declare("reverbs.lib/name", "Faust Reverb Library");
        m.declare("reverbs.lib/version", "0.0");
        m.declare("routes.lib/name", "Faust Signal Routing Library");
        m.declare("routes.lib/version", "0.2");
        m.declare("signals.lib/name", "Faust Signal Routing Library");
        m.declare("signals.lib/version", "0.0");
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
    fn instance_reset_params(&mut self) {
        self.fVslider0 = -6.0;
        self.fVslider1 = 0.0;
    }
    fn instance_clear(&mut self) {
        for l0 in 0..2 {
            self.fRec0[l0 as usize] = 0.0;
        }
        self.IOTA = 0;
        for l1 in 0..16384 {
            self.fVec0[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.fRec1[l2 as usize] = 0.0;
        }
        for l3 in 0..2 {
            self.fRec15[l3 as usize] = 0.0;
        }
        for l4 in 0..2 {
            self.fRec14[l4 as usize] = 0.0;
        }
        for l5 in 0..32768 {
            self.fVec1[l5 as usize] = 0.0;
        }
        for l6 in 0..16384 {
            self.fVec2[l6 as usize] = 0.0;
        }
        for l7 in 0..2048 {
            self.fVec3[l7 as usize] = 0.0;
        }
        for l8 in 0..2 {
            self.fRec12[l8 as usize] = 0.0;
        }
        for l9 in 0..2 {
            self.fRec19[l9 as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec18[l10 as usize] = 0.0;
        }
        for l11 in 0..32768 {
            self.fVec4[l11 as usize] = 0.0;
        }
        for l12 in 0..4096 {
            self.fVec5[l12 as usize] = 0.0;
        }
        for l13 in 0..2 {
            self.fRec16[l13 as usize] = 0.0;
        }
        for l14 in 0..2 {
            self.fRec23[l14 as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec22[l15 as usize] = 0.0;
        }
        for l16 in 0..16384 {
            self.fVec6[l16 as usize] = 0.0;
        }
        for l17 in 0..4096 {
            self.fVec7[l17 as usize] = 0.0;
        }
        for l18 in 0..2 {
            self.fRec20[l18 as usize] = 0.0;
        }
        for l19 in 0..2 {
            self.fRec27[l19 as usize] = 0.0;
        }
        for l20 in 0..2 {
            self.fRec26[l20 as usize] = 0.0;
        }
        for l21 in 0..32768 {
            self.fVec8[l21 as usize] = 0.0;
        }
        for l22 in 0..4096 {
            self.fVec9[l22 as usize] = 0.0;
        }
        for l23 in 0..2 {
            self.fRec24[l23 as usize] = 0.0;
        }
        for l24 in 0..2 {
            self.fRec31[l24 as usize] = 0.0;
        }
        for l25 in 0..2 {
            self.fRec30[l25 as usize] = 0.0;
        }
        for l26 in 0..16384 {
            self.fVec10[l26 as usize] = 0.0;
        }
        for l27 in 0..2048 {
            self.fVec11[l27 as usize] = 0.0;
        }
        for l28 in 0..2 {
            self.fRec28[l28 as usize] = 0.0;
        }
        for l29 in 0..2 {
            self.fRec35[l29 as usize] = 0.0;
        }
        for l30 in 0..2 {
            self.fRec34[l30 as usize] = 0.0;
        }
        for l31 in 0..16384 {
            self.fVec12[l31 as usize] = 0.0;
        }
        for l32 in 0..4096 {
            self.fVec13[l32 as usize] = 0.0;
        }
        for l33 in 0..2 {
            self.fRec32[l33 as usize] = 0.0;
        }
        for l34 in 0..2 {
            self.fRec39[l34 as usize] = 0.0;
        }
        for l35 in 0..2 {
            self.fRec38[l35 as usize] = 0.0;
        }
        for l36 in 0..16384 {
            self.fVec14[l36 as usize] = 0.0;
        }
        for l37 in 0..4096 {
            self.fVec15[l37 as usize] = 0.0;
        }
        for l38 in 0..2 {
            self.fRec36[l38 as usize] = 0.0;
        }
        for l39 in 0..2 {
            self.fRec43[l39 as usize] = 0.0;
        }
        for l40 in 0..2 {
            self.fRec42[l40 as usize] = 0.0;
        }
        for l41 in 0..16384 {
            self.fVec16[l41 as usize] = 0.0;
        }
        for l42 in 0..2048 {
            self.fVec17[l42 as usize] = 0.0;
        }
        for l43 in 0..2 {
            self.fRec40[l43 as usize] = 0.0;
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
            self.fRec10[l50 as usize] = 0.0;
        }
        for l51 in 0..3 {
            self.fRec11[l51 as usize] = 0.0;
        }
        for l52 in 0..3 {
            self.fRec3[l52 as usize] = 0.0;
        }
        for l53 in 0..3 {
            self.fRec2[l53 as usize] = 0.0;
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
        self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
        self.fConst1 = (9424.77832 / self.fConst0);
        self.fConst2 = ((1.0 - self.fConst1) / (self.fConst1 + 1.0));
        self.fConst3 = (1979.20337 / self.fConst0);
        self.fConst4 = ((1.0 - self.fConst3) / (self.fConst3 + 1.0));
        self.fConst5 = f32::floor(((0.219990999 * self.fConst0) + 0.5));
        self.fConst6 = ((0.0 - (6.90775537 * self.fConst5)) / self.fConst0);
        self.fConst7 = f32::exp((0.5 * self.fConst6));
        self.fConst8 = Dsp_faustpower2_f(self.fConst7);
        self.fConst9 = f32::cos((37699.1133 / self.fConst0));
        self.fConst10 = (1.0 - (self.fConst8 * self.fConst9));
        self.fConst11 = (1.0 - self.fConst8);
        self.fConst12 = (self.fConst10 / self.fConst11);
        self.fConst13 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst10) / Dsp_faustpower2_f(self.fConst11)) + -1.0),
        ));
        self.fConst14 = (self.fConst12 - self.fConst13);
        self.fConst15 = (self.fConst7 * (self.fConst13 + (1.0 - self.fConst12)));
        self.fConst16 = ((f32::exp((0.333333343 * self.fConst6)) / self.fConst7) + -1.0);
        self.fConst17 = (1.0 / f32::tan((628.318542 / self.fConst0)));
        self.fConst18 = (1.0 / (self.fConst17 + 1.0));
        self.fConst19 = (1.0 - self.fConst17);
        self.fConst20 = f32::floor(((0.0191229992 * self.fConst0) + 0.5));
        self.iConst21 = (f32::min(16384.0, f32::max(0.0, (self.fConst5 - self.fConst20))) as i32);
        self.iConst22 = (f32::min(8192.0, f32::max(0.0, (0.0599999987 * self.fConst0))) as i32);
        self.iConst23 = (f32::min(1024.0, f32::max(0.0, (self.fConst20 + -1.0))) as i32);
        self.fConst24 = f32::floor(((0.256891012 * self.fConst0) + 0.5));
        self.fConst25 = ((0.0 - (6.90775537 * self.fConst24)) / self.fConst0);
        self.fConst26 = f32::exp((0.5 * self.fConst25));
        self.fConst27 = Dsp_faustpower2_f(self.fConst26);
        self.fConst28 = (1.0 - (self.fConst27 * self.fConst9));
        self.fConst29 = (1.0 - self.fConst27);
        self.fConst30 = (self.fConst28 / self.fConst29);
        self.fConst31 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst28) / Dsp_faustpower2_f(self.fConst29)) + -1.0),
        ));
        self.fConst32 = (self.fConst30 - self.fConst31);
        self.fConst33 = (self.fConst26 * (self.fConst31 + (1.0 - self.fConst30)));
        self.fConst34 = ((f32::exp((0.333333343 * self.fConst25)) / self.fConst26) + -1.0);
        self.fConst35 = f32::floor(((0.0273330007 * self.fConst0) + 0.5));
        self.iConst36 = (f32::min(16384.0, f32::max(0.0, (self.fConst24 - self.fConst35))) as i32);
        self.iConst37 = (f32::min(2048.0, f32::max(0.0, (self.fConst35 + -1.0))) as i32);
        self.fConst38 = f32::floor(((0.192303002 * self.fConst0) + 0.5));
        self.fConst39 = ((0.0 - (6.90775537 * self.fConst38)) / self.fConst0);
        self.fConst40 = f32::exp((0.5 * self.fConst39));
        self.fConst41 = Dsp_faustpower2_f(self.fConst40);
        self.fConst42 = (1.0 - (self.fConst41 * self.fConst9));
        self.fConst43 = (1.0 - self.fConst41);
        self.fConst44 = (self.fConst42 / self.fConst43);
        self.fConst45 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst42) / Dsp_faustpower2_f(self.fConst43)) + -1.0),
        ));
        self.fConst46 = (self.fConst44 - self.fConst45);
        self.fConst47 = (self.fConst40 * (self.fConst45 + (1.0 - self.fConst44)));
        self.fConst48 = ((f32::exp((0.333333343 * self.fConst39)) / self.fConst40) + -1.0);
        self.fConst49 = f32::floor(((0.0292910002 * self.fConst0) + 0.5));
        self.iConst50 = (f32::min(8192.0, f32::max(0.0, (self.fConst38 - self.fConst49))) as i32);
        self.iConst51 = (f32::min(2048.0, f32::max(0.0, (self.fConst49 + -1.0))) as i32);
        self.fConst52 = f32::floor(((0.210389003 * self.fConst0) + 0.5));
        self.fConst53 = ((0.0 - (6.90775537 * self.fConst52)) / self.fConst0);
        self.fConst54 = f32::exp((0.5 * self.fConst53));
        self.fConst55 = Dsp_faustpower2_f(self.fConst54);
        self.fConst56 = (1.0 - (self.fConst55 * self.fConst9));
        self.fConst57 = (1.0 - self.fConst55);
        self.fConst58 = (self.fConst56 / self.fConst57);
        self.fConst59 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst56) / Dsp_faustpower2_f(self.fConst57)) + -1.0),
        ));
        self.fConst60 = (self.fConst58 - self.fConst59);
        self.fConst61 = (self.fConst54 * (self.fConst59 + (1.0 - self.fConst58)));
        self.fConst62 = ((f32::exp((0.333333343 * self.fConst53)) / self.fConst54) + -1.0);
        self.fConst63 = f32::floor(((0.0244210009 * self.fConst0) + 0.5));
        self.iConst64 = (f32::min(16384.0, f32::max(0.0, (self.fConst52 - self.fConst63))) as i32);
        self.iConst65 = (f32::min(2048.0, f32::max(0.0, (self.fConst63 + -1.0))) as i32);
        self.fConst66 = f32::floor(((0.125 * self.fConst0) + 0.5));
        self.fConst67 = ((0.0 - (6.90775537 * self.fConst66)) / self.fConst0);
        self.fConst68 = f32::exp((0.5 * self.fConst67));
        self.fConst69 = Dsp_faustpower2_f(self.fConst68);
        self.fConst70 = (1.0 - (self.fConst69 * self.fConst9));
        self.fConst71 = (1.0 - self.fConst69);
        self.fConst72 = (self.fConst70 / self.fConst71);
        self.fConst73 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst70) / Dsp_faustpower2_f(self.fConst71)) + -1.0),
        ));
        self.fConst74 = (self.fConst72 - self.fConst73);
        self.fConst75 = (self.fConst68 * (self.fConst73 + (1.0 - self.fConst72)));
        self.fConst76 = ((f32::exp((0.333333343 * self.fConst67)) / self.fConst68) + -1.0);
        self.fConst77 = f32::floor(((0.0134579996 * self.fConst0) + 0.5));
        self.iConst78 = (f32::min(8192.0, f32::max(0.0, (self.fConst66 - self.fConst77))) as i32);
        self.iConst79 = (f32::min(1024.0, f32::max(0.0, (self.fConst77 + -1.0))) as i32);
        self.fConst80 = f32::floor(((0.127837002 * self.fConst0) + 0.5));
        self.fConst81 = ((0.0 - (6.90775537 * self.fConst80)) / self.fConst0);
        self.fConst82 = f32::exp((0.5 * self.fConst81));
        self.fConst83 = Dsp_faustpower2_f(self.fConst82);
        self.fConst84 = (1.0 - (self.fConst83 * self.fConst9));
        self.fConst85 = (1.0 - self.fConst83);
        self.fConst86 = (self.fConst84 / self.fConst85);
        self.fConst87 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst84) / Dsp_faustpower2_f(self.fConst85)) + -1.0),
        ));
        self.fConst88 = (self.fConst86 - self.fConst87);
        self.fConst89 = (self.fConst82 * (self.fConst87 + (1.0 - self.fConst86)));
        self.fConst90 = ((f32::exp((0.333333343 * self.fConst81)) / self.fConst82) + -1.0);
        self.fConst91 = f32::floor(((0.0316039994 * self.fConst0) + 0.5));
        self.iConst92 = (f32::min(8192.0, f32::max(0.0, (self.fConst80 - self.fConst91))) as i32);
        self.iConst93 = (f32::min(2048.0, f32::max(0.0, (self.fConst91 + -1.0))) as i32);
        self.fConst94 = f32::floor(((0.174713001 * self.fConst0) + 0.5));
        self.fConst95 = ((0.0 - (6.90775537 * self.fConst94)) / self.fConst0);
        self.fConst96 = f32::exp((0.5 * self.fConst95));
        self.fConst97 = Dsp_faustpower2_f(self.fConst96);
        self.fConst98 = (1.0 - (self.fConst97 * self.fConst9));
        self.fConst99 = (1.0 - self.fConst97);
        self.fConst100 = (self.fConst98 / self.fConst99);
        self.fConst101 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst98) / Dsp_faustpower2_f(self.fConst99)) + -1.0),
        ));
        self.fConst102 = (self.fConst100 - self.fConst101);
        self.fConst103 = (self.fConst96 * (self.fConst101 + (1.0 - self.fConst100)));
        self.fConst104 = ((f32::exp((0.333333343 * self.fConst95)) / self.fConst96) + -1.0);
        self.fConst105 = f32::floor(((0.0229039993 * self.fConst0) + 0.5));
        self.iConst106 = (f32::min(8192.0, f32::max(0.0, (self.fConst94 - self.fConst105))) as i32);
        self.iConst107 = (f32::min(2048.0, f32::max(0.0, (self.fConst105 + -1.0))) as i32);
        self.fConst108 = f32::floor(((0.153128996 * self.fConst0) + 0.5));
        self.fConst109 = ((0.0 - (6.90775537 * self.fConst108)) / self.fConst0);
        self.fConst110 = f32::exp((0.5 * self.fConst109));
        self.fConst111 = Dsp_faustpower2_f(self.fConst110);
        self.fConst112 = (1.0 - (self.fConst111 * self.fConst9));
        self.fConst113 = (1.0 - self.fConst111);
        self.fConst114 = (self.fConst112 / self.fConst113);
        self.fConst115 = f32::sqrt(f32::max(
            0.0,
            ((Dsp_faustpower2_f(self.fConst112) / Dsp_faustpower2_f(self.fConst113)) + -1.0),
        ));
        self.fConst116 = (self.fConst114 - self.fConst115);
        self.fConst117 = (self.fConst110 * (self.fConst115 + (1.0 - self.fConst114)));
        self.fConst118 = ((f32::exp((0.333333343 * self.fConst109)) / self.fConst110) + -1.0);
        self.fConst119 = f32::floor(((0.0203460008 * self.fConst0) + 0.5));
        self.iConst120 =
            (f32::min(8192.0, f32::max(0.0, (self.fConst108 - self.fConst119))) as i32);
        self.iConst121 = (f32::min(1024.0, f32::max(0.0, (self.fConst119 + -1.0))) as i32);
        self.fConst122 = (0.0 - (f32::cos(self.fConst3) * (self.fConst4 + 1.0)));
        self.fConst123 = (0.0 - (f32::cos(self.fConst1) * (self.fConst2 + 1.0)));
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
        ui_interface.declare(Some(ParamIndex(1)), "tooltip", "Output scale   factor");
        ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
        ui_interface.add_vertical_slider(
            "Level",
            ParamIndex(1),
            -6.0,
            -70.0,
            40.0,
            0.10000000000000001,
        );
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        match param.0 {
            1 => Some(self.fVslider0),
            0 => Some(self.fVslider1),
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        match param.0 {
            1 => self.fVslider0 = value,
            0 => self.fVslider1 = value,
            _ => {}
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        let inputs0 = &inputs[0][..count as usize];
        let inputs1 = &inputs[1][..count as usize];
        assert!(outputs.len() >= 2);
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = &mut outputs[0..2] {
            let outputs0 = &mut outputs0[..count as usize];
            let outputs1 = &mut outputs1[..count as usize];
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };

        let mut fSlow0: f32 =
            (0.00100000005 * f32::powf(10.0, (0.0500000007 * (self.fVslider0 as f32))));
        let mut fSlow1: f32 = (0.00100000005 * (self.fVslider1 as f32));
        for (((input0, input1), output0), output1) in inputs0.iter().zip(inputs1.iter()).zip(outputs0.iter_mut()).zip(outputs1.iter_mut()) {
            self.fRec0[0] = (fSlow0 + (0.999000013 * self.fRec0[1]));
            let mut fTemp0: f32 = (*input0 as f32);
            self.fVec0[(self.IOTA & 16383) as usize] = fTemp0;
            self.fRec1[0] = (fSlow1 + (0.999000013 * self.fRec1[1]));
            let mut fTemp1: f32 = (self.fRec1[0] + 1.0);
            let mut fTemp2: f32 = (1.0 - (0.5 * fTemp1));
            self.fRec15[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec15[1]) - (self.fRec11[1] + self.fRec11[2]))));
            self.fRec14[0] = ((self.fConst14 * self.fRec14[1])
                + (self.fConst15 * (self.fRec11[1] + (self.fConst16 * self.fRec15[0]))));
            self.fVec1[(self.IOTA & 32767) as usize] =
                ((0.353553385 * self.fRec14[0]) + 9.99999968e-21);
            let mut fTemp3: f32 = (*input1 as f32);
            self.fVec2[(self.IOTA & 16383) as usize] = fTemp3;
            let mut fTemp4: f32 =
                (0.300000012 * self.fVec2[((self.IOTA - self.iConst22) & 16383) as usize]);
            let mut fTemp5: f32 = (((0.600000024 * self.fRec12[1])
                + self.fVec1[((self.IOTA - self.iConst21) & 32767) as usize])
                - fTemp4);
            self.fVec3[(self.IOTA & 2047) as usize] = fTemp5;
            self.fRec12[0] = self.fVec3[((self.IOTA - self.iConst23) & 2047) as usize];
            let mut fRec13: f32 = (0.0 - (0.600000024 * fTemp5));
            self.fRec19[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec19[1]) - (self.fRec7[1] + self.fRec7[2]))));
            self.fRec18[0] = ((self.fConst32 * self.fRec18[1])
                + (self.fConst33 * (self.fRec7[1] + (self.fConst34 * self.fRec19[0]))));
            self.fVec4[(self.IOTA & 32767) as usize] =
                ((0.353553385 * self.fRec18[0]) + 9.99999968e-21);
            let mut fTemp6: f32 = (((0.600000024 * self.fRec16[1])
                + self.fVec4[((self.IOTA - self.iConst36) & 32767) as usize])
                - fTemp4);
            self.fVec5[(self.IOTA & 4095) as usize] = fTemp6;
            self.fRec16[0] = self.fVec5[((self.IOTA - self.iConst37) & 4095) as usize];
            let mut fRec17: f32 = (0.0 - (0.600000024 * fTemp6));
            self.fRec23[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec23[1]) - (self.fRec9[1] + self.fRec9[2]))));
            self.fRec22[0] = ((self.fConst46 * self.fRec22[1])
                + (self.fConst47 * (self.fRec9[1] + (self.fConst48 * self.fRec23[0]))));
            self.fVec6[(self.IOTA & 16383) as usize] =
                ((0.353553385 * self.fRec22[0]) + 9.99999968e-21);
            let mut fTemp7: f32 = (self.fVec6[((self.IOTA - self.iConst50) & 16383) as usize]
                + (fTemp4 + (0.600000024 * self.fRec20[1])));
            self.fVec7[(self.IOTA & 4095) as usize] = fTemp7;
            self.fRec20[0] = self.fVec7[((self.IOTA - self.iConst51) & 4095) as usize];
            let mut fRec21: f32 = (0.0 - (0.600000024 * fTemp7));
            self.fRec27[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec27[1]) - (self.fRec5[1] + self.fRec5[2]))));
            self.fRec26[0] = ((self.fConst60 * self.fRec26[1])
                + (self.fConst61 * (self.fRec5[1] + (self.fConst62 * self.fRec27[0]))));
            self.fVec8[(self.IOTA & 32767) as usize] =
                ((0.353553385 * self.fRec26[0]) + 9.99999968e-21);
            let mut fTemp8: f32 = (fTemp4
                + ((0.600000024 * self.fRec24[1])
                    + self.fVec8[((self.IOTA - self.iConst64) & 32767) as usize]));
            self.fVec9[(self.IOTA & 4095) as usize] = fTemp8;
            self.fRec24[0] = self.fVec9[((self.IOTA - self.iConst65) & 4095) as usize];
            let mut fRec25: f32 = (0.0 - (0.600000024 * fTemp8));
            self.fRec31[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec31[1]) - (self.fRec10[1] + self.fRec10[2]))));
            self.fRec30[0] = ((self.fConst74 * self.fRec30[1])
                + (self.fConst75 * (self.fRec10[1] + (self.fConst76 * self.fRec31[0]))));
            self.fVec10[(self.IOTA & 16383) as usize] =
                ((0.353553385 * self.fRec30[0]) + 9.99999968e-21);
            let mut fTemp9: f32 =
                (0.300000012 * self.fVec0[((self.IOTA - self.iConst22) & 16383) as usize]);
            let mut fTemp10: f32 = (self.fVec10[((self.IOTA - self.iConst78) & 16383) as usize]
                - (fTemp9 + (0.600000024 * self.fRec28[1])));
            self.fVec11[(self.IOTA & 2047) as usize] = fTemp10;
            self.fRec28[0] = self.fVec11[((self.IOTA - self.iConst79) & 2047) as usize];
            let mut fRec29: f32 = (0.600000024 * fTemp10);
            self.fRec35[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec35[1]) - (self.fRec6[1] + self.fRec6[2]))));
            self.fRec34[0] = ((self.fConst88 * self.fRec34[1])
                + (self.fConst89 * (self.fRec6[1] + (self.fConst90 * self.fRec35[0]))));
            self.fVec12[(self.IOTA & 16383) as usize] =
                ((0.353553385 * self.fRec34[0]) + 9.99999968e-21);
            let mut fTemp11: f32 = (self.fVec12[((self.IOTA - self.iConst92) & 16383) as usize]
                - (fTemp9 + (0.600000024 * self.fRec32[1])));
            self.fVec13[(self.IOTA & 4095) as usize] = fTemp11;
            self.fRec32[0] = self.fVec13[((self.IOTA - self.iConst93) & 4095) as usize];
            let mut fRec33: f32 = (0.600000024 * fTemp11);
            self.fRec39[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec39[1]) - (self.fRec8[1] + self.fRec8[2]))));
            self.fRec38[0] = ((self.fConst102 * self.fRec38[1])
                + (self.fConst103 * (self.fRec8[1] + (self.fConst104 * self.fRec39[0]))));
            self.fVec14[(self.IOTA & 16383) as usize] =
                ((0.353553385 * self.fRec38[0]) + 9.99999968e-21);
            let mut fTemp12: f32 = ((fTemp9
                + self.fVec14[((self.IOTA - self.iConst106) & 16383) as usize])
                - (0.600000024 * self.fRec36[1]));
            self.fVec15[(self.IOTA & 4095) as usize] = fTemp12;
            self.fRec36[0] = self.fVec15[((self.IOTA - self.iConst107) & 4095) as usize];
            let mut fRec37: f32 = (0.600000024 * fTemp12);
            self.fRec43[0] = (0.0
                - (self.fConst18
                    * ((self.fConst19 * self.fRec43[1]) - (self.fRec4[1] + self.fRec4[2]))));
            self.fRec42[0] = ((self.fConst116 * self.fRec42[1])
                + (self.fConst117 * (self.fRec4[1] + (self.fConst118 * self.fRec43[0]))));
            self.fVec16[(self.IOTA & 16383) as usize] =
                ((0.353553385 * self.fRec42[0]) + 9.99999968e-21);
            let mut fTemp13: f32 = ((self.fVec16[((self.IOTA - self.iConst120) & 16383) as usize]
                + fTemp9)
                - (0.600000024 * self.fRec40[1]));
            self.fVec17[(self.IOTA & 2047) as usize] = fTemp13;
            self.fRec40[0] = self.fVec17[((self.IOTA - self.iConst121) & 2047) as usize];
            let mut fRec41: f32 = (0.600000024 * fTemp13);
            let mut fTemp14: f32 = (fRec41 + fRec37);
            let mut fTemp15: f32 = (fRec29 + (fRec33 + fTemp14));
            self.fRec4[0] = (self.fRec12[1]
                + (self.fRec16[1]
                    + (self.fRec20[1]
                        + (self.fRec24[1]
                            + (self.fRec28[1]
                                + (self.fRec32[1]
                                    + (self.fRec36[1]
                                        + (self.fRec40[1]
                                            + (fRec13
                                                + (fRec17 + (fRec21 + (fRec25 + fTemp15))))))))))));
            self.fRec5[0] = ((self.fRec28[1]
                + (self.fRec32[1] + (self.fRec36[1] + (self.fRec40[1] + fTemp15))))
                - (self.fRec12[1]
                    + (self.fRec16[1]
                        + (self.fRec20[1]
                            + (self.fRec24[1] + (fRec13 + (fRec17 + (fRec25 + fRec21))))))));
            let mut fTemp16: f32 = (fRec33 + fRec29);
            self.fRec6[0] = ((self.fRec20[1]
                + (self.fRec24[1]
                    + (self.fRec36[1] + (self.fRec40[1] + (fRec21 + (fRec25 + fTemp14))))))
                - (self.fRec12[1]
                    + (self.fRec16[1]
                        + (self.fRec28[1] + (self.fRec32[1] + (fRec13 + (fRec17 + fTemp16)))))));
            self.fRec7[0] = ((self.fRec12[1]
                + (self.fRec16[1]
                    + (self.fRec36[1] + (self.fRec40[1] + (fRec13 + (fRec17 + fTemp14))))))
                - (self.fRec20[1]
                    + (self.fRec24[1]
                        + (self.fRec28[1] + (self.fRec32[1] + (fRec21 + (fRec25 + fTemp16)))))));
            let mut fTemp17: f32 = (fRec41 + fRec33);
            let mut fTemp18: f32 = (fRec37 + fRec29);
            self.fRec8[0] = ((self.fRec16[1]
                + (self.fRec24[1]
                    + (self.fRec32[1] + (self.fRec40[1] + (fRec17 + (fRec25 + fTemp17))))))
                - (self.fRec12[1]
                    + (self.fRec20[1]
                        + (self.fRec28[1] + (self.fRec36[1] + (fRec13 + (fRec21 + fTemp18)))))));
            self.fRec9[0] = ((self.fRec12[1]
                + (self.fRec20[1]
                    + (self.fRec32[1] + (self.fRec40[1] + (fRec13 + (fRec21 + fTemp17))))))
                - (self.fRec16[1]
                    + (self.fRec24[1]
                        + (self.fRec28[1] + (self.fRec36[1] + (fRec17 + (fRec25 + fTemp18)))))));
            let mut fTemp19: f32 = (fRec41 + fRec29);
            let mut fTemp20: f32 = (fRec37 + fRec33);
            self.fRec10[0] = ((self.fRec12[1]
                + (self.fRec24[1]
                    + (self.fRec28[1] + (self.fRec40[1] + (fRec13 + (fRec25 + fTemp19))))))
                - (self.fRec16[1]
                    + (self.fRec20[1]
                        + (self.fRec32[1] + (self.fRec36[1] + (fRec17 + (fRec21 + fTemp20)))))));
            self.fRec11[0] = ((self.fRec16[1]
                + (self.fRec20[1]
                    + (self.fRec28[1] + (self.fRec40[1] + (fRec17 + (fRec21 + fTemp19))))))
                - (self.fRec12[1]
                    + (self.fRec24[1]
                        + (self.fRec32[1] + (self.fRec36[1] + (fRec13 + (fRec25 + fTemp20)))))));
            let mut fTemp21: f32 = (0.370000005 * (self.fRec5[0] + self.fRec6[0]));
            let mut fTemp22: f32 = (self.fConst122 * self.fRec3[1]);
            self.fRec3[0] = (fTemp21 - (fTemp22 + (self.fConst4 * self.fRec3[2])));
            let mut fTemp23: f32 = (self.fConst4 * self.fRec3[0]);
            let mut fTemp24: f32 = (0.5
                * ((fTemp23 + (self.fRec3[2] + (fTemp21 + fTemp22)))
                    + ((fTemp23 + (fTemp22 + self.fRec3[2])) - fTemp21)));
            let mut fTemp25: f32 = (self.fConst123 * self.fRec2[1]);
            self.fRec2[0] = (fTemp24 - (fTemp25 + (self.fConst2 * self.fRec2[2])));
            let mut fTemp26: f32 = (self.fConst2 * self.fRec2[0]);
            *output0 = ((0.5
                * (self.fRec0[0]
                    * ((fTemp0 * fTemp1)
                        + (fTemp2
                            * ((fTemp26 + (self.fRec2[2] + (fTemp24 + fTemp25)))
                                + ((fTemp26 + (fTemp25 + self.fRec2[2])) - fTemp24))))))
                as f32);
            let mut fTemp27: f32 = (0.370000005 * (self.fRec5[0] - self.fRec6[0]));
            let mut fTemp28: f32 = (self.fConst122 * self.fRec45[1]);
            self.fRec45[0] = (fTemp27 - (fTemp28 + (self.fConst4 * self.fRec45[2])));
            let mut fTemp29: f32 = (self.fConst4 * self.fRec45[0]);
            let mut fTemp30: f32 = (0.5
                * ((fTemp29 + (self.fRec45[2] + (fTemp27 + fTemp28)))
                    + ((fTemp29 + (fTemp28 + self.fRec45[2])) - fTemp27)));
            let mut fTemp31: f32 = (self.fConst123 * self.fRec44[1]);
            self.fRec44[0] = (fTemp30 - (fTemp31 + (self.fConst2 * self.fRec44[2])));
            let mut fTemp32: f32 = (self.fConst2 * self.fRec44[0]);
            *output1 = ((0.5
                * (self.fRec0[0]
                    * ((fTemp3 * fTemp1)
                        + (fTemp2
                            * ((fTemp32 + (self.fRec44[2] + (fTemp30 + fTemp31)))
                                + ((fTemp32 + (fTemp31 + self.fRec44[2])) - fTemp30))))))
                as f32);
            self.fRec0[1] = self.fRec0[0];
            self.IOTA = (self.IOTA + 1);
            self.fRec1[1] = self.fRec1[0];
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
            self.fRec43[1] = self.fRec43[0];
            self.fRec42[1] = self.fRec42[0];
            self.fRec40[1] = self.fRec40[0];
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
            self.fRec3[2] = self.fRec3[1];
            self.fRec3[1] = self.fRec3[0];
            self.fRec2[2] = self.fRec2[1];
            self.fRec2[1] = self.fRec2[0];
            self.fRec45[2] = self.fRec45[1];
            self.fRec45[1] = self.fRec45[0];
            self.fRec44[2] = self.fRec44[1];
            self.fRec44[1] = self.fRec44[0];
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
