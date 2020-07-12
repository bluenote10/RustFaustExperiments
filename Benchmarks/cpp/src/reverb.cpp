/* ------------------------------------------------------------
name: "reverb"
Code generated with Faust 2.26.0 (https://faust.grame.fr)
Compilation options: -lang cpp -scal -ftz 0
------------------------------------------------------------ */

#ifndef  __Dsp_H__
#define  __Dsp_H__


#include <algorithm>
#include <chrono>
#include <iostream>
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
#include <math.h>

static float Dsp_faustpower2_f(float value) {
	return (value * value);
}

#ifndef FAUSTCLASS 
#define FAUSTCLASS Dsp
#endif

#ifdef __APPLE__ 
#define exp10f __exp10f
#define exp10 __exp10
#endif

class Dsp : public dsp {
	
 private:
	
	FAUSTFLOAT fVslider0;
	float fRec0[2];
	int IOTA;
	float fVec0[16384];
	FAUSTFLOAT fVslider1;
	float fRec1[2];
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	float fConst3;
	float fConst4;
	float fConst5;
	float fConst6;
	float fConst7;
	float fConst8;
	float fConst9;
	float fConst10;
	float fConst11;
	float fConst12;
	float fConst13;
	float fConst14;
	float fConst15;
	float fConst16;
	float fConst17;
	float fConst18;
	float fConst19;
	float fRec15[2];
	float fRec14[2];
	float fVec1[32768];
	float fConst20;
	int iConst21;
	float fVec2[16384];
	int iConst22;
	float fVec3[2048];
	int iConst23;
	float fRec12[2];
	float fConst24;
	float fConst25;
	float fConst26;
	float fConst27;
	float fConst28;
	float fConst29;
	float fConst30;
	float fConst31;
	float fConst32;
	float fConst33;
	float fConst34;
	float fRec19[2];
	float fRec18[2];
	float fVec4[32768];
	float fConst35;
	int iConst36;
	float fVec5[4096];
	int iConst37;
	float fRec16[2];
	float fConst38;
	float fConst39;
	float fConst40;
	float fConst41;
	float fConst42;
	float fConst43;
	float fConst44;
	float fConst45;
	float fConst46;
	float fConst47;
	float fConst48;
	float fRec23[2];
	float fRec22[2];
	float fVec6[16384];
	float fConst49;
	int iConst50;
	float fVec7[4096];
	int iConst51;
	float fRec20[2];
	float fConst52;
	float fConst53;
	float fConst54;
	float fConst55;
	float fConst56;
	float fConst57;
	float fConst58;
	float fConst59;
	float fConst60;
	float fConst61;
	float fConst62;
	float fRec27[2];
	float fRec26[2];
	float fVec8[32768];
	float fConst63;
	int iConst64;
	float fVec9[4096];
	int iConst65;
	float fRec24[2];
	float fConst66;
	float fConst67;
	float fConst68;
	float fConst69;
	float fConst70;
	float fConst71;
	float fConst72;
	float fConst73;
	float fConst74;
	float fConst75;
	float fConst76;
	float fRec31[2];
	float fRec30[2];
	float fVec10[16384];
	float fConst77;
	int iConst78;
	float fVec11[2048];
	int iConst79;
	float fRec28[2];
	float fConst80;
	float fConst81;
	float fConst82;
	float fConst83;
	float fConst84;
	float fConst85;
	float fConst86;
	float fConst87;
	float fConst88;
	float fConst89;
	float fConst90;
	float fRec35[2];
	float fRec34[2];
	float fVec12[16384];
	float fConst91;
	int iConst92;
	float fVec13[4096];
	int iConst93;
	float fRec32[2];
	float fConst94;
	float fConst95;
	float fConst96;
	float fConst97;
	float fConst98;
	float fConst99;
	float fConst100;
	float fConst101;
	float fConst102;
	float fConst103;
	float fConst104;
	float fRec39[2];
	float fRec38[2];
	float fVec14[16384];
	float fConst105;
	int iConst106;
	float fVec15[4096];
	int iConst107;
	float fRec36[2];
	float fConst108;
	float fConst109;
	float fConst110;
	float fConst111;
	float fConst112;
	float fConst113;
	float fConst114;
	float fConst115;
	float fConst116;
	float fConst117;
	float fConst118;
	float fRec43[2];
	float fRec42[2];
	float fVec16[16384];
	float fConst119;
	int iConst120;
	float fVec17[2048];
	int iConst121;
	float fRec40[2];
	float fRec4[3];
	float fRec5[3];
	float fRec6[3];
	float fRec7[3];
	float fRec8[3];
	float fRec9[3];
	float fRec10[3];
	float fRec11[3];
	float fConst122;
	float fRec3[3];
	float fConst123;
	float fRec2[3];
	float fRec45[3];
	float fRec44[3];
	
 public:
	
