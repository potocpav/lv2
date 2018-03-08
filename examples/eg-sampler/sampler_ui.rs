
#[macro_use]
extern crate lv2;

pub struct SamplerUI;

impl lv2::PluginUI for SamplerUI {
    fn instantiate() -> Self {
        SamplerUI
    }

    fn show(&mut self) {
        eprintln!("show");
    }

    fn hide(&mut self) {
        eprintln!("hide");
    }

    fn idle(&mut self) {
        eprintln!("idle");
    }
}

plugin_ui!(SamplerUI, b"http://example.org/eg-sampler-rs#ui\0");
