/* ------------------------------------------------------------
author: "Grame"
copyright: "(c)GRAME 2006"
license: "BSD"
name: "karplus32"
version: "1.0"
Code generated with Faust 2.75.12 (https://faust.grame.fr)
Compilation options: -a ./console-bench.cpp -lang cpp -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

#ifndef  __Dsp_H__
#define  __Dsp_H__


#include <algorithm>
#include <chrono>
#include <iomanip>
#include <iostream>
#include <fstream>
#include <memory>
#include <vector>

#include <libgen.h>

#include "faust/gui/UI.h"
#include "faust/dsp/dsp.h"  // or "faust/dsp/llvm-dsp.h" ?
#include "faust/misc.h"


#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif 

#include <algorithm>
#include <cmath>
#include <cstdint>

#ifndef FAUSTCLASS 
#define FAUSTCLASS Dsp
#endif

#ifdef __APPLE__ 
#define exp10f __exp10f
#define exp10 __exp10
#endif

#if defined(_WIN32)
#define RESTRICT __restrict
#else
#define RESTRICT __restrict__
#endif


class Dsp : public dsp {
	
 private:
	
	FAUSTFLOAT fHslider0;
	int iRec1[2];
	FAUSTFLOAT fHslider1;
	FAUSTFLOAT fButton0;
	float fVec0[2];
	float fRec2[2];
	FAUSTFLOAT fHslider2;
	int IOTA0;
	float fVec1[8192];
	FAUSTFLOAT fHslider3;
	FAUSTFLOAT fHslider4;
	float fRec0[3];
	FAUSTFLOAT fHslider5;
	float fVec2[8192];
	float fRec3[3];
	float fVec3[8192];
	float fRec4[3];
	float fVec4[8192];
	float fRec5[3];
	float fVec5[8192];
	float fRec6[3];
	float fVec6[8192];
	float fRec7[3];
	float fVec7[8192];
	float fRec8[3];
	float fVec8[8192];
	float fRec9[3];
	float fVec9[8192];
	float fRec10[3];
	float fVec10[8192];
	float fRec11[3];
	float fVec11[8192];
	float fRec12[3];
	float fVec12[8192];
	float fRec13[3];
	float fVec13[4096];
	float fRec14[3];
	float fVec14[4096];
	float fRec15[3];
	float fVec15[2048];
	float fRec16[3];
	float fVec16[512];
	float fRec17[3];
	FAUSTFLOAT fHslider6;
	float fVec17[8192];
	float fRec18[3];
	float fVec18[8192];
	float fRec19[3];
	float fVec19[8192];
	float fRec20[3];
	float fVec20[8192];
	float fRec21[3];
	float fVec21[8192];
	float fRec22[3];
	float fVec22[8192];
	float fRec23[3];
	float fVec23[8192];
	float fRec24[3];
	float fVec24[8192];
	float fRec25[3];
	float fVec25[8192];
	float fRec26[3];
	float fVec26[8192];
	float fRec27[3];
	float fVec27[8192];
	float fRec28[3];
	float fVec28[8192];
	float fRec29[3];
	float fVec29[4096];
	float fRec30[3];
	float fVec30[4096];
	float fRec31[3];
	float fVec31[2048];
	float fRec32[3];
	float fVec32[1024];
	float fRec33[3];
	int fSampleRate;
	
 public:
	Dsp() {
	}
	
	void metadata(Meta* m) { 
		m->declare("author", "Grame");
		m->declare("compile_options", "-a ./console-bench.cpp -lang cpp -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m->declare("copyright", "(c)GRAME 2006");
		m->declare("delays.lib/name", "Faust Delay Library");
		m->declare("delays.lib/version", "1.1.0");
		m->declare("filename", "karplus32.dsp");
		m->declare("license", "BSD");
		m->declare("name", "karplus32");
		m->declare("noises.lib/name", "Faust Noise Generator Library");
		m->declare("noises.lib/version", "1.4.1");
		m->declare("version", "1.0");
	}

	virtual int getNumInputs() {
		return 0;
	}
	virtual int getNumOutputs() {
		return 2;
	}
	
	static void classInit(int sample_rate) {
	}
	
	virtual void instanceConstants(int sample_rate) {
		fSampleRate = sample_rate;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = FAUSTFLOAT(0.1f);
		fHslider1 = FAUSTFLOAT(128.0f);
		fButton0 = FAUSTFLOAT(0.0f);
		fHslider2 = FAUSTFLOAT(0.5f);
		fHslider3 = FAUSTFLOAT(32.0f);
		fHslider4 = FAUSTFLOAT(128.0f);
		fHslider5 = FAUSTFLOAT(1.0f);
		fHslider6 = FAUSTFLOAT(0.5f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = l0 + 1) {
			iRec1[l0] = 0;
		}
		for (int l1 = 0; l1 < 2; l1 = l1 + 1) {
			fVec0[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 2; l2 = l2 + 1) {
			fRec2[l2] = 0.0f;
		}
		IOTA0 = 0;
		for (int l3 = 0; l3 < 8192; l3 = l3 + 1) {
			fVec1[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 3; l4 = l4 + 1) {
			fRec0[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 8192; l5 = l5 + 1) {
			fVec2[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 3; l6 = l6 + 1) {
			fRec3[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 8192; l7 = l7 + 1) {
			fVec3[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 3; l8 = l8 + 1) {
			fRec4[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 8192; l9 = l9 + 1) {
			fVec4[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 3; l10 = l10 + 1) {
			fRec5[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 8192; l11 = l11 + 1) {
			fVec5[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 3; l12 = l12 + 1) {
			fRec6[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 8192; l13 = l13 + 1) {
			fVec6[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 3; l14 = l14 + 1) {
			fRec7[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 8192; l15 = l15 + 1) {
			fVec7[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 3; l16 = l16 + 1) {
			fRec8[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 8192; l17 = l17 + 1) {
			fVec8[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 3; l18 = l18 + 1) {
			fRec9[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 8192; l19 = l19 + 1) {
			fVec9[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 3; l20 = l20 + 1) {
			fRec10[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 8192; l21 = l21 + 1) {
			fVec10[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 3; l22 = l22 + 1) {
			fRec11[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 8192; l23 = l23 + 1) {
			fVec11[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 3; l24 = l24 + 1) {
			fRec12[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 8192; l25 = l25 + 1) {
			fVec12[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 3; l26 = l26 + 1) {
			fRec13[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 4096; l27 = l27 + 1) {
			fVec13[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 3; l28 = l28 + 1) {
			fRec14[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 4096; l29 = l29 + 1) {
			fVec14[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 3; l30 = l30 + 1) {
			fRec15[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 2048; l31 = l31 + 1) {
			fVec15[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 3; l32 = l32 + 1) {
			fRec16[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 512; l33 = l33 + 1) {
			fVec16[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 3; l34 = l34 + 1) {
			fRec17[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 8192; l35 = l35 + 1) {
			fVec17[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 3; l36 = l36 + 1) {
			fRec18[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 8192; l37 = l37 + 1) {
			fVec18[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 3; l38 = l38 + 1) {
			fRec19[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 8192; l39 = l39 + 1) {
			fVec19[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 3; l40 = l40 + 1) {
			fRec20[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 8192; l41 = l41 + 1) {
			fVec20[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 3; l42 = l42 + 1) {
			fRec21[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 8192; l43 = l43 + 1) {
			fVec21[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 3; l44 = l44 + 1) {
			fRec22[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 8192; l45 = l45 + 1) {
			fVec22[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 3; l46 = l46 + 1) {
			fRec23[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 8192; l47 = l47 + 1) {
			fVec23[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 3; l48 = l48 + 1) {
			fRec24[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 8192; l49 = l49 + 1) {
			fVec24[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 3; l50 = l50 + 1) {
			fRec25[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 8192; l51 = l51 + 1) {
			fVec25[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 3; l52 = l52 + 1) {
			fRec26[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 8192; l53 = l53 + 1) {
			fVec26[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 3; l54 = l54 + 1) {
			fRec27[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 8192; l55 = l55 + 1) {
			fVec27[l55] = 0.0f;
		}
		for (int l56 = 0; l56 < 3; l56 = l56 + 1) {
			fRec28[l56] = 0.0f;
		}
		for (int l57 = 0; l57 < 8192; l57 = l57 + 1) {
			fVec28[l57] = 0.0f;
		}
		for (int l58 = 0; l58 < 3; l58 = l58 + 1) {
			fRec29[l58] = 0.0f;
		}
		for (int l59 = 0; l59 < 4096; l59 = l59 + 1) {
			fVec29[l59] = 0.0f;
		}
		for (int l60 = 0; l60 < 3; l60 = l60 + 1) {
			fRec30[l60] = 0.0f;
		}
		for (int l61 = 0; l61 < 4096; l61 = l61 + 1) {
			fVec30[l61] = 0.0f;
		}
		for (int l62 = 0; l62 < 3; l62 = l62 + 1) {
			fRec31[l62] = 0.0f;
		}
		for (int l63 = 0; l63 < 2048; l63 = l63 + 1) {
			fVec31[l63] = 0.0f;
		}
		for (int l64 = 0; l64 < 3; l64 = l64 + 1) {
			fRec32[l64] = 0.0f;
		}
		for (int l65 = 0; l65 < 1024; l65 = l65 + 1) {
			fVec32[l65] = 0.0f;
		}
		for (int l66 = 0; l66 < 3; l66 = l66 + 1) {
			fRec33[l66] = 0.0f;
		}
	}
	
	virtual void init(int sample_rate) {
		classInit(sample_rate);
		instanceInit(sample_rate);
	}
	
	virtual void instanceInit(int sample_rate) {
		instanceConstants(sample_rate);
		instanceResetUserInterface();
		instanceClear();
	}
	
	virtual Dsp* clone() {
		return new Dsp();
	}
	
	virtual int getSampleRate() {
		return fSampleRate;
	}
	
	virtual void buildUserInterface(UI* ui_interface) {
		ui_interface->openVerticalBox("karplus32");
		ui_interface->openVerticalBox("excitator");
		ui_interface->addHorizontalSlider("excitation (samples)", &fHslider1, FAUSTFLOAT(128.0f), FAUSTFLOAT(2.0f), FAUSTFLOAT(512.0f), FAUSTFLOAT(1.0f));
		ui_interface->addButton("play", &fButton0);
		ui_interface->closeBox();
		ui_interface->openVerticalBox("noise generator");
		ui_interface->addHorizontalSlider("level", &fHslider2, FAUSTFLOAT(0.5f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.1f));
		ui_interface->closeBox();
		ui_interface->addHorizontalSlider("output volume", &fHslider6, FAUSTFLOAT(0.5f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.1f));
		ui_interface->openVerticalBox("resonator x32");
		ui_interface->addHorizontalSlider("attenuation", &fHslider0, FAUSTFLOAT(0.1f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->addHorizontalSlider("detune", &fHslider3, FAUSTFLOAT(32.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(512.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("duration (samples)", &fHslider4, FAUSTFLOAT(128.0f), FAUSTFLOAT(2.0f), FAUSTFLOAT(512.0f), FAUSTFLOAT(1.0f));
		ui_interface->addHorizontalSlider("polyphony", &fHslider5, FAUSTFLOAT(1.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(32.0f), FAUSTFLOAT(1.0f));
		ui_interface->closeBox();
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = 0.5f * (1.0f - float(fHslider0));
		float fSlow1 = 1.0f / float(fHslider1);
		float fSlow2 = float(fButton0);
		float fSlow3 = 4.656613e-10f * float(fHslider2);
		float fSlow4 = float(fHslider3);
		float fSlow5 = float(fHslider4);
		int iSlow6 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 3e+01f * fSlow4 + -1.5f)));
		float fSlow7 = float(fHslider5);
		float fSlow8 = float(fSlow7 > 3e+01f);
		int iSlow9 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 28.0f * fSlow4 + -1.5f)));
		float fSlow10 = float(fSlow7 > 28.0f);
		int iSlow11 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 26.0f * fSlow4 + -1.5f)));
		float fSlow12 = float(fSlow7 > 26.0f);
		int iSlow13 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 24.0f * fSlow4 + -1.5f)));
		float fSlow14 = float(fSlow7 > 24.0f);
		int iSlow15 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 22.0f * fSlow4 + -1.5f)));
		float fSlow16 = float(fSlow7 > 22.0f);
		int iSlow17 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 2e+01f * fSlow4 + -1.5f)));
		float fSlow18 = float(fSlow7 > 2e+01f);
		int iSlow19 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 18.0f * fSlow4 + -1.5f)));
		float fSlow20 = float(fSlow7 > 18.0f);
		int iSlow21 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 16.0f * fSlow4 + -1.5f)));
		float fSlow22 = float(fSlow7 > 16.0f);
		int iSlow23 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 14.0f * fSlow4 + -1.5f)));
		float fSlow24 = float(fSlow7 > 14.0f);
		int iSlow25 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 12.0f * fSlow4 + -1.5f)));
		float fSlow26 = float(fSlow7 > 12.0f);
		int iSlow27 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 1e+01f * fSlow4 + -1.5f)));
		float fSlow28 = float(fSlow7 > 1e+01f);
		int iSlow29 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 8.0f * fSlow4 + -1.5f)));
		float fSlow30 = float(fSlow7 > 8.0f);
		int iSlow31 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 6.0f * fSlow4 + -1.5f)));
		float fSlow32 = float(fSlow7 > 6.0f);
		int iSlow33 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 4.0f * fSlow4 + -1.5f)));
		float fSlow34 = float(fSlow7 > 4.0f);
		int iSlow35 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 2.0f * fSlow4 + -1.5f)));
		float fSlow36 = float(fSlow7 > 2.0f);
		int iSlow37 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + -1.5f)));
		float fSlow38 = float(fSlow7 > 0.0f);
		float fSlow39 = float(fHslider6);
		int iSlow40 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 31.0f * fSlow4 + -1.5f)));
		float fSlow41 = float(fSlow7 > 31.0f);
		int iSlow42 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 29.0f * fSlow4 + -1.5f)));
		float fSlow43 = float(fSlow7 > 29.0f);
		int iSlow44 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 27.0f * fSlow4 + -1.5f)));
		float fSlow45 = float(fSlow7 > 27.0f);
		int iSlow46 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 25.0f * fSlow4 + -1.5f)));
		float fSlow47 = float(fSlow7 > 25.0f);
		int iSlow48 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 23.0f * fSlow4 + -1.5f)));
		float fSlow49 = float(fSlow7 > 23.0f);
		int iSlow50 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 21.0f * fSlow4 + -1.5f)));
		float fSlow51 = float(fSlow7 > 21.0f);
		int iSlow52 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 19.0f * fSlow4 + -1.5f)));
		float fSlow53 = float(fSlow7 > 19.0f);
		int iSlow54 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 17.0f * fSlow4 + -1.5f)));
		float fSlow55 = float(fSlow7 > 17.0f);
		int iSlow56 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 15.0f * fSlow4 + -1.5f)));
		float fSlow57 = float(fSlow7 > 15.0f);
		int iSlow58 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 13.0f * fSlow4 + -1.5f)));
		float fSlow59 = float(fSlow7 > 13.0f);
		int iSlow60 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 11.0f * fSlow4 + -1.5f)));
		float fSlow61 = float(fSlow7 > 11.0f);
		int iSlow62 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 9.0f * fSlow4 + -1.5f)));
		float fSlow63 = float(fSlow7 > 9.0f);
		int iSlow64 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 7.0f * fSlow4 + -1.5f)));
		float fSlow65 = float(fSlow7 > 7.0f);
		int iSlow66 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 5.0f * fSlow4 + -1.5f)));
		float fSlow67 = float(fSlow7 > 5.0f);
		int iSlow68 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow5 + 3.0f * fSlow4 + -1.5f)));
		float fSlow69 = float(fSlow7 > 3.0f);
		int iSlow70 = int(std::min<float>(4096.0f, std::max<float>(0.0f, fSlow4 + fSlow5 + -1.5f)));
		float fSlow71 = float(fSlow7 > 1.0f);
		for (int i0 = 0; i0 < count; i0 = i0 + 1) {
			iRec1[0] = 1103515245 * iRec1[1] + 12345;
			fVec0[0] = fSlow2;
			fRec2[0] = fRec2[1] + float((fSlow2 - fVec0[1]) > 0.0f) - fSlow1 * float(fRec2[1] > 0.0f);
			float fTemp0 = fSlow3 * (float(fRec2[0] > 0.0f) + 1.5258789e-05f) * float(iRec1[0]);
			fVec1[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec0[1] + fRec0[2]);
			fRec0[0] = fVec1[(IOTA0 - iSlow6) & 8191];
			fVec2[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec3[1] + fRec3[2]);
			fRec3[0] = fVec2[(IOTA0 - iSlow9) & 8191];
			fVec3[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec4[1] + fRec4[2]);
			fRec4[0] = fVec3[(IOTA0 - iSlow11) & 8191];
			fVec4[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec5[2] + fRec5[1]);
			fRec5[0] = fVec4[(IOTA0 - iSlow13) & 8191];
			fVec5[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec6[1] + fRec6[2]);
			fRec6[0] = fVec5[(IOTA0 - iSlow15) & 8191];
			fVec6[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec7[1] + fRec7[2]);
			fRec7[0] = fVec6[(IOTA0 - iSlow17) & 8191];
			fVec7[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec8[1] + fRec8[2]);
			fRec8[0] = fVec7[(IOTA0 - iSlow19) & 8191];
			fVec8[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec9[1] + fRec9[2]);
			fRec9[0] = fVec8[(IOTA0 - iSlow21) & 8191];
			fVec9[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec10[1] + fRec10[2]);
			fRec10[0] = fVec9[(IOTA0 - iSlow23) & 8191];
			fVec10[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec11[1] + fRec11[2]);
			fRec11[0] = fVec10[(IOTA0 - iSlow25) & 8191];
			fVec11[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec12[1] + fRec12[2]);
			fRec12[0] = fVec11[(IOTA0 - iSlow27) & 8191];
			fVec12[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec13[1] + fRec13[2]);
			fRec13[0] = fVec12[(IOTA0 - iSlow29) & 8191];
			fVec13[IOTA0 & 4095] = fTemp0 + fSlow0 * (fRec14[1] + fRec14[2]);
			fRec14[0] = fVec13[(IOTA0 - iSlow31) & 4095];
			fVec14[IOTA0 & 4095] = fTemp0 + fSlow0 * (fRec15[1] + fRec15[2]);
			fRec15[0] = fVec14[(IOTA0 - iSlow33) & 4095];
			fVec15[IOTA0 & 2047] = fTemp0 + fSlow0 * (fRec16[1] + fRec16[2]);
			fRec16[0] = fVec15[(IOTA0 - iSlow35) & 2047];
			fVec16[IOTA0 & 511] = fSlow0 * (fRec17[1] + fRec17[2]) + fTemp0;
			fRec17[0] = fVec16[(IOTA0 - iSlow37) & 511];
			output0[i0] = FAUSTFLOAT(fSlow39 * (fSlow38 * fRec17[0] + fSlow36 * fRec16[0] + fSlow34 * fRec15[0] + fSlow32 * fRec14[0] + fSlow30 * fRec13[0] + fSlow28 * fRec12[0] + fSlow26 * fRec11[0] + fSlow24 * fRec10[0] + fSlow22 * fRec9[0] + fSlow20 * fRec8[0] + fSlow18 * fRec7[0] + fSlow16 * fRec6[0] + fSlow14 * fRec5[0] + fSlow12 * fRec4[0] + fSlow10 * fRec3[0] + fSlow8 * fRec0[0]));
			fVec17[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec18[1] + fRec18[2]);
			fRec18[0] = fVec17[(IOTA0 - iSlow40) & 8191];
			fVec18[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec19[1] + fRec19[2]);
			fRec19[0] = fVec18[(IOTA0 - iSlow42) & 8191];
			fVec19[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec20[1] + fRec20[2]);
			fRec20[0] = fVec19[(IOTA0 - iSlow44) & 8191];
			fVec20[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec21[1] + fRec21[2]);
			fRec21[0] = fVec20[(IOTA0 - iSlow46) & 8191];
			fVec21[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec22[1] + fRec22[2]);
			fRec22[0] = fVec21[(IOTA0 - iSlow48) & 8191];
			fVec22[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec23[1] + fRec23[2]);
			fRec23[0] = fVec22[(IOTA0 - iSlow50) & 8191];
			fVec23[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec24[1] + fRec24[2]);
			fRec24[0] = fVec23[(IOTA0 - iSlow52) & 8191];
			fVec24[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec25[2] + fRec25[1]);
			fRec25[0] = fVec24[(IOTA0 - iSlow54) & 8191];
			fVec25[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec26[1] + fRec26[2]);
			fRec26[0] = fVec25[(IOTA0 - iSlow56) & 8191];
			fVec26[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec27[1] + fRec27[2]);
			fRec27[0] = fVec26[(IOTA0 - iSlow58) & 8191];
			fVec27[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec28[1] + fRec28[2]);
			fRec28[0] = fVec27[(IOTA0 - iSlow60) & 8191];
			fVec28[IOTA0 & 8191] = fTemp0 + fSlow0 * (fRec29[1] + fRec29[2]);
			fRec29[0] = fVec28[(IOTA0 - iSlow62) & 8191];
			fVec29[IOTA0 & 4095] = fTemp0 + fSlow0 * (fRec30[1] + fRec30[2]);
			fRec30[0] = fVec29[(IOTA0 - iSlow64) & 4095];
			fVec30[IOTA0 & 4095] = fTemp0 + fSlow0 * (fRec31[1] + fRec31[2]);
			fRec31[0] = fVec30[(IOTA0 - iSlow66) & 4095];
			fVec31[IOTA0 & 2047] = fTemp0 + fSlow0 * (fRec32[1] + fRec32[2]);
			fRec32[0] = fVec31[(IOTA0 - iSlow68) & 2047];
			fVec32[IOTA0 & 1023] = fTemp0 + fSlow0 * (fRec33[1] + fRec33[2]);
			fRec33[0] = fVec32[(IOTA0 - iSlow70) & 1023];
			output1[i0] = FAUSTFLOAT(fSlow39 * (fSlow71 * fRec33[0] + fSlow69 * fRec32[0] + fSlow67 * fRec31[0] + fSlow65 * fRec30[0] + fSlow63 * fRec29[0] + fSlow61 * fRec28[0] + fSlow59 * fRec27[0] + fSlow57 * fRec26[0] + fSlow55 * fRec25[0] + fSlow53 * fRec24[0] + fSlow51 * fRec23[0] + fSlow49 * fRec22[0] + fSlow47 * fRec21[0] + fSlow45 * fRec20[0] + fSlow43 * fRec19[0] + fSlow41 * fRec18[0]));
			iRec1[1] = iRec1[0];
			fVec0[1] = fVec0[0];
			fRec2[1] = fRec2[0];
			IOTA0 = IOTA0 + 1;
			fRec0[2] = fRec0[1];
			fRec0[1] = fRec0[0];
			fRec3[2] = fRec3[1];
			fRec3[1] = fRec3[0];
			fRec4[2] = fRec4[1];
			fRec4[1] = fRec4[0];
			fRec5[2] = fRec5[1];
			fRec5[1] = fRec5[0];
			fRec6[2] = fRec6[1];
			fRec6[1] = fRec6[0];
			fRec7[2] = fRec7[1];
			fRec7[1] = fRec7[0];
			fRec8[2] = fRec8[1];
			fRec8[1] = fRec8[0];
			fRec9[2] = fRec9[1];
			fRec9[1] = fRec9[0];
			fRec10[2] = fRec10[1];
			fRec10[1] = fRec10[0];
			fRec11[2] = fRec11[1];
			fRec11[1] = fRec11[0];
			fRec12[2] = fRec12[1];
			fRec12[1] = fRec12[0];
			fRec13[2] = fRec13[1];
			fRec13[1] = fRec13[0];
			fRec14[2] = fRec14[1];
			fRec14[1] = fRec14[0];
			fRec15[2] = fRec15[1];
			fRec15[1] = fRec15[0];
			fRec16[2] = fRec16[1];
			fRec16[1] = fRec16[0];
			fRec17[2] = fRec17[1];
			fRec17[1] = fRec17[0];
			fRec18[2] = fRec18[1];
			fRec18[1] = fRec18[0];
			fRec19[2] = fRec19[1];
			fRec19[1] = fRec19[0];
			fRec20[2] = fRec20[1];
			fRec20[1] = fRec20[0];
			fRec21[2] = fRec21[1];
			fRec21[1] = fRec21[0];
			fRec22[2] = fRec22[1];
			fRec22[1] = fRec22[0];
			fRec23[2] = fRec23[1];
			fRec23[1] = fRec23[0];
			fRec24[2] = fRec24[1];
			fRec24[1] = fRec24[0];
			fRec25[2] = fRec25[1];
			fRec25[1] = fRec25[0];
			fRec26[2] = fRec26[1];
			fRec26[1] = fRec26[0];
			fRec27[2] = fRec27[1];
			fRec27[1] = fRec27[0];
			fRec28[2] = fRec28[1];
			fRec28[1] = fRec28[0];
			fRec29[2] = fRec29[1];
			fRec29[1] = fRec29[0];
			fRec30[2] = fRec30[1];
			fRec30[1] = fRec30[0];
			fRec31[2] = fRec31[1];
			fRec31[1] = fRec31[0];
			fRec32[2] = fRec32[1];
			fRec32[1] = fRec32[0];
			fRec33[2] = fRec33[1];
			fRec33[1] = fRec33[0];
		}
	}

};

//-------------------------------------------------------------------------
// 									MAIN
//-------------------------------------------------------------------------

int main(int argc, char *argv[])
{
    if (argc != 2) {
        throw std::runtime_error("Wrong number of arguments");
    }
    std::ofstream result_file(argv[1], std::ios::out);
    result_file << "[";

    int buffer_size = 1024;
    int sample_rate = 44100;
    int min_samples = sample_rate * 60 * 3;

    std::vector<double> throughputs;

    for (int i = 0; i < 10; ++i) {
        auto dsp = std::make_unique<Dsp>();
        dsp->init(sample_rate);

        int num_inputs = dsp->getNumInputs();
        int num_outputs = dsp->getNumOutputs();

        // Prepare buffers
        float** in_buffer = new float* [num_inputs];
        for (int i = 0; i < num_inputs; ++i) {
            in_buffer[i] = new float[buffer_size];
        }
        float** out_buffer = new float* [num_outputs];
        for (int i = 0; i < num_outputs; ++i) {
            out_buffer[i] = new float[buffer_size];
        }

        // Set input buffers to fixed impulse responses
        for (int c = 0; c < num_inputs; ++c) {
            for (int j = 0; j < buffer_size; ++j) {
                in_buffer[c][j] = (j == 0 ? 1.0 : 0.0);
            }
        }

        // Compute
        float sample_sum = 0.0;
        int num_samples_written = 0;

        std::chrono::steady_clock::time_point t1 = std::chrono::steady_clock::now();
        while (num_samples_written < min_samples) {
            dsp->compute(buffer_size, in_buffer, out_buffer);

            // Lightweight result access to prevent over-optimizations
            for (int c = 0; c < num_outputs; ++c) {
                sample_sum += out_buffer[c][0];
            }
            num_samples_written += buffer_size;
        }
        std::chrono::steady_clock::time_point t2 = std::chrono::steady_clock::now();

        auto elapsed_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(t2 - t1).count();
        auto elapsed = (double) elapsed_ns / 1e9;
        auto audio_length = (double) num_samples_written / (double) sample_rate;
        auto load = 100.0 * elapsed / audio_length;
        auto throughput = double(num_samples_written * 4 * num_outputs) / double(elapsed);

        throughputs.emplace_back(throughput);
        if (throughputs.size() > 1) {
            result_file << ", ";
        }
        result_file << throughput;

        std::cout <<
            "Rendered audio of length " << audio_length <<
            " sec in " << elapsed <<
            " sec [load: " << load << " %]    " <<
            throughput / 1024 / 1024 << " MB/sec" <<
            "    sample checksum: " << sample_sum << "\n";

        // Cleanup buffers
        for (int i = 0; i < num_inputs; ++i) {
            delete [] in_buffer[i];
        }
        delete [] in_buffer;
        for (int i = 0; i < num_outputs; ++i) {
            delete [] out_buffer[i];
        }
        delete [] out_buffer;
    }

    // print throughput stats
    double min = *std::min_element(std::begin(throughputs), std::end(throughputs));
    double max = *std::max_element(std::begin(throughputs), std::end(throughputs));

    size_t mid = throughputs.size() / 2;
    std::sort(throughputs.begin(), throughputs.end());
    double median = (
        throughputs.size() % 2 == 0 ?
        (throughputs[mid] + throughputs[mid + 1]) / 2 :
        throughputs[mid]
    );
    std::cout << std::fixed << std::setprecision(3);
    std::cout << "\n";
    std::cout << "Throughput min:    " << min / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput median: " << median / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput max:    " << max / 1024 / 1024 << " MB/sec" << std::endl;

    result_file << "]";
    return 0;
}

#endif
