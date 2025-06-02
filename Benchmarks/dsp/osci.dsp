import("stdfaust.lib");

envelope(gate) = gate : en.adsr(0.01, 0.01, 0.3, 0.1);
synth(gate, freq) = envelope(gate) * os.osc(freq);

process = synth;