	void metadata(Meta* m) { 
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/version", "0.1");
		m->declare("delays.lib/name", "Faust Delay Library");
		m->declare("delays.lib/version", "0.1");
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
		m->declare("maths.lib/author", "GRAME");
		m->declare("maths.lib/copyright", "GRAME");
		m->declare("maths.lib/license", "LGPL with exception");
		m->declare("maths.lib/name", "Faust Math Library");
		m->declare("maths.lib/version", "2.3");
		m->declare("name", "reverb");
		m->declare("platform.lib/name", "Generic Platform Library");
		m->declare("platform.lib/version", "0.1");
		m->declare("reverbs.lib/name", "Faust Reverb Library");
		m->declare("reverbs.lib/version", "0.0");
		m->declare("routes.lib/name", "Faust Signal Routing Library");
		m->declare("routes.lib/version", "0.2");
		m->declare("signals.lib/name", "Faust Signal Routing Library");
		m->declare("signals.lib/version", "0.0");
	}

	virtual int getNumInputs() {
		return 2;
	}
	virtual int getNumOutputs() {
		return 2;
	}
	virtual int getInputRate(int channel) {
		int rate;
		switch ((channel)) {
			case 0: {
				rate = 1;
				break;
			}
			case 1: {
				rate = 1;
				break;
			}
			default: {
				rate = -1;
				break;
			}
		}
		return rate;
	}
	virtual int getOutputRate(int channel) {
		int rate;
		switch ((channel)) {
			case 0: {
				rate = 1;
				break;
			}
			case 1: {
				rate = 1;
				break;
			}
			default: {
				rate = -1;
				break;
			}
		}
		return rate;
	}
	
	static void classInit(int sample_rate) {
	}
	
