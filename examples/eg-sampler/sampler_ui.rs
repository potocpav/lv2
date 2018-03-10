
#[macro_use]
extern crate lv2;
#[macro_use]
extern crate conrod;

mod conrod_window;
use conrod_window::ConrodWindow;


pub struct SamplerUI {
    pub window: ConrodWindow,
    pub ids: Ids,
    pub gain: f32,
}

impl lv2::PluginUI for SamplerUI {
    fn instantiate() -> Self {
        eprintln!("instantiate");
        let mut window = ConrodWindow::new();
        SamplerUI {
            ids: Ids::new(window.ui.widget_id_generator()),
            window,
            gain: 50.0,
        }
    }

    fn show(&mut self) {
        eprintln!("show");
    }

    fn hide(&mut self) {
        eprintln!("hide");
    }

    fn run(&mut self) {
        for event in self.window.events() {
            match event {
                _ => (),
            }
        }

        set_ui(&mut self.gain, self.window.ui.set_widgets(), &self.ids);
        self.window.render();
    }
}

plugin_ui!(SamplerUI, b"http://example.org/eg-sampler-rs#ui\0");

// Generate a type that will produce a unique `widget::Id` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        slider,
    }
}


fn set_ui(gain: &mut f32, ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::{Positionable, Labelable, Widget};
    use conrod::position::Sizeable;
    use conrod::widget::{Canvas, Slider};

    // The background canvas upon which we'll place our widgets.
    Canvas::new().pad(80.0).set(ids.canvas, ui);

    for event in Slider::new(*gain, 0.0, 100.0)
            .padded_w_of(ids.canvas, 80.0)
            .h(20.0)
            .top_left_of(ids.canvas)
            // .parent(ids.canvas)
            .label("Gain")
            .set(ids.slider, ui) {
        *gain = event;
        eprintln!("Hello, slider!");
    }
}
