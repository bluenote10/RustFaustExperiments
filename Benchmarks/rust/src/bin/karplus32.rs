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
    fHslider0: f32,
    fHslider1: f32,
    fHslider2: f32,
    fHslider3: f32,
    fButton0: f32,
    fVec0: [f32; 2],
    fHslider4: f32,
    fRec1: [f32; 2],
    iRec2: [i32; 2],
    IOTA: i32,
    fVec1: [f32; 512],
    fHslider5: f32,
    fRec0: [f32; 3],
    fVec2: [f32; 2048],
    fHslider6: f32,
    fRec3: [f32; 3],
    fVec3: [f32; 4096],
    fRec4: [f32; 3],
    fVec4: [f32; 4096],
    fRec5: [f32; 3],
    fVec5: [f32; 8192],
    fRec6: [f32; 3],
    fVec6: [f32; 8192],
    fRec7: [f32; 3],
    fVec7: [f32; 8192],
    fRec8: [f32; 3],
    fVec8: [f32; 8192],
    fRec9: [f32; 3],
    fVec9: [f32; 8192],
    fRec10: [f32; 3],
    fVec10: [f32; 8192],
    fRec11: [f32; 3],
    fVec11: [f32; 8192],
    fRec12: [f32; 3],
    fVec12: [f32; 8192],
    fRec13: [f32; 3],
    fVec13: [f32; 8192],
    fRec14: [f32; 3],
    fVec14: [f32; 8192],
    fRec15: [f32; 3],
    fVec15: [f32; 8192],
    fRec16: [f32; 3],
    fVec16: [f32; 8192],
    fRec17: [f32; 3],
    fVec17: [f32; 1024],
    fRec18: [f32; 3],
    fVec18: [f32; 2048],
    fRec19: [f32; 3],
    fVec19: [f32; 4096],
    fRec20: [f32; 3],
    fVec20: [f32; 4096],
    fRec21: [f32; 3],
    fVec21: [f32; 8192],
    fRec22: [f32; 3],
    fVec22: [f32; 8192],
    fRec23: [f32; 3],
    fVec23: [f32; 8192],
    fRec24: [f32; 3],
    fVec24: [f32; 8192],
    fRec25: [f32; 3],
    fVec25: [f32; 8192],
    fRec26: [f32; 3],
    fVec26: [f32; 8192],
    fRec27: [f32; 3],
    fVec27: [f32; 8192],
    fRec28: [f32; 3],
    fVec28: [f32; 8192],
    fRec29: [f32; 3],
    fVec29: [f32; 8192],
    fRec30: [f32; 3],
    fVec30: [f32; 8192],
    fRec31: [f32; 3],
    fVec31: [f32; 8192],
    fRec32: [f32; 3],
    fVec32: [f32; 8192],
    fRec33: [f32; 3],
    fSampleRate: i32,
}

impl FaustDsp for Dsp {
    type T = f32;

