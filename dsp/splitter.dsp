import("stdfaust.lib");

panner = _ : *(level) <: *(1-g), *(g)
with {
    level = hslider("level", 0.5, 0.0, 1.0, 0.01);
    g = hslider("pan", 0.5, 0.0, 1.0, 0.01);
};

process = panner;
