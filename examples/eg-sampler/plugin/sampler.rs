
#[macro_use]
extern crate lv2;

// port numbers
const GAIN: usize = 0;
const INPUT: usize = 1;
const OUTPUT: usize = 2;


pub struct Sampler;

impl lv2::Plugin for Sampler {
    fn initialize() -> Self {
        Sampler
    }

    fn run(&mut self, buffers: &mut [&mut [f32]]) {
        let gain = buffers[GAIN][0];
        let coef = if gain > -90.0 {
                (10.0 as f32).powf(gain * 0.05)
            } else {
                0.0
            };
        for i in 0..buffers[OUTPUT].len() {
            buffers[OUTPUT][i] = buffers[INPUT][i] * coef;
        }
    }
}

plugin!(Sampler, b"http://example.org/eg-sampler-rs\0");