    fn new() -> Dsp {
        Dsp {
            fHslider0: 0.0,
            fHslider1: 0.0,
            fHslider2: 0.0,
            fHslider3: 0.0,
            fButton0: 0.0,
            fVec0: [0.0; 2],
            fHslider4: 0.0,
            fRec1: [0.0; 2],
            iRec2: [0; 2],
            IOTA: 0,
            fVec1: [0.0; 512],
            fHslider5: 0.0,
            fRec0: [0.0; 3],
            fVec2: [0.0; 2048],
            fHslider6: 0.0,
            fRec3: [0.0; 3],
            fVec3: [0.0; 4096],
            fRec4: [0.0; 3],
            fVec4: [0.0; 4096],
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
            fVec13: [0.0; 8192],
            fRec14: [0.0; 3],
            fVec14: [0.0; 8192],
            fRec15: [0.0; 3],
            fVec15: [0.0; 8192],
            fRec16: [0.0; 3],
            fVec16: [0.0; 8192],
            fRec17: [0.0; 3],
            fVec17: [0.0; 1024],
            fRec18: [0.0; 3],
            fVec18: [0.0; 2048],
            fRec19: [0.0; 3],
            fVec19: [0.0; 4096],
            fRec20: [0.0; 3],
            fVec20: [0.0; 4096],
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
            fVec29: [0.0; 8192],
            fRec30: [0.0; 3],
            fVec30: [0.0; 8192],
            fRec31: [0.0; 3],
            fVec31: [0.0; 8192],
            fRec32: [0.0; 3],
            fVec32: [0.0; 8192],
            fRec33: [0.0; 3],
            fSampleRate: 0,
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("author", "Grame");
        m.declare("copyright", "(c)GRAME 2006");
        m.declare("delays.lib/name", "Faust Delay Library");
        m.declare("delays.lib/version", "0.1");
        m.declare("filename", "karplus32.dsp");
        m.declare("license", "BSD");
        m.declare("name", "karplus32");
        m.declare("noises.lib/name", "Faust Noise Generator Library");
        m.declare("noises.lib/version", "0.0");
        m.declare("version", "1.0");
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
    fn get_input_rate(&self, channel: i32) -> i32 {
        let mut rate: i32;
        match (channel) {
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
        self.fHslider0 = 0.5;
        self.fHslider1 = 1.0;
        self.fHslider2 = 0.100000001;
        self.fHslider3 = 0.5;
        self.fButton0 = 0.0;
        self.fHslider4 = 128.0;
        self.fHslider5 = 128.0;
        self.fHslider6 = 32.0;
    }
    fn instance_clear(&mut self) {
        for l0 in 0..2 {
            self.fVec0[l0 as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec1[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.iRec2[l2 as usize] = 0;
        }
        self.IOTA = 0;
        for l3 in 0..512 {
            self.fVec1[l3 as usize] = 0.0;
        }
        for l4 in 0..3 {
            self.fRec0[l4 as usize] = 0.0;
        }
        for l5 in 0..2048 {
            self.fVec2[l5 as usize] = 0.0;
        }
        for l6 in 0..3 {
            self.fRec3[l6 as usize] = 0.0;
        }
        for l7 in 0..4096 {
            self.fVec3[l7 as usize] = 0.0;
        }
        for l8 in 0..3 {
            self.fRec4[l8 as usize] = 0.0;
        }
        for l9 in 0..4096 {
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
        for l27 in 0..8192 {
            self.fVec13[l27 as usize] = 0.0;
        }
        for l28 in 0..3 {
            self.fRec14[l28 as usize] = 0.0;
        }
        for l29 in 0..8192 {
            self.fVec14[l29 as usize] = 0.0;
        }
        for l30 in 0..3 {
            self.fRec15[l30 as usize] = 0.0;
        }
        for l31 in 0..8192 {
            self.fVec15[l31 as usize] = 0.0;
        }
        for l32 in 0..3 {
            self.fRec16[l32 as usize] = 0.0;
        }
        for l33 in 0..8192 {
            self.fVec16[l33 as usize] = 0.0;
        }
        for l34 in 0..3 {
            self.fRec17[l34 as usize] = 0.0;
        }
        for l35 in 0..1024 {
            self.fVec17[l35 as usize] = 0.0;
        }
        for l36 in 0..3 {
            self.fRec18[l36 as usize] = 0.0;
        }
        for l37 in 0..2048 {
            self.fVec18[l37 as usize] = 0.0;
        }
        for l38 in 0..3 {
            self.fRec19[l38 as usize] = 0.0;
        }
        for l39 in 0..4096 {
            self.fVec19[l39 as usize] = 0.0;
        }
        for l40 in 0..3 {
            self.fRec20[l40 as usize] = 0.0;
        }
        for l41 in 0..4096 {
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
        for l59 in 0..8192 {
            self.fVec29[l59 as usize] = 0.0;
        }
        for l60 in 0..3 {
            self.fRec30[l60 as usize] = 0.0;
        }
        for l61 in 0..8192 {
            self.fVec30[l61 as usize] = 0.0;
        }
        for l62 in 0..3 {
            self.fRec31[l62 as usize] = 0.0;
        }
        for l63 in 0..8192 {
            self.fVec31[l63 as usize] = 0.0;
        }
        for l64 in 0..3 {
            self.fRec32[l64 as usize] = 0.0;
        }
        for l65 in 0..8192 {
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
        ui_interface.add_horizontal_slider(
            "excitation (samples)",
            ParamIndex(0),
            128.0,
            2.0,
            512.0,
            1.0,
        );
        ui_interface.add_button("play", ParamIndex(1));
        ui_interface.close_box();
        ui_interface.open_vertical_box("noise generator");
        ui_interface.add_horizontal_slider(
            "level",
            ParamIndex(2),
            0.5,
            0.0,
            1.0,
            0.10000000000000001,
        );
        ui_interface.close_box();
        ui_interface.add_horizontal_slider(
            "output volume",
            ParamIndex(3),
            0.5,
            0.0,
            1.0,
            0.10000000000000001,
        );
        ui_interface.open_vertical_box("resonator x32");
        ui_interface.add_horizontal_slider(
            "attenuation",
            ParamIndex(4),
            0.10000000000000001,
            0.0,
            1.0,
            0.01,
        );
        ui_interface.add_horizontal_slider("detune", ParamIndex(5), 32.0, 0.0, 512.0, 1.0);
        ui_interface.add_horizontal_slider(
            "duration (samples)",
            ParamIndex(6),
            128.0,
            2.0,
            512.0,
            1.0,
        );
        ui_interface.add_horizontal_slider("polyphony", ParamIndex(7), 1.0, 0.0, 32.0, 1.0);
        ui_interface.close_box();
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        match param.0 {
            1 => Some(self.fButton0),
            3 => Some(self.fHslider0),
            7 => Some(self.fHslider1),
            4 => Some(self.fHslider2),
            2 => Some(self.fHslider3),
            0 => Some(self.fHslider4),
            6 => Some(self.fHslider5),
            5 => Some(self.fHslider6),
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        match param.0 {
            1 => self.fButton0 = value,
            3 => self.fHslider0 = value,
            7 => self.fHslider1 = value,
            4 => self.fHslider2 = value,
            2 => self.fHslider3 = value,
            0 => self.fHslider4 = value,
            6 => self.fHslider5 = value,
            5 => self.fHslider6 = value,
            _ => {}
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        let mut fSlow0: f32 = (self.fHslider0 as f32);
        let mut fSlow1: f32 = (self.fHslider1 as f32);
        let mut fSlow2: f32 = (((fSlow1 > 0.0) as i32) as f32);
        let mut fSlow3: f32 = (0.5 * (1.0 - (self.fHslider2 as f32)));
        let mut fSlow4: f32 = (4.65661287e-10 * (self.fHslider3 as f32));
        let mut fSlow5: f32 = (self.fButton0 as f32);
        let mut fSlow6: f32 = (1.0 / (self.fHslider4 as f32));
        let mut fSlow7: f32 = (self.fHslider5 as f32);
        let mut iSlow8: i32 = (f32::min(4096.0, f32::max(0.0, (fSlow7 + -1.5))) as i32);
        let mut fSlow9: f32 = (((fSlow1 > 2.0) as i32) as f32);
        let mut fSlow10: f32 = (self.fHslider6 as f32);
        let mut iSlow11: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (2.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow12: f32 = (((fSlow1 > 4.0) as i32) as f32);
        let mut iSlow13: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (4.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow14: f32 = (((fSlow1 > 6.0) as i32) as f32);
        let mut iSlow15: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (6.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow16: f32 = (((fSlow1 > 8.0) as i32) as f32);
        let mut iSlow17: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (8.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow18: f32 = (((fSlow1 > 10.0) as i32) as f32);
        let mut iSlow19: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (10.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow20: f32 = (((fSlow1 > 12.0) as i32) as f32);
        let mut iSlow21: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (12.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow22: f32 = (((fSlow1 > 14.0) as i32) as f32);
        let mut iSlow23: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (14.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow24: f32 = (((fSlow1 > 16.0) as i32) as f32);
        let mut iSlow25: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (16.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow26: f32 = (((fSlow1 > 18.0) as i32) as f32);
        let mut iSlow27: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (18.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow28: f32 = (((fSlow1 > 20.0) as i32) as f32);
        let mut iSlow29: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (20.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow30: f32 = (((fSlow1 > 22.0) as i32) as f32);
        let mut iSlow31: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (22.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow32: f32 = (((fSlow1 > 24.0) as i32) as f32);
        let mut iSlow33: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (24.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow34: f32 = (((fSlow1 > 26.0) as i32) as f32);
        let mut iSlow35: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (26.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow36: f32 = (((fSlow1 > 28.0) as i32) as f32);
        let mut iSlow37: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (28.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow38: f32 = (((fSlow1 > 30.0) as i32) as f32);
        let mut iSlow39: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (30.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow40: f32 = (((fSlow1 > 1.0) as i32) as f32);
        let mut iSlow41: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow10 + fSlow7) + -1.5))) as i32);
        let mut fSlow42: f32 = (((fSlow1 > 3.0) as i32) as f32);
        let mut iSlow43: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (3.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow44: f32 = (((fSlow1 > 5.0) as i32) as f32);
        let mut iSlow45: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (5.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow46: f32 = (((fSlow1 > 7.0) as i32) as f32);
        let mut iSlow47: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (7.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow48: f32 = (((fSlow1 > 9.0) as i32) as f32);
        let mut iSlow49: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (9.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow50: f32 = (((fSlow1 > 11.0) as i32) as f32);
        let mut iSlow51: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (11.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow52: f32 = (((fSlow1 > 13.0) as i32) as f32);
        let mut iSlow53: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (13.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow54: f32 = (((fSlow1 > 15.0) as i32) as f32);
        let mut iSlow55: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (15.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow56: f32 = (((fSlow1 > 17.0) as i32) as f32);
        let mut iSlow57: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (17.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow58: f32 = (((fSlow1 > 19.0) as i32) as f32);
        let mut iSlow59: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (19.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow60: f32 = (((fSlow1 > 21.0) as i32) as f32);
        let mut iSlow61: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (21.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow62: f32 = (((fSlow1 > 23.0) as i32) as f32);
        let mut iSlow63: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (23.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow64: f32 = (((fSlow1 > 25.0) as i32) as f32);
        let mut iSlow65: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (25.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow66: f32 = (((fSlow1 > 27.0) as i32) as f32);
        let mut iSlow67: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (27.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow68: f32 = (((fSlow1 > 29.0) as i32) as f32);
        let mut iSlow69: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (29.0 * fSlow10)) + -1.5))) as i32);
        let mut fSlow70: f32 = (((fSlow1 > 31.0) as i32) as f32);
        let mut iSlow71: i32 =
            (f32::min(4096.0, f32::max(0.0, ((fSlow7 + (31.0 * fSlow10)) + -1.5))) as i32);
        for i in 0..count {
            self.fVec0[0] = fSlow5;
            self.fRec1[0] = ((self.fRec1[1] + ((((fSlow5 - self.fVec0[1]) > 0.0) as i32) as f32))
                - (fSlow6 * (((self.fRec1[1] > 0.0) as i32) as f32)));
            self.iRec2[0] = ((1103515245 * self.iRec2[1]) + 12345);
            let mut fTemp0: f32 = (fSlow4
                * (((((self.fRec1[0] > 0.0) as i32) as f32) + 1.52587891e-05)
                    * (self.iRec2[0] as f32)));
            self.fVec1[(self.IOTA & 511) as usize] =
                ((fSlow3 * (self.fRec0[1] + self.fRec0[2])) + fTemp0);
            self.fRec0[0] = self.fVec1[((self.IOTA - iSlow8) & 511) as usize];
            self.fVec2[(self.IOTA & 2047) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec3[1] + self.fRec3[2])));
            self.fRec3[0] = self.fVec2[((self.IOTA - iSlow11) & 2047) as usize];
            self.fVec3[(self.IOTA & 4095) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec4[1] + self.fRec4[2])));
            self.fRec4[0] = self.fVec3[((self.IOTA - iSlow13) & 4095) as usize];
            self.fVec4[(self.IOTA & 4095) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec5[1] + self.fRec5[2])));
            self.fRec5[0] = self.fVec4[((self.IOTA - iSlow15) & 4095) as usize];
            self.fVec5[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec6[1] + self.fRec6[2])));
            self.fRec6[0] = self.fVec5[((self.IOTA - iSlow17) & 8191) as usize];
            self.fVec6[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec7[1] + self.fRec7[2])));
            self.fRec7[0] = self.fVec6[((self.IOTA - iSlow19) & 8191) as usize];
            self.fVec7[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec8[1] + self.fRec8[2])));
            self.fRec8[0] = self.fVec7[((self.IOTA - iSlow21) & 8191) as usize];
            self.fVec8[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec9[1] + self.fRec9[2])));
            self.fRec9[0] = self.fVec8[((self.IOTA - iSlow23) & 8191) as usize];
            self.fVec9[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec10[1] + self.fRec10[2])));
            self.fRec10[0] = self.fVec9[((self.IOTA - iSlow25) & 8191) as usize];
            self.fVec10[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec11[1] + self.fRec11[2])));
            self.fRec11[0] = self.fVec10[((self.IOTA - iSlow27) & 8191) as usize];
            self.fVec11[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec12[1] + self.fRec12[2])));
            self.fRec12[0] = self.fVec11[((self.IOTA - iSlow29) & 8191) as usize];
            self.fVec12[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec13[1] + self.fRec13[2])));
            self.fRec13[0] = self.fVec12[((self.IOTA - iSlow31) & 8191) as usize];
            self.fVec13[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec14[1] + self.fRec14[2])));
            self.fRec14[0] = self.fVec13[((self.IOTA - iSlow33) & 8191) as usize];
            self.fVec14[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec15[1] + self.fRec15[2])));
            self.fRec15[0] = self.fVec14[((self.IOTA - iSlow35) & 8191) as usize];
            self.fVec15[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec16[1] + self.fRec16[2])));
            self.fRec16[0] = self.fVec15[((self.IOTA - iSlow37) & 8191) as usize];
            self.fVec16[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec17[1] + self.fRec17[2])));
            self.fRec17[0] = self.fVec16[((self.IOTA - iSlow39) & 8191) as usize];
            outputs[0][i as usize] = ((fSlow0
                * ((((((((((((((((fSlow2 * self.fRec0[0]) + (fSlow9 * self.fRec3[0]))
                    + (fSlow12 * self.fRec4[0]))
                    + (fSlow14 * self.fRec5[0]))
                    + (fSlow16 * self.fRec6[0]))
                    + (fSlow18 * self.fRec7[0]))
                    + (fSlow20 * self.fRec8[0]))
                    + (fSlow22 * self.fRec9[0]))
                    + (fSlow24 * self.fRec10[0]))
                    + (fSlow26 * self.fRec11[0]))
                    + (fSlow28 * self.fRec12[0]))
                    + (fSlow30 * self.fRec13[0]))
                    + (fSlow32 * self.fRec14[0]))
                    + (fSlow34 * self.fRec15[0]))
                    + (fSlow36 * self.fRec16[0]))
                    + (fSlow38 * self.fRec17[0]))) as f32);
            self.fVec17[(self.IOTA & 1023) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec18[1] + self.fRec18[2])));
            self.fRec18[0] = self.fVec17[((self.IOTA - iSlow41) & 1023) as usize];
            self.fVec18[(self.IOTA & 2047) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec19[1] + self.fRec19[2])));
            self.fRec19[0] = self.fVec18[((self.IOTA - iSlow43) & 2047) as usize];
            self.fVec19[(self.IOTA & 4095) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec20[1] + self.fRec20[2])));
            self.fRec20[0] = self.fVec19[((self.IOTA - iSlow45) & 4095) as usize];
            self.fVec20[(self.IOTA & 4095) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec21[1] + self.fRec21[2])));
            self.fRec21[0] = self.fVec20[((self.IOTA - iSlow47) & 4095) as usize];
            self.fVec21[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec22[1] + self.fRec22[2])));
            self.fRec22[0] = self.fVec21[((self.IOTA - iSlow49) & 8191) as usize];
            self.fVec22[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec23[1] + self.fRec23[2])));
            self.fRec23[0] = self.fVec22[((self.IOTA - iSlow51) & 8191) as usize];
            self.fVec23[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec24[1] + self.fRec24[2])));
            self.fRec24[0] = self.fVec23[((self.IOTA - iSlow53) & 8191) as usize];
            self.fVec24[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec25[1] + self.fRec25[2])));
            self.fRec25[0] = self.fVec24[((self.IOTA - iSlow55) & 8191) as usize];
            self.fVec25[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec26[1] + self.fRec26[2])));
            self.fRec26[0] = self.fVec25[((self.IOTA - iSlow57) & 8191) as usize];
            self.fVec26[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec27[1] + self.fRec27[2])));
            self.fRec27[0] = self.fVec26[((self.IOTA - iSlow59) & 8191) as usize];
            self.fVec27[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec28[1] + self.fRec28[2])));
            self.fRec28[0] = self.fVec27[((self.IOTA - iSlow61) & 8191) as usize];
            self.fVec28[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec29[1] + self.fRec29[2])));
            self.fRec29[0] = self.fVec28[((self.IOTA - iSlow63) & 8191) as usize];
            self.fVec29[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec30[1] + self.fRec30[2])));
            self.fRec30[0] = self.fVec29[((self.IOTA - iSlow65) & 8191) as usize];
            self.fVec30[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec31[1] + self.fRec31[2])));
            self.fRec31[0] = self.fVec30[((self.IOTA - iSlow67) & 8191) as usize];
            self.fVec31[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec32[1] + self.fRec32[2])));
            self.fRec32[0] = self.fVec31[((self.IOTA - iSlow69) & 8191) as usize];
            self.fVec32[(self.IOTA & 8191) as usize] =
                (fTemp0 + (fSlow3 * (self.fRec33[1] + self.fRec33[2])));
            self.fRec33[0] = self.fVec32[((self.IOTA - iSlow71) & 8191) as usize];
            outputs[1][i as usize] = ((fSlow0
                * ((((((((((((((((fSlow40 * self.fRec18[0])
                    + (fSlow42 * self.fRec19[0]))
                    + (fSlow44 * self.fRec20[0]))
                    + (fSlow46 * self.fRec21[0]))
                    + (fSlow48 * self.fRec22[0]))
                    + (fSlow50 * self.fRec23[0]))
                    + (fSlow52 * self.fRec24[0]))
                    + (fSlow54 * self.fRec25[0]))
                    + (fSlow56 * self.fRec26[0]))
                    + (fSlow58 * self.fRec27[0]))
                    + (fSlow60 * self.fRec28[0]))
                    + (fSlow62 * self.fRec29[0]))
                    + (fSlow64 * self.fRec30[0]))
                    + (fSlow66 * self.fRec31[0]))
                    + (fSlow68 * self.fRec32[0]))
                    + (fSlow70 * self.fRec33[0]))) as f32);
            self.fVec0[1] = self.fVec0[0];
            self.fRec1[1] = self.fRec1[0];
            self.iRec2[1] = self.iRec2[0];
            self.IOTA = (self.IOTA + 1);
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
