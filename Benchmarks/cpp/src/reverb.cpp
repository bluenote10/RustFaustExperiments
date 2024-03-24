/* ------------------------------------------------------------
name: "reverb"
Code generated with Faust 2.72.11 (https://faust.grame.fr)
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
#include <math.h>

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

static float Dsp_faustpower2_f(float value) {
	return value * value;
}

class Dsp : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst2;
	float fConst3;
	float fConst5;
	float fConst6;
	float fConst16;
	float fConst18;
	float fConst19;
	float fRec13[2];
	float fConst20;
	float fConst21;
	float fRec12[2];
	int IOTA0;
	float fVec0[16384];
	int iConst23;
	float fVec1[16384];
	int iConst24;
	float fVec2[4096];
	int iConst25;
	float fRec10[2];
	float fConst34;
	float fRec17[2];
	float fConst35;
	float fConst36;
	float fRec16[2];
	float fVec3[16384];
	int iConst38;
	float fVec4[2048];
	int iConst39;
	float fRec14[2];
	float fConst48;
	float fRec21[2];
	float fConst49;
	float fConst50;
	float fRec20[2];
	float fVec5[16384];
	int iConst52;
	float fVec6[4096];
	int iConst53;
	float fRec18[2];
	float fConst62;
	float fRec25[2];
	float fConst63;
	float fConst64;
	float fRec24[2];
	float fVec7[16384];
	int iConst66;
	float fVec8[2048];
	int iConst67;
	float fRec22[2];
	float fConst76;
	float fRec29[2];
	float fConst77;
	float fConst78;
	float fRec28[2];
	float fVec9[32768];
	int iConst80;
	float fVec10[16384];
	float fVec11[4096];
	int iConst81;
	float fRec26[2];
	float fConst90;
	float fRec33[2];
	float fConst91;
	float fConst92;
	float fRec32[2];
	float fVec12[16384];
	int iConst94;
	float fVec13[4096];
	int iConst95;
	float fRec30[2];
	float fConst104;
	float fRec37[2];
	float fConst105;
	float fConst106;
	float fRec36[2];
	float fVec14[32768];
	int iConst108;
	float fVec15[4096];
	int iConst109;
	float fRec34[2];
	float fConst118;
	float fRec41[2];
	float fConst119;
	float fConst120;
	float fRec40[2];
	float fVec16[32768];
	int iConst122;
	float fVec17[2048];
	int iConst123;
	float fRec38[2];
	float fRec2[3];
	float fRec3[3];
	float fRec4[3];
	float fRec5[3];
	float fRec6[3];
	float fRec7[3];
	float fRec8[3];
	float fRec9[3];
	float fRec1[3];
	float fRec0[3];
	float fConst124;
	float fConst125;
	float fConst126;
	float fConst127;
	FAUSTFLOAT fVslider0;
	float fRec42[2];
	FAUSTFLOAT fVslider1;
	float fRec43[2];
	float fRec45[3];
	float fRec44[3];
	
 public:
	Dsp() {}

	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/tabulateNd", "Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m->declare("basics.lib/version", "1.15.0");
		m->declare("compile_options", "-a ./console-bench.cpp -lang cpp -ct 1 -cn Dsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m->declare("delays.lib/name", "Faust Delay Library");
		m->declare("delays.lib/version", "1.1.0");
		m->declare("demos.lib/name", "Faust Demos Library");
		m->declare("demos.lib/version", "1.1.1");
		m->declare("demos.lib/zita_light:author", "Julius O. Smith III");
		m->declare("demos.lib/zita_light:licence", "MIT");
		m->declare("filename", "reverb.dsp");
		m->declare("filters.lib/allpass_comb:author", "Julius O. Smith III");
		m->declare("filters.lib/allpass_comb:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/allpass_comb:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/fir:author", "Julius O. Smith III");
		m->declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/iir:author", "Julius O. Smith III");
		m->declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
		m->declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
		m->declare("filters.lib/lowpass:author", "Julius O. Smith III");
		m->declare("filters.lib/lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/lowpass:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/name", "Faust Filters Library");
		m->declare("filters.lib/peak_eq_rm:author", "Julius O. Smith III");
		m->declare("filters.lib/peak_eq_rm:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/peak_eq_rm:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/tf1:author", "Julius O. Smith III");
		m->declare("filters.lib/tf1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf1:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/tf1s:author", "Julius O. Smith III");
		m->declare("filters.lib/tf1s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf1s:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/tf2:author", "Julius O. Smith III");
		m->declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m->declare("filters.lib/version", "1.3.0");
		m->declare("maths.lib/author", "GRAME");
		m->declare("maths.lib/copyright", "GRAME");
		m->declare("maths.lib/license", "LGPL with exception");
		m->declare("maths.lib/name", "Faust Math Library");
		m->declare("maths.lib/version", "2.8.0");
		m->declare("name", "reverb");
		m->declare("platform.lib/name", "Generic Platform Library");
		m->declare("platform.lib/version", "1.3.0");
		m->declare("reverbs.lib/name", "Faust Reverb Library");
		m->declare("reverbs.lib/version", "1.2.1");
		m->declare("routes.lib/hadamard:author", "Remy Muller, revised by Romain Michon");
		m->declare("routes.lib/name", "Faust Signal Routing Library");
		m->declare("routes.lib/version", "1.2.0");
		m->declare("signals.lib/name", "Faust Signal Routing Library");
		m->declare("signals.lib/version", "1.5.0");
	}

	virtual int getNumInputs() {
		return 2;
	}
	virtual int getNumOutputs() {
		return 2;
	}
	
	static void classInit(int sample_rate) {
	}
	
	virtual void instanceConstants(int sample_rate) {
		fSampleRate = sample_rate;
		float fConst0 = std::min<float>(1.92e+05f, std::max<float>(1.0f, float(fSampleRate)));
		float fConst1 = 9424.778f / fConst0;
		fConst2 = (1.0f - fConst1) / (fConst1 + 1.0f);
		fConst3 = std::cos(fConst1) * (fConst2 + 1.0f);
		float fConst4 = 1979.2034f / fConst0;
		fConst5 = (1.0f - fConst4) / (fConst4 + 1.0f);
		fConst6 = std::cos(fConst4) * (fConst5 + 1.0f);
		float fConst7 = std::floor(0.174713f * fConst0 + 0.5f);
		float fConst8 = fConst7 / fConst0;
		float fConst9 = std::exp(-(3.4538777f * fConst8));
		float fConst10 = Dsp_faustpower2_f(fConst9);
		float fConst11 = 1.0f - fConst10;
		float fConst12 = std::cos(37699.113f / fConst0);
		float fConst13 = 1.0f - fConst12 * fConst10;
		float fConst14 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst13) / Dsp_faustpower2_f(fConst11) + -1.0f));
		float fConst15 = fConst13 / fConst11;
		fConst16 = fConst15 - fConst14;
		float fConst17 = 1.0f / std::tan(628.31854f / fConst0);
		fConst18 = 1.0f - fConst17;
		fConst19 = 1.0f / (fConst17 + 1.0f);
		fConst20 = std::exp(-(2.3025851f * fConst8)) / fConst9 + -1.0f;
		fConst21 = fConst9 * (fConst14 + (1.0f - fConst15));
		float fConst22 = std::floor(0.022904f * fConst0 + 0.5f);
		iConst23 = int(std::min<float>(8192.0f, std::max<float>(0.0f, fConst7 - fConst22)));
		iConst24 = int(std::min<float>(8192.0f, std::max<float>(0.0f, 0.06f * fConst0)));
		iConst25 = int(std::min<float>(2048.0f, std::max<float>(0.0f, fConst22 + -1.0f)));
		float fConst26 = std::floor(0.153129f * fConst0 + 0.5f);
		float fConst27 = fConst26 / fConst0;
		float fConst28 = std::exp(-(3.4538777f * fConst27));
		float fConst29 = Dsp_faustpower2_f(fConst28);
		float fConst30 = 1.0f - fConst29;
		float fConst31 = 1.0f - fConst12 * fConst29;
		float fConst32 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst31) / Dsp_faustpower2_f(fConst30) + -1.0f));
		float fConst33 = fConst31 / fConst30;
		fConst34 = fConst33 - fConst32;
		fConst35 = std::exp(-(2.3025851f * fConst27)) / fConst28 + -1.0f;
		fConst36 = fConst28 * (fConst32 + (1.0f - fConst33));
		float fConst37 = std::floor(0.020346f * fConst0 + 0.5f);
		iConst38 = int(std::min<float>(8192.0f, std::max<float>(0.0f, fConst26 - fConst37)));
		iConst39 = int(std::min<float>(1024.0f, std::max<float>(0.0f, fConst37 + -1.0f)));
		float fConst40 = std::floor(0.127837f * fConst0 + 0.5f);
		float fConst41 = fConst40 / fConst0;
		float fConst42 = std::exp(-(3.4538777f * fConst41));
		float fConst43 = Dsp_faustpower2_f(fConst42);
		float fConst44 = 1.0f - fConst43;
		float fConst45 = 1.0f - fConst12 * fConst43;
		float fConst46 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst45) / Dsp_faustpower2_f(fConst44) + -1.0f));
		float fConst47 = fConst45 / fConst44;
		fConst48 = fConst47 - fConst46;
		fConst49 = std::exp(-(2.3025851f * fConst41)) / fConst42 + -1.0f;
		fConst50 = fConst42 * (fConst46 + (1.0f - fConst47));
		float fConst51 = std::floor(0.031604f * fConst0 + 0.5f);
		iConst52 = int(std::min<float>(8192.0f, std::max<float>(0.0f, fConst40 - fConst51)));
		iConst53 = int(std::min<float>(2048.0f, std::max<float>(0.0f, fConst51 + -1.0f)));
		float fConst54 = std::floor(0.125f * fConst0 + 0.5f);
		float fConst55 = fConst54 / fConst0;
		float fConst56 = std::exp(-(3.4538777f * fConst55));
		float fConst57 = Dsp_faustpower2_f(fConst56);
		float fConst58 = 1.0f - fConst57;
		float fConst59 = 1.0f - fConst12 * fConst57;
		float fConst60 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst59) / Dsp_faustpower2_f(fConst58) + -1.0f));
		float fConst61 = fConst59 / fConst58;
		fConst62 = fConst61 - fConst60;
		fConst63 = std::exp(-(2.3025851f * fConst55)) / fConst56 + -1.0f;
		fConst64 = fConst56 * (fConst60 + (1.0f - fConst61));
		float fConst65 = std::floor(0.013458f * fConst0 + 0.5f);
		iConst66 = int(std::min<float>(8192.0f, std::max<float>(0.0f, fConst54 - fConst65)));
		iConst67 = int(std::min<float>(1024.0f, std::max<float>(0.0f, fConst65 + -1.0f)));
		float fConst68 = std::floor(0.210389f * fConst0 + 0.5f);
		float fConst69 = fConst68 / fConst0;
		float fConst70 = std::exp(-(3.4538777f * fConst69));
		float fConst71 = Dsp_faustpower2_f(fConst70);
		float fConst72 = 1.0f - fConst71;
		float fConst73 = 1.0f - fConst12 * fConst71;
		float fConst74 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst73) / Dsp_faustpower2_f(fConst72) + -1.0f));
		float fConst75 = fConst73 / fConst72;
		fConst76 = fConst75 - fConst74;
		fConst77 = std::exp(-(2.3025851f * fConst69)) / fConst70 + -1.0f;
		fConst78 = fConst70 * (fConst74 + (1.0f - fConst75));
		float fConst79 = std::floor(0.024421f * fConst0 + 0.5f);
		iConst80 = int(std::min<float>(16384.0f, std::max<float>(0.0f, fConst68 - fConst79)));
		iConst81 = int(std::min<float>(2048.0f, std::max<float>(0.0f, fConst79 + -1.0f)));
		float fConst82 = std::floor(0.192303f * fConst0 + 0.5f);
		float fConst83 = fConst82 / fConst0;
		float fConst84 = std::exp(-(3.4538777f * fConst83));
		float fConst85 = Dsp_faustpower2_f(fConst84);
		float fConst86 = 1.0f - fConst85;
		float fConst87 = 1.0f - fConst12 * fConst85;
		float fConst88 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst87) / Dsp_faustpower2_f(fConst86) + -1.0f));
		float fConst89 = fConst87 / fConst86;
		fConst90 = fConst89 - fConst88;
		fConst91 = std::exp(-(2.3025851f * fConst83)) / fConst84 + -1.0f;
		fConst92 = fConst84 * (fConst88 + (1.0f - fConst89));
		float fConst93 = std::floor(0.029291f * fConst0 + 0.5f);
		iConst94 = int(std::min<float>(8192.0f, std::max<float>(0.0f, fConst82 - fConst93)));
		iConst95 = int(std::min<float>(2048.0f, std::max<float>(0.0f, fConst93 + -1.0f)));
		float fConst96 = std::floor(0.256891f * fConst0 + 0.5f);
		float fConst97 = fConst96 / fConst0;
		float fConst98 = std::exp(-(3.4538777f * fConst97));
		float fConst99 = Dsp_faustpower2_f(fConst98);
		float fConst100 = 1.0f - fConst99;
		float fConst101 = 1.0f - fConst12 * fConst99;
		float fConst102 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst101) / Dsp_faustpower2_f(fConst100) + -1.0f));
		float fConst103 = fConst101 / fConst100;
		fConst104 = fConst103 - fConst102;
		fConst105 = std::exp(-(2.3025851f * fConst97)) / fConst98 + -1.0f;
		fConst106 = fConst98 * (fConst102 + (1.0f - fConst103));
		float fConst107 = std::floor(0.027333f * fConst0 + 0.5f);
		iConst108 = int(std::min<float>(16384.0f, std::max<float>(0.0f, fConst96 - fConst107)));
		iConst109 = int(std::min<float>(2048.0f, std::max<float>(0.0f, fConst107 + -1.0f)));
		float fConst110 = std::floor(0.219991f * fConst0 + 0.5f);
		float fConst111 = fConst110 / fConst0;
		float fConst112 = std::exp(-(3.4538777f * fConst111));
		float fConst113 = Dsp_faustpower2_f(fConst112);
		float fConst114 = 1.0f - fConst113;
		float fConst115 = 1.0f - fConst12 * fConst113;
		float fConst116 = std::sqrt(std::max<float>(0.0f, Dsp_faustpower2_f(fConst115) / Dsp_faustpower2_f(fConst114) + -1.0f));
		float fConst117 = fConst115 / fConst114;
		fConst118 = fConst117 - fConst116;
		fConst119 = std::exp(-(2.3025851f * fConst111)) / fConst112 + -1.0f;
		fConst120 = fConst112 * (fConst116 + (1.0f - fConst117));
		float fConst121 = std::floor(0.019123f * fConst0 + 0.5f);
		iConst122 = int(std::min<float>(16384.0f, std::max<float>(0.0f, fConst110 - fConst121)));
		iConst123 = int(std::min<float>(1024.0f, std::max<float>(0.0f, fConst121 + -1.0f)));
		fConst124 = 2.0f * fConst3;
		fConst125 = 2.0f * fConst2;
		fConst126 = 44.1f / fConst0;
		fConst127 = 1.0f - fConst126;
	}
	
	virtual void instanceResetUserInterface() {
		fVslider0 = FAUSTFLOAT(0.0f);
		fVslider1 = FAUSTFLOAT(-6.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = l0 + 1) {
			fRec13[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 2; l1 = l1 + 1) {
			fRec12[l1] = 0.0f;
		}
		IOTA0 = 0;
		for (int l2 = 0; l2 < 16384; l2 = l2 + 1) {
			fVec0[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 16384; l3 = l3 + 1) {
			fVec1[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 4096; l4 = l4 + 1) {
			fVec2[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 2; l5 = l5 + 1) {
			fRec10[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 2; l6 = l6 + 1) {
			fRec17[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = l7 + 1) {
			fRec16[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 16384; l8 = l8 + 1) {
			fVec3[l8] = 0.0f;
		}
		for (int l9 = 0; l9 < 2048; l9 = l9 + 1) {
			fVec4[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 2; l10 = l10 + 1) {
			fRec14[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 2; l11 = l11 + 1) {
			fRec21[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 2; l12 = l12 + 1) {
			fRec20[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 16384; l13 = l13 + 1) {
			fVec5[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 4096; l14 = l14 + 1) {
			fVec6[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 2; l15 = l15 + 1) {
			fRec18[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 2; l16 = l16 + 1) {
			fRec25[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 2; l17 = l17 + 1) {
			fRec24[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 16384; l18 = l18 + 1) {
			fVec7[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 2048; l19 = l19 + 1) {
			fVec8[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 2; l20 = l20 + 1) {
			fRec22[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 2; l21 = l21 + 1) {
			fRec29[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 2; l22 = l22 + 1) {
			fRec28[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 32768; l23 = l23 + 1) {
			fVec9[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 16384; l24 = l24 + 1) {
			fVec10[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 4096; l25 = l25 + 1) {
			fVec11[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 2; l26 = l26 + 1) {
			fRec26[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 2; l27 = l27 + 1) {
			fRec33[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 2; l28 = l28 + 1) {
			fRec32[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 16384; l29 = l29 + 1) {
			fVec12[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 4096; l30 = l30 + 1) {
			fVec13[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 2; l31 = l31 + 1) {
			fRec30[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 2; l32 = l32 + 1) {
			fRec37[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 2; l33 = l33 + 1) {
			fRec36[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 32768; l34 = l34 + 1) {
			fVec14[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 4096; l35 = l35 + 1) {
			fVec15[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 2; l36 = l36 + 1) {
			fRec34[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 2; l37 = l37 + 1) {
			fRec41[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 2; l38 = l38 + 1) {
			fRec40[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 32768; l39 = l39 + 1) {
			fVec16[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 2048; l40 = l40 + 1) {
			fVec17[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 2; l41 = l41 + 1) {
			fRec38[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 3; l42 = l42 + 1) {
			fRec2[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 3; l43 = l43 + 1) {
			fRec3[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 3; l44 = l44 + 1) {
			fRec4[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 3; l45 = l45 + 1) {
			fRec5[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 3; l46 = l46 + 1) {
			fRec6[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 3; l47 = l47 + 1) {
			fRec7[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 3; l48 = l48 + 1) {
			fRec8[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 3; l49 = l49 + 1) {
			fRec9[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 3; l50 = l50 + 1) {
			fRec1[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 3; l51 = l51 + 1) {
			fRec0[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 2; l52 = l52 + 1) {
			fRec42[l52] = 0.0f;
		}
		for (int l53 = 0; l53 < 2; l53 = l53 + 1) {
			fRec43[l53] = 0.0f;
		}
		for (int l54 = 0; l54 < 3; l54 = l54 + 1) {
			fRec45[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 3; l55 = l55 + 1) {
			fRec44[l55] = 0.0f;
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
		ui_interface->openHorizontalBox("Zita Light");
		ui_interface->declare(&fVslider0, "1", "");
		ui_interface->declare(&fVslider0, "style", "knob");
		ui_interface->declare(&fVslider0, "tooltip", "-1 = dry, 1 = wet");
		ui_interface->addVerticalSlider("Dry/Wet Mix", &fVslider0, FAUSTFLOAT(0.0f), FAUSTFLOAT(-1.0f), FAUSTFLOAT(1.0f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fVslider1, "2", "");
		ui_interface->declare(&fVslider1, "style", "knob");
		ui_interface->declare(&fVslider1, "tooltip", "Output scale         factor");
		ui_interface->declare(&fVslider1, "unit", "dB");
		ui_interface->addVerticalSlider("Level", &fVslider1, FAUSTFLOAT(-6.0f), FAUSTFLOAT(-7e+01f), FAUSTFLOAT(4e+01f), FAUSTFLOAT(0.1f));
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0 = inputs[0];
		FAUSTFLOAT* input1 = inputs[1];
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = fConst126 * float(fVslider0);
		float fSlow1 = fConst126 * std::pow(1e+01f, 0.05f * float(fVslider1));
		for (int i0 = 0; i0 < count; i0 = i0 + 1) {
			float fTemp0 = fConst6 * fRec1[1];
			fRec13[0] = -(fConst19 * (fConst18 * fRec13[1] - (fRec6[1] + fRec6[2])));
			fRec12[0] = fConst21 * (fRec6[1] + fConst20 * fRec13[0]) + fConst16 * fRec12[1];
			fVec0[IOTA0 & 16383] = 0.35355338f * fRec12[0] + 1e-20f;
			float fTemp1 = float(input0[i0]);
			fVec1[IOTA0 & 16383] = fTemp1;
			float fTemp2 = 0.3f * fVec1[(IOTA0 - iConst24) & 16383];
			float fTemp3 = fTemp2 + fVec0[(IOTA0 - iConst23) & 16383] - 0.6f * fRec10[1];
			fVec2[IOTA0 & 4095] = fTemp3;
			fRec10[0] = fVec2[(IOTA0 - iConst25) & 4095];
			float fRec11 = 0.6f * fTemp3;
			fRec17[0] = -(fConst19 * (fConst18 * fRec17[1] - (fRec2[1] + fRec2[2])));
			fRec16[0] = fConst36 * (fRec2[1] + fConst35 * fRec17[0]) + fConst34 * fRec16[1];
			fVec3[IOTA0 & 16383] = 0.35355338f * fRec16[0] + 1e-20f;
			float fTemp4 = fVec3[(IOTA0 - iConst38) & 16383] + fTemp2 - 0.6f * fRec14[1];
			fVec4[IOTA0 & 2047] = fTemp4;
			fRec14[0] = fVec4[(IOTA0 - iConst39) & 2047];
			float fRec15 = 0.6f * fTemp4;
			float fTemp5 = fRec15 + fRec11;
			fRec21[0] = -(fConst19 * (fConst18 * fRec21[1] - (fRec4[1] + fRec4[2])));
			fRec20[0] = fConst50 * (fRec4[1] + fConst49 * fRec21[0]) + fConst48 * fRec20[1];
			fVec5[IOTA0 & 16383] = 0.35355338f * fRec20[0] + 1e-20f;
			float fTemp6 = fVec5[(IOTA0 - iConst52) & 16383] - (fTemp2 + 0.6f * fRec18[1]);
			fVec6[IOTA0 & 4095] = fTemp6;
			fRec18[0] = fVec6[(IOTA0 - iConst53) & 4095];
			float fRec19 = 0.6f * fTemp6;
			fRec25[0] = -(fConst19 * (fConst18 * fRec25[1] - (fRec8[1] + fRec8[2])));
			fRec24[0] = fConst64 * (fRec8[1] + fConst63 * fRec25[0]) + fConst62 * fRec24[1];
			fVec7[IOTA0 & 16383] = 0.35355338f * fRec24[0] + 1e-20f;
			float fTemp7 = fVec7[(IOTA0 - iConst66) & 16383] - (fTemp2 + 0.6f * fRec22[1]);
			fVec8[IOTA0 & 2047] = fTemp7;
			fRec22[0] = fVec8[(IOTA0 - iConst67) & 2047];
			float fRec23 = 0.6f * fTemp7;
			float fTemp8 = fRec23 + fRec19 + fTemp5;
			fRec29[0] = -(fConst19 * (fConst18 * fRec29[1] - (fRec3[1] + fRec3[2])));
			fRec28[0] = fConst78 * (fRec3[1] + fConst77 * fRec29[0]) + fConst76 * fRec28[1];
			fVec9[IOTA0 & 32767] = 0.35355338f * fRec28[0] + 1e-20f;
			float fTemp9 = float(input1[i0]);
			fVec10[IOTA0 & 16383] = fTemp9;
			float fTemp10 = 0.3f * fVec10[(IOTA0 - iConst24) & 16383];
			float fTemp11 = fTemp10 + 0.6f * fRec26[1] + fVec9[(IOTA0 - iConst80) & 32767];
			fVec11[IOTA0 & 4095] = fTemp11;
			fRec26[0] = fVec11[(IOTA0 - iConst81) & 4095];
			float fRec27 = -(0.6f * fTemp11);
			fRec33[0] = -(fConst19 * (fConst18 * fRec33[1] - (fRec7[1] + fRec7[2])));
			fRec32[0] = fConst92 * (fRec7[1] + fConst91 * fRec33[0]) + fConst90 * fRec32[1];
			fVec12[IOTA0 & 16383] = 0.35355338f * fRec32[0] + 1e-20f;
			float fTemp12 = fVec12[(IOTA0 - iConst94) & 16383] + fTemp10 + 0.6f * fRec30[1];
			fVec13[IOTA0 & 4095] = fTemp12;
			fRec30[0] = fVec13[(IOTA0 - iConst95) & 4095];
			float fRec31 = -(0.6f * fTemp12);
			fRec37[0] = -(fConst19 * (fConst18 * fRec37[1] - (fRec5[1] + fRec5[2])));
			fRec36[0] = fConst106 * (fRec5[1] + fConst105 * fRec37[0]) + fConst104 * fRec36[1];
			fVec14[IOTA0 & 32767] = 0.35355338f * fRec36[0] + 1e-20f;
			float fTemp13 = 0.6f * fRec34[1] + fVec14[(IOTA0 - iConst108) & 32767];
			fVec15[IOTA0 & 4095] = fTemp13 - fTemp10;
			fRec34[0] = fVec15[(IOTA0 - iConst109) & 4095];
			float fRec35 = 0.6f * (fTemp10 - fTemp13);
			fRec41[0] = -(fConst19 * (fConst18 * fRec41[1] - (fRec9[1] + fRec9[2])));
			fRec40[0] = fConst120 * (fRec9[1] + fConst119 * fRec41[0]) + fConst118 * fRec40[1];
			fVec16[IOTA0 & 32767] = 0.35355338f * fRec40[0] + 1e-20f;
			float fTemp14 = 0.6f * fRec38[1] + fVec16[(IOTA0 - iConst122) & 32767];
			fVec17[IOTA0 & 2047] = fTemp14 - fTemp10;
			fRec38[0] = fVec17[(IOTA0 - iConst123) & 2047];
			float fRec39 = 0.6f * (fTemp10 - fTemp14);
			fRec2[0] = fRec38[1] + fRec34[1] + fRec30[1] + fRec26[1] + fRec22[1] + fRec18[1] + fRec10[1] + fRec14[1] + fRec39 + fRec35 + fRec31 + fRec27 + fTemp8;
			fRec3[0] = fRec22[1] + fRec18[1] + fRec10[1] + fRec14[1] + fTemp8 - (fRec38[1] + fRec34[1] + fRec30[1] + fRec26[1] + fRec39 + fRec35 + fRec27 + fRec31);
			float fTemp15 = fRec19 + fRec23;
			fRec4[0] = fRec30[1] + fRec26[1] + fRec10[1] + fRec14[1] + fRec31 + fRec27 + fTemp5 - (fRec38[1] + fRec34[1] + fRec22[1] + fRec18[1] + fRec39 + fRec35 + fTemp15);
			fRec5[0] = fRec38[1] + fRec34[1] + fRec10[1] + fRec14[1] + fRec39 + fRec35 + fTemp5 - (fRec30[1] + fRec26[1] + fRec22[1] + fRec18[1] + fRec31 + fRec27 + fTemp15);
			float fTemp16 = fRec11 + fRec23;
			float fTemp17 = fRec15 + fRec19;
			fRec6[0] = fRec34[1] + fRec26[1] + fRec18[1] + fRec14[1] + fRec35 + fRec27 + fTemp17 - (fRec38[1] + fRec30[1] + fRec22[1] + fRec10[1] + fRec39 + fRec31 + fTemp16);
			fRec7[0] = fRec38[1] + fRec30[1] + fRec18[1] + fRec14[1] + fRec39 + fRec31 + fTemp17 - (fRec34[1] + fRec26[1] + fRec22[1] + fRec10[1] + fRec35 + fRec27 + fTemp16);
			float fTemp18 = fRec11 + fRec19;
			float fTemp19 = fRec15 + fRec23;
			fRec8[0] = fRec38[1] + fRec26[1] + fRec22[1] + fRec14[1] + fRec39 + fRec27 + fTemp19 - (fRec34[1] + fRec30[1] + fRec18[1] + fRec10[1] + fRec35 + fRec31 + fTemp18);
			fRec9[0] = fRec34[1] + fRec30[1] + fRec22[1] + fRec14[1] + fRec35 + fRec31 + fTemp19 - (fRec38[1] + fRec26[1] + fRec18[1] + fRec10[1] + fRec39 + fRec27 + fTemp18);
			float fTemp20 = 0.37f * (fRec3[0] + fRec4[0]);
			float fTemp21 = fTemp20 + fTemp0;
			fRec1[0] = fTemp21 - fConst5 * fRec1[2];
			float fTemp22 = fConst5 * fRec1[0];
			fRec0[0] = 0.5f * (fTemp22 + fTemp20 + fRec1[2] - fTemp0 + (fRec1[2] + fTemp22 - fTemp21)) + fConst3 * fRec0[1] - fConst2 * fRec0[2];
			fRec42[0] = fSlow0 + fConst127 * fRec42[1];
			float fTemp23 = fRec42[0] + 1.0f;
			float fTemp24 = 1.0f - 0.5f * fTemp23;
			fRec43[0] = fSlow1 + fConst127 * fRec43[1];
			output0[i0] = FAUSTFLOAT(0.5f * fRec43[0] * (fTemp1 * fTemp23 + fTemp24 * (2.0f * fRec0[2] + fConst125 * fRec0[0] - fConst124 * fRec0[1])));
			float fTemp25 = fConst6 * fRec45[1];
			float fTemp26 = 0.37f * (fRec3[0] - fRec4[0]);
			float fTemp27 = fTemp26 + fTemp25;
			fRec45[0] = fTemp27 - fConst5 * fRec45[2];
			float fTemp28 = fConst5 * fRec45[0];
			fRec44[0] = 0.5f * (fTemp28 + fTemp26 + fRec45[2] - fTemp25 + (fRec45[2] + fTemp28 - fTemp27)) + fConst3 * fRec44[1] - fConst2 * fRec44[2];
			output1[i0] = FAUSTFLOAT(0.5f * fRec43[0] * (fTemp9 * fTemp23 + fTemp24 * (2.0f * fRec44[2] + fConst125 * fRec44[0] - fConst124 * fRec44[1])));
			fRec13[1] = fRec13[0];
			fRec12[1] = fRec12[0];
			IOTA0 = IOTA0 + 1;
			fRec10[1] = fRec10[0];
			fRec17[1] = fRec17[0];
			fRec16[1] = fRec16[0];
			fRec14[1] = fRec14[0];
			fRec21[1] = fRec21[0];
			fRec20[1] = fRec20[0];
			fRec18[1] = fRec18[0];
			fRec25[1] = fRec25[0];
			fRec24[1] = fRec24[0];
			fRec22[1] = fRec22[0];
			fRec29[1] = fRec29[0];
			fRec28[1] = fRec28[0];
			fRec26[1] = fRec26[0];
			fRec33[1] = fRec33[0];
			fRec32[1] = fRec32[0];
			fRec30[1] = fRec30[0];
			fRec37[1] = fRec37[0];
			fRec36[1] = fRec36[0];
			fRec34[1] = fRec34[0];
			fRec41[1] = fRec41[0];
			fRec40[1] = fRec40[0];
			fRec38[1] = fRec38[0];
			fRec2[2] = fRec2[1];
			fRec2[1] = fRec2[0];
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
			fRec1[2] = fRec1[1];
			fRec1[1] = fRec1[0];
			fRec0[2] = fRec0[1];
			fRec0[1] = fRec0[0];
			fRec42[1] = fRec42[0];
			fRec43[1] = fRec43[0];
			fRec45[2] = fRec45[1];
			fRec45[1] = fRec45[0];
			fRec44[2] = fRec44[1];
			fRec44[1] = fRec44[0];
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
