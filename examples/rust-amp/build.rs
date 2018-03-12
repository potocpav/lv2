
extern crate lv2;

use lv2::ttl::*;

fn main() {
    let plugin = PluginBuilder::new("rust-amp")
        .name("A Simple Rust Amplifier")
        .category(Category::AmplifierPlugin)
        .input_port("In")
        .output_port("Out")
        .finish().unwrap();

    lv2::ttl::generate(&plugin, "rust-amp.lv2").unwrap();
}
