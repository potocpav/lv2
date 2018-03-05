// This is a re-implementation of http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/amp.c
// from C into Rust by S. Riha (2015)
// Read the README.txt of the original code:
// http://lv2plug.in/git/cgit.cgi/lv2.git/tree/plugins/eg-amp.lv2/README.txt

#[macro_use]
extern crate lv2;

// port numbers
const GAIN: usize = 0;
const INPUT: usize = 1;
const OUTPUT: usize = 2;

pub struct AmpNew;

impl lv2::Plugin for AmpNew {
    fn initialize() -> Self {
        AmpNew
    }

    fn activate(&mut self) {}

    fn run(&mut self, buffers: &mut [&mut [f32]]) {
        let gain = buffers[GAIN][0];
        let coef = if gain > -90.0 {
                (10.0 as f32).powf(gain * 0.05)
            } else {
                0.0
            };
        for x in 0..buffers[OUTPUT].len() {
            let i = x as usize;
            buffers[OUTPUT][i] = buffers[INPUT][i] * coef;
        }
    }

    fn deactivate(&mut self) {}

    fn cleanup(&mut self) {}
}

plugin!(AmpNew, b"http://example.org/eg-amp-rust\0");