	virtual void instanceConstants(int sample_rate) {
		fSampleRate = sample_rate;
		fConst0 = std::min<float>(192000.0f, std::max<float>(1.0f, float(fSampleRate)));
		fConst1 = (9424.77832f / fConst0);
		fConst2 = ((1.0f - fConst1) / (fConst1 + 1.0f));
		fConst3 = (1979.20337f / fConst0);
		fConst4 = ((1.0f - fConst3) / (fConst3 + 1.0f));
		fConst5 = std::floor(((0.219990999f * fConst0) + 0.5f));
		fConst6 = ((0.0f - (6.90775537f * fConst5)) / fConst0);
		fConst7 = std::exp((0.5f * fConst6));
		fConst8 = Dsp_faustpower2_f(fConst7);
		fConst9 = std::cos((37699.1133f / fConst0));
		fConst10 = (1.0f - (fConst8 * fConst9));
		fConst11 = (1.0f - fConst8);
		fConst12 = (fConst10 / fConst11);
		fConst13 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst10) / Dsp_faustpower2_f(fConst11)) + -1.0f)));
		fConst14 = (fConst12 - fConst13);
		fConst15 = (fConst7 * (fConst13 + (1.0f - fConst12)));
		fConst16 = ((std::exp((0.333333343f * fConst6)) / fConst7) + -1.0f);
		fConst17 = (1.0f / std::tan((628.318542f / fConst0)));
		fConst18 = (1.0f / (fConst17 + 1.0f));
		fConst19 = (1.0f - fConst17);
		fConst20 = std::floor(((0.0191229992f * fConst0) + 0.5f));
		iConst21 = int(std::min<float>(16384.0f, std::max<float>(0.0f, (fConst5 - fConst20))));
		iConst22 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (0.0599999987f * fConst0))));
		iConst23 = int(std::min<float>(1024.0f, std::max<float>(0.0f, (fConst20 + -1.0f))));
		fConst24 = std::floor(((0.256891012f * fConst0) + 0.5f));
		fConst25 = ((0.0f - (6.90775537f * fConst24)) / fConst0);
		fConst26 = std::exp((0.5f * fConst25));
		fConst27 = Dsp_faustpower2_f(fConst26);
		fConst28 = (1.0f - (fConst27 * fConst9));
		fConst29 = (1.0f - fConst27);
		fConst30 = (fConst28 / fConst29);
		fConst31 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst28) / Dsp_faustpower2_f(fConst29)) + -1.0f)));
		fConst32 = (fConst30 - fConst31);
		fConst33 = (fConst26 * (fConst31 + (1.0f - fConst30)));
		fConst34 = ((std::exp((0.333333343f * fConst25)) / fConst26) + -1.0f);
		fConst35 = std::floor(((0.0273330007f * fConst0) + 0.5f));
		iConst36 = int(std::min<float>(16384.0f, std::max<float>(0.0f, (fConst24 - fConst35))));
		iConst37 = int(std::min<float>(2048.0f, std::max<float>(0.0f, (fConst35 + -1.0f))));
		fConst38 = std::floor(((0.192303002f * fConst0) + 0.5f));
		fConst39 = ((0.0f - (6.90775537f * fConst38)) / fConst0);
		fConst40 = std::exp((0.5f * fConst39));
		fConst41 = Dsp_faustpower2_f(fConst40);
		fConst42 = (1.0f - (fConst41 * fConst9));
		fConst43 = (1.0f - fConst41);
		fConst44 = (fConst42 / fConst43);
		fConst45 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst42) / Dsp_faustpower2_f(fConst43)) + -1.0f)));
		fConst46 = (fConst44 - fConst45);
		fConst47 = (fConst40 * (fConst45 + (1.0f - fConst44)));
		fConst48 = ((std::exp((0.333333343f * fConst39)) / fConst40) + -1.0f);
		fConst49 = std::floor(((0.0292910002f * fConst0) + 0.5f));
		iConst50 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (fConst38 - fConst49))));
		iConst51 = int(std::min<float>(2048.0f, std::max<float>(0.0f, (fConst49 + -1.0f))));
		fConst52 = std::floor(((0.210389003f * fConst0) + 0.5f));
		fConst53 = ((0.0f - (6.90775537f * fConst52)) / fConst0);
		fConst54 = std::exp((0.5f * fConst53));
		fConst55 = Dsp_faustpower2_f(fConst54);
		fConst56 = (1.0f - (fConst55 * fConst9));
		fConst57 = (1.0f - fConst55);
		fConst58 = (fConst56 / fConst57);
		fConst59 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst56) / Dsp_faustpower2_f(fConst57)) + -1.0f)));
		fConst60 = (fConst58 - fConst59);
		fConst61 = (fConst54 * (fConst59 + (1.0f - fConst58)));
		fConst62 = ((std::exp((0.333333343f * fConst53)) / fConst54) + -1.0f);
		fConst63 = std::floor(((0.0244210009f * fConst0) + 0.5f));
		iConst64 = int(std::min<float>(16384.0f, std::max<float>(0.0f, (fConst52 - fConst63))));
		iConst65 = int(std::min<float>(2048.0f, std::max<float>(0.0f, (fConst63 + -1.0f))));
		fConst66 = std::floor(((0.125f * fConst0) + 0.5f));
		fConst67 = ((0.0f - (6.90775537f * fConst66)) / fConst0);
		fConst68 = std::exp((0.5f * fConst67));
		fConst69 = Dsp_faustpower2_f(fConst68);
		fConst70 = (1.0f - (fConst69 * fConst9));
		fConst71 = (1.0f - fConst69);
		fConst72 = (fConst70 / fConst71);
		fConst73 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst70) / Dsp_faustpower2_f(fConst71)) + -1.0f)));
		fConst74 = (fConst72 - fConst73);
		fConst75 = (fConst68 * (fConst73 + (1.0f - fConst72)));
		fConst76 = ((std::exp((0.333333343f * fConst67)) / fConst68) + -1.0f);
		fConst77 = std::floor(((0.0134579996f * fConst0) + 0.5f));
		iConst78 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (fConst66 - fConst77))));
		iConst79 = int(std::min<float>(1024.0f, std::max<float>(0.0f, (fConst77 + -1.0f))));
		fConst80 = std::floor(((0.127837002f * fConst0) + 0.5f));
		fConst81 = ((0.0f - (6.90775537f * fConst80)) / fConst0);
		fConst82 = std::exp((0.5f * fConst81));
		fConst83 = Dsp_faustpower2_f(fConst82);
		fConst84 = (1.0f - (fConst83 * fConst9));
		fConst85 = (1.0f - fConst83);
		fConst86 = (fConst84 / fConst85);
		fConst87 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst84) / Dsp_faustpower2_f(fConst85)) + -1.0f)));
		fConst88 = (fConst86 - fConst87);
		fConst89 = (fConst82 * (fConst87 + (1.0f - fConst86)));
		fConst90 = ((std::exp((0.333333343f * fConst81)) / fConst82) + -1.0f);
		fConst91 = std::floor(((0.0316039994f * fConst0) + 0.5f));
		iConst92 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (fConst80 - fConst91))));
		iConst93 = int(std::min<float>(2048.0f, std::max<float>(0.0f, (fConst91 + -1.0f))));
		fConst94 = std::floor(((0.174713001f * fConst0) + 0.5f));
		fConst95 = ((0.0f - (6.90775537f * fConst94)) / fConst0);
		fConst96 = std::exp((0.5f * fConst95));
		fConst97 = Dsp_faustpower2_f(fConst96);
		fConst98 = (1.0f - (fConst97 * fConst9));
		fConst99 = (1.0f - fConst97);
		fConst100 = (fConst98 / fConst99);
		fConst101 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst98) / Dsp_faustpower2_f(fConst99)) + -1.0f)));
		fConst102 = (fConst100 - fConst101);
		fConst103 = (fConst96 * (fConst101 + (1.0f - fConst100)));
		fConst104 = ((std::exp((0.333333343f * fConst95)) / fConst96) + -1.0f);
		fConst105 = std::floor(((0.0229039993f * fConst0) + 0.5f));
		iConst106 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (fConst94 - fConst105))));
		iConst107 = int(std::min<float>(2048.0f, std::max<float>(0.0f, (fConst105 + -1.0f))));
		fConst108 = std::floor(((0.153128996f * fConst0) + 0.5f));
		fConst109 = ((0.0f - (6.90775537f * fConst108)) / fConst0);
		fConst110 = std::exp((0.5f * fConst109));
		fConst111 = Dsp_faustpower2_f(fConst110);
		fConst112 = (1.0f - (fConst111 * fConst9));
		fConst113 = (1.0f - fConst111);
		fConst114 = (fConst112 / fConst113);
		fConst115 = std::sqrt(std::max<float>(0.0f, ((Dsp_faustpower2_f(fConst112) / Dsp_faustpower2_f(fConst113)) + -1.0f)));
		fConst116 = (fConst114 - fConst115);
		fConst117 = (fConst110 * (fConst115 + (1.0f - fConst114)));
		fConst118 = ((std::exp((0.333333343f * fConst109)) / fConst110) + -1.0f);
		fConst119 = std::floor(((0.0203460008f * fConst0) + 0.5f));
		iConst120 = int(std::min<float>(8192.0f, std::max<float>(0.0f, (fConst108 - fConst119))));
		iConst121 = int(std::min<float>(1024.0f, std::max<float>(0.0f, (fConst119 + -1.0f))));
		fConst122 = (0.0f - (std::cos(fConst3) * (fConst4 + 1.0f)));
		fConst123 = (0.0f - (std::cos(fConst1) * (fConst2 + 1.0f)));
	}
	
	virtual void instanceResetUserInterface() {
		fVslider0 = FAUSTFLOAT(-6.0f);
		fVslider1 = FAUSTFLOAT(0.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; (l0 < 2); l0 = (l0 + 1)) {
			fRec0[l0] = 0.0f;
		}
		IOTA = 0;
		for (int l1 = 0; (l1 < 16384); l1 = (l1 + 1)) {
			fVec0[l1] = 0.0f;
		}
		for (int l2 = 0; (l2 < 2); l2 = (l2 + 1)) {
			fRec1[l2] = 0.0f;
		}
		for (int l3 = 0; (l3 < 2); l3 = (l3 + 1)) {
			fRec15[l3] = 0.0f;
		}
		for (int l4 = 0; (l4 < 2); l4 = (l4 + 1)) {
			fRec14[l4] = 0.0f;
		}
		for (int l5 = 0; (l5 < 32768); l5 = (l5 + 1)) {
			fVec1[l5] = 0.0f;
		}
		for (int l6 = 0; (l6 < 16384); l6 = (l6 + 1)) {
			fVec2[l6] = 0.0f;
		}
		for (int l7 = 0; (l7 < 2048); l7 = (l7 + 1)) {
			fVec3[l7] = 0.0f;
		}
		for (int l8 = 0; (l8 < 2); l8 = (l8 + 1)) {
			fRec12[l8] = 0.0f;
		}
		for (int l9 = 0; (l9 < 2); l9 = (l9 + 1)) {
			fRec19[l9] = 0.0f;
		}
		for (int l10 = 0; (l10 < 2); l10 = (l10 + 1)) {
			fRec18[l10] = 0.0f;
		}
		for (int l11 = 0; (l11 < 32768); l11 = (l11 + 1)) {
			fVec4[l11] = 0.0f;
		}
		for (int l12 = 0; (l12 < 4096); l12 = (l12 + 1)) {
			fVec5[l12] = 0.0f;
		}
		for (int l13 = 0; (l13 < 2); l13 = (l13 + 1)) {
			fRec16[l13] = 0.0f;
		}
		for (int l14 = 0; (l14 < 2); l14 = (l14 + 1)) {
			fRec23[l14] = 0.0f;
		}
		for (int l15 = 0; (l15 < 2); l15 = (l15 + 1)) {
			fRec22[l15] = 0.0f;
		}
		for (int l16 = 0; (l16 < 16384); l16 = (l16 + 1)) {
			fVec6[l16] = 0.0f;
		}
		for (int l17 = 0; (l17 < 4096); l17 = (l17 + 1)) {
			fVec7[l17] = 0.0f;
		}
		for (int l18 = 0; (l18 < 2); l18 = (l18 + 1)) {
			fRec20[l18] = 0.0f;
		}
		for (int l19 = 0; (l19 < 2); l19 = (l19 + 1)) {
			fRec27[l19] = 0.0f;
		}
		for (int l20 = 0; (l20 < 2); l20 = (l20 + 1)) {
			fRec26[l20] = 0.0f;
		}
		for (int l21 = 0; (l21 < 32768); l21 = (l21 + 1)) {
			fVec8[l21] = 0.0f;
		}
		for (int l22 = 0; (l22 < 4096); l22 = (l22 + 1)) {
			fVec9[l22] = 0.0f;
		}
		for (int l23 = 0; (l23 < 2); l23 = (l23 + 1)) {
			fRec24[l23] = 0.0f;
		}
		for (int l24 = 0; (l24 < 2); l24 = (l24 + 1)) {
			fRec31[l24] = 0.0f;
		}
		for (int l25 = 0; (l25 < 2); l25 = (l25 + 1)) {
			fRec30[l25] = 0.0f;
		}
		for (int l26 = 0; (l26 < 16384); l26 = (l26 + 1)) {
			fVec10[l26] = 0.0f;
		}
		for (int l27 = 0; (l27 < 2048); l27 = (l27 + 1)) {
			fVec11[l27] = 0.0f;
		}
		for (int l28 = 0; (l28 < 2); l28 = (l28 + 1)) {
			fRec28[l28] = 0.0f;
		}
		for (int l29 = 0; (l29 < 2); l29 = (l29 + 1)) {
			fRec35[l29] = 0.0f;
		}
		for (int l30 = 0; (l30 < 2); l30 = (l30 + 1)) {
			fRec34[l30] = 0.0f;
		}
		for (int l31 = 0; (l31 < 16384); l31 = (l31 + 1)) {
			fVec12[l31] = 0.0f;
		}
		for (int l32 = 0; (l32 < 4096); l32 = (l32 + 1)) {
			fVec13[l32] = 0.0f;
		}
		for (int l33 = 0; (l33 < 2); l33 = (l33 + 1)) {
			fRec32[l33] = 0.0f;
		}
		for (int l34 = 0; (l34 < 2); l34 = (l34 + 1)) {
			fRec39[l34] = 0.0f;
		}
		for (int l35 = 0; (l35 < 2); l35 = (l35 + 1)) {
			fRec38[l35] = 0.0f;
		}
		for (int l36 = 0; (l36 < 16384); l36 = (l36 + 1)) {
			fVec14[l36] = 0.0f;
		}
		for (int l37 = 0; (l37 < 4096); l37 = (l37 + 1)) {
			fVec15[l37] = 0.0f;
		}
		for (int l38 = 0; (l38 < 2); l38 = (l38 + 1)) {
			fRec36[l38] = 0.0f;
		}
		for (int l39 = 0; (l39 < 2); l39 = (l39 + 1)) {
			fRec43[l39] = 0.0f;
		}
		for (int l40 = 0; (l40 < 2); l40 = (l40 + 1)) {
			fRec42[l40] = 0.0f;
		}
		for (int l41 = 0; (l41 < 16384); l41 = (l41 + 1)) {
			fVec16[l41] = 0.0f;
		}
		for (int l42 = 0; (l42 < 2048); l42 = (l42 + 1)) {
			fVec17[l42] = 0.0f;
		}
		for (int l43 = 0; (l43 < 2); l43 = (l43 + 1)) {
			fRec40[l43] = 0.0f;
		}
		for (int l44 = 0; (l44 < 3); l44 = (l44 + 1)) {
			fRec4[l44] = 0.0f;
		}
		for (int l45 = 0; (l45 < 3); l45 = (l45 + 1)) {
			fRec5[l45] = 0.0f;
		}
		for (int l46 = 0; (l46 < 3); l46 = (l46 + 1)) {
			fRec6[l46] = 0.0f;
		}
		for (int l47 = 0; (l47 < 3); l47 = (l47 + 1)) {
			fRec7[l47] = 0.0f;
		}
		for (int l48 = 0; (l48 < 3); l48 = (l48 + 1)) {
			fRec8[l48] = 0.0f;
		}
		for (int l49 = 0; (l49 < 3); l49 = (l49 + 1)) {
			fRec9[l49] = 0.0f;
		}
		for (int l50 = 0; (l50 < 3); l50 = (l50 + 1)) {
			fRec10[l50] = 0.0f;
		}
		for (int l51 = 0; (l51 < 3); l51 = (l51 + 1)) {
			fRec11[l51] = 0.0f;
		}
		for (int l52 = 0; (l52 < 3); l52 = (l52 + 1)) {
			fRec3[l52] = 0.0f;
		}
		for (int l53 = 0; (l53 < 3); l53 = (l53 + 1)) {
			fRec2[l53] = 0.0f;
		}
		for (int l54 = 0; (l54 < 3); l54 = (l54 + 1)) {
			fRec45[l54] = 0.0f;
		}
		for (int l55 = 0; (l55 < 3); l55 = (l55 + 1)) {
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
		ui_interface->declare(&fVslider1, "1", "");
		ui_interface->declare(&fVslider1, "style", "knob");
		ui_interface->declare(&fVslider1, "tooltip", "-1 = dry, 1 = wet");
		ui_interface->addVerticalSlider("Dry/Wet Mix", &fVslider1, 0.0f, -1.0f, 1.0f, 0.00999999978f);
		ui_interface->declare(&fVslider0, "2", "");
		ui_interface->declare(&fVslider0, "style", "knob");
		ui_interface->declare(&fVslider0, "tooltip", "Output scale   factor");
		ui_interface->declare(&fVslider0, "unit", "dB");
		ui_interface->addVerticalSlider("Level", &fVslider0, -6.0f, -70.0f, 40.0f, 0.100000001f);
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** inputs, FAUSTFLOAT** outputs) {
		FAUSTFLOAT* input0 = inputs[0];
		FAUSTFLOAT* input1 = inputs[1];
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = (0.00100000005f * std::pow(10.0f, (0.0500000007f * float(fVslider0))));
		float fSlow1 = (0.00100000005f * float(fVslider1));
		for (int i = 0; (i < count); i = (i + 1)) {
			fRec0[0] = (fSlow0 + (0.999000013f * fRec0[1]));
			float fTemp0 = float(input0[i]);
			fVec0[(IOTA & 16383)] = fTemp0;
			fRec1[0] = (fSlow1 + (0.999000013f * fRec1[1]));
			float fTemp1 = (fRec1[0] + 1.0f);
			float fTemp2 = (1.0f - (0.5f * fTemp1));
			fRec15[0] = (0.0f - (fConst18 * ((fConst19 * fRec15[1]) - (fRec11[1] + fRec11[2]))));
			fRec14[0] = ((fConst14 * fRec14[1]) + (fConst15 * (fRec11[1] + (fConst16 * fRec15[0]))));
			fVec1[(IOTA & 32767)] = ((0.353553385f * fRec14[0]) + 9.99999968e-21f);
			float fTemp3 = float(input1[i]);
			fVec2[(IOTA & 16383)] = fTemp3;
			float fTemp4 = (0.300000012f * fVec2[((IOTA - iConst22) & 16383)]);
			float fTemp5 = (((0.600000024f * fRec12[1]) + fVec1[((IOTA - iConst21) & 32767)]) - fTemp4);
			fVec3[(IOTA & 2047)] = fTemp5;
			fRec12[0] = fVec3[((IOTA - iConst23) & 2047)];
			float fRec13 = (0.0f - (0.600000024f * fTemp5));
			fRec19[0] = (0.0f - (fConst18 * ((fConst19 * fRec19[1]) - (fRec7[1] + fRec7[2]))));
			fRec18[0] = ((fConst32 * fRec18[1]) + (fConst33 * (fRec7[1] + (fConst34 * fRec19[0]))));
			fVec4[(IOTA & 32767)] = ((0.353553385f * fRec18[0]) + 9.99999968e-21f);
			float fTemp6 = (((0.600000024f * fRec16[1]) + fVec4[((IOTA - iConst36) & 32767)]) - fTemp4);
			fVec5[(IOTA & 4095)] = fTemp6;
			fRec16[0] = fVec5[((IOTA - iConst37) & 4095)];
			float fRec17 = (0.0f - (0.600000024f * fTemp6));
			fRec23[0] = (0.0f - (fConst18 * ((fConst19 * fRec23[1]) - (fRec9[1] + fRec9[2]))));
			fRec22[0] = ((fConst46 * fRec22[1]) + (fConst47 * (fRec9[1] + (fConst48 * fRec23[0]))));
			fVec6[(IOTA & 16383)] = ((0.353553385f * fRec22[0]) + 9.99999968e-21f);
			float fTemp7 = (fVec6[((IOTA - iConst50) & 16383)] + (fTemp4 + (0.600000024f * fRec20[1])));
			fVec7[(IOTA & 4095)] = fTemp7;
			fRec20[0] = fVec7[((IOTA - iConst51) & 4095)];
			float fRec21 = (0.0f - (0.600000024f * fTemp7));
			fRec27[0] = (0.0f - (fConst18 * ((fConst19 * fRec27[1]) - (fRec5[1] + fRec5[2]))));
			fRec26[0] = ((fConst60 * fRec26[1]) + (fConst61 * (fRec5[1] + (fConst62 * fRec27[0]))));
			fVec8[(IOTA & 32767)] = ((0.353553385f * fRec26[0]) + 9.99999968e-21f);
			float fTemp8 = (fTemp4 + ((0.600000024f * fRec24[1]) + fVec8[((IOTA - iConst64) & 32767)]));
			fVec9[(IOTA & 4095)] = fTemp8;
			fRec24[0] = fVec9[((IOTA - iConst65) & 4095)];
			float fRec25 = (0.0f - (0.600000024f * fTemp8));
			fRec31[0] = (0.0f - (fConst18 * ((fConst19 * fRec31[1]) - (fRec10[1] + fRec10[2]))));
			fRec30[0] = ((fConst74 * fRec30[1]) + (fConst75 * (fRec10[1] + (fConst76 * fRec31[0]))));
			fVec10[(IOTA & 16383)] = ((0.353553385f * fRec30[0]) + 9.99999968e-21f);
			float fTemp9 = (0.300000012f * fVec0[((IOTA - iConst22) & 16383)]);
			float fTemp10 = (fVec10[((IOTA - iConst78) & 16383)] - (fTemp9 + (0.600000024f * fRec28[1])));
			fVec11[(IOTA & 2047)] = fTemp10;
			fRec28[0] = fVec11[((IOTA - iConst79) & 2047)];
			float fRec29 = (0.600000024f * fTemp10);
			fRec35[0] = (0.0f - (fConst18 * ((fConst19 * fRec35[1]) - (fRec6[1] + fRec6[2]))));
			fRec34[0] = ((fConst88 * fRec34[1]) + (fConst89 * (fRec6[1] + (fConst90 * fRec35[0]))));
			fVec12[(IOTA & 16383)] = ((0.353553385f * fRec34[0]) + 9.99999968e-21f);
			float fTemp11 = (fVec12[((IOTA - iConst92) & 16383)] - (fTemp9 + (0.600000024f * fRec32[1])));
			fVec13[(IOTA & 4095)] = fTemp11;
			fRec32[0] = fVec13[((IOTA - iConst93) & 4095)];
			float fRec33 = (0.600000024f * fTemp11);
			fRec39[0] = (0.0f - (fConst18 * ((fConst19 * fRec39[1]) - (fRec8[1] + fRec8[2]))));
			fRec38[0] = ((fConst102 * fRec38[1]) + (fConst103 * (fRec8[1] + (fConst104 * fRec39[0]))));
			fVec14[(IOTA & 16383)] = ((0.353553385f * fRec38[0]) + 9.99999968e-21f);
			float fTemp12 = ((fTemp9 + fVec14[((IOTA - iConst106) & 16383)]) - (0.600000024f * fRec36[1]));
			fVec15[(IOTA & 4095)] = fTemp12;
			fRec36[0] = fVec15[((IOTA - iConst107) & 4095)];
			float fRec37 = (0.600000024f * fTemp12);
			fRec43[0] = (0.0f - (fConst18 * ((fConst19 * fRec43[1]) - (fRec4[1] + fRec4[2]))));
			fRec42[0] = ((fConst116 * fRec42[1]) + (fConst117 * (fRec4[1] + (fConst118 * fRec43[0]))));
			fVec16[(IOTA & 16383)] = ((0.353553385f * fRec42[0]) + 9.99999968e-21f);
			float fTemp13 = ((fVec16[((IOTA - iConst120) & 16383)] + fTemp9) - (0.600000024f * fRec40[1]));
			fVec17[(IOTA & 2047)] = fTemp13;
			fRec40[0] = fVec17[((IOTA - iConst121) & 2047)];
			float fRec41 = (0.600000024f * fTemp13);
			float fTemp14 = (fRec41 + fRec37);
			float fTemp15 = (fRec29 + (fRec33 + fTemp14));
			fRec4[0] = (fRec12[1] + (fRec16[1] + (fRec20[1] + (fRec24[1] + (fRec28[1] + (fRec32[1] + (fRec36[1] + (fRec40[1] + (fRec13 + (fRec17 + (fRec21 + (fRec25 + fTemp15))))))))))));
			fRec5[0] = ((fRec28[1] + (fRec32[1] + (fRec36[1] + (fRec40[1] + fTemp15)))) - (fRec12[1] + (fRec16[1] + (fRec20[1] + (fRec24[1] + (fRec13 + (fRec17 + (fRec25 + fRec21))))))));
			float fTemp16 = (fRec33 + fRec29);
			fRec6[0] = ((fRec20[1] + (fRec24[1] + (fRec36[1] + (fRec40[1] + (fRec21 + (fRec25 + fTemp14)))))) - (fRec12[1] + (fRec16[1] + (fRec28[1] + (fRec32[1] + (fRec13 + (fRec17 + fTemp16)))))));
			fRec7[0] = ((fRec12[1] + (fRec16[1] + (fRec36[1] + (fRec40[1] + (fRec13 + (fRec17 + fTemp14)))))) - (fRec20[1] + (fRec24[1] + (fRec28[1] + (fRec32[1] + (fRec21 + (fRec25 + fTemp16)))))));
			float fTemp17 = (fRec41 + fRec33);
			float fTemp18 = (fRec37 + fRec29);
			fRec8[0] = ((fRec16[1] + (fRec24[1] + (fRec32[1] + (fRec40[1] + (fRec17 + (fRec25 + fTemp17)))))) - (fRec12[1] + (fRec20[1] + (fRec28[1] + (fRec36[1] + (fRec13 + (fRec21 + fTemp18)))))));
			fRec9[0] = ((fRec12[1] + (fRec20[1] + (fRec32[1] + (fRec40[1] + (fRec13 + (fRec21 + fTemp17)))))) - (fRec16[1] + (fRec24[1] + (fRec28[1] + (fRec36[1] + (fRec17 + (fRec25 + fTemp18)))))));
			float fTemp19 = (fRec41 + fRec29);
			float fTemp20 = (fRec37 + fRec33);
			fRec10[0] = ((fRec12[1] + (fRec24[1] + (fRec28[1] + (fRec40[1] + (fRec13 + (fRec25 + fTemp19)))))) - (fRec16[1] + (fRec20[1] + (fRec32[1] + (fRec36[1] + (fRec17 + (fRec21 + fTemp20)))))));
			fRec11[0] = ((fRec16[1] + (fRec20[1] + (fRec28[1] + (fRec40[1] + (fRec17 + (fRec21 + fTemp19)))))) - (fRec12[1] + (fRec24[1] + (fRec32[1] + (fRec36[1] + (fRec13 + (fRec25 + fTemp20)))))));
			float fTemp21 = (0.370000005f * (fRec5[0] + fRec6[0]));
			float fTemp22 = (fConst122 * fRec3[1]);
			fRec3[0] = (fTemp21 - (fTemp22 + (fConst4 * fRec3[2])));
			float fTemp23 = (fConst4 * fRec3[0]);
			float fTemp24 = (0.5f * ((fTemp23 + (fRec3[2] + (fTemp21 + fTemp22))) + ((fTemp23 + (fTemp22 + fRec3[2])) - fTemp21)));
			float fTemp25 = (fConst123 * fRec2[1]);
			fRec2[0] = (fTemp24 - (fTemp25 + (fConst2 * fRec2[2])));
			float fTemp26 = (fConst2 * fRec2[0]);
			output0[i] = FAUSTFLOAT((0.5f * (fRec0[0] * ((fTemp0 * fTemp1) + (fTemp2 * ((fTemp26 + (fRec2[2] + (fTemp24 + fTemp25))) + ((fTemp26 + (fTemp25 + fRec2[2])) - fTemp24)))))));
			float fTemp27 = (0.370000005f * (fRec5[0] - fRec6[0]));
			float fTemp28 = (fConst122 * fRec45[1]);
			fRec45[0] = (fTemp27 - (fTemp28 + (fConst4 * fRec45[2])));
			float fTemp29 = (fConst4 * fRec45[0]);
			float fTemp30 = (0.5f * ((fTemp29 + (fRec45[2] + (fTemp27 + fTemp28))) + ((fTemp29 + (fTemp28 + fRec45[2])) - fTemp27)));
			float fTemp31 = (fConst123 * fRec44[1]);
			fRec44[0] = (fTemp30 - (fTemp31 + (fConst2 * fRec44[2])));
			float fTemp32 = (fConst2 * fRec44[0]);
			output1[i] = FAUSTFLOAT((0.5f * (fRec0[0] * ((fTemp3 * fTemp1) + (fTemp2 * ((fTemp32 + (fRec44[2] + (fTemp30 + fTemp31))) + ((fTemp32 + (fTemp31 + fRec44[2])) - fTemp30)))))));
			fRec0[1] = fRec0[0];
			IOTA = (IOTA + 1);
			fRec1[1] = fRec1[0];
			fRec15[1] = fRec15[0];
			fRec14[1] = fRec14[0];
			fRec12[1] = fRec12[0];
			fRec19[1] = fRec19[0];
			fRec18[1] = fRec18[0];
			fRec16[1] = fRec16[0];
			fRec23[1] = fRec23[0];
			fRec22[1] = fRec22[0];
			fRec20[1] = fRec20[0];
			fRec27[1] = fRec27[0];
			fRec26[1] = fRec26[0];
			fRec24[1] = fRec24[0];
			fRec31[1] = fRec31[0];
			fRec30[1] = fRec30[0];
			fRec28[1] = fRec28[0];
			fRec35[1] = fRec35[0];
			fRec34[1] = fRec34[0];
			fRec32[1] = fRec32[0];
			fRec39[1] = fRec39[0];
			fRec38[1] = fRec38[0];
			fRec36[1] = fRec36[0];
			fRec43[1] = fRec43[0];
			fRec42[1] = fRec42[0];
			fRec40[1] = fRec40[0];
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
			fRec3[2] = fRec3[1];
			fRec3[1] = fRec3[0];
			fRec2[2] = fRec2[1];
			fRec2[1] = fRec2[0];
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

            // Lightweight result access to prevent overoptimizations
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
    std::cout << "\n";
    std::cout << "Throughput min:    " << min / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput median: " << median / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput max:    " << max / 1024 / 1024 << " MB/sec" << std::endl;

    return 0;
}

#endif
