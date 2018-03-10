
#[macro_use]
extern crate lv2;
#[macro_use]
extern crate conrod;

mod conrod_window;

use conrod::backend::glium::glium;
use conrod_window::ConrodWindow;


pub struct SamplerUI {
    pub window: ConrodWindow,
    pub ids: Ids,
    pub gain: f32,
}

impl lv2::PluginUI for SamplerUI {
    fn instantiate(human_id: &str) -> Self {
        eprintln!("instantiate {:?}", human_id);
        let mut window = ConrodWindow::new(human_id);
        SamplerUI {
            ids: Ids::new(window.ui.widget_id_generator()),
            window,
            gain: 0.0,
        }
    }

    fn show(&mut self) {
        eprintln!("show");
    }

    fn hide(&mut self) {
        eprintln!("hide");
    }

    fn run<F: Fn(u32, f32)>(&mut self, send_float: F) -> bool {
        let mut open = true;
        {
        let ref mut ui = self.window.ui;
        let ref display = self.window.display;
        self.window.events_loop.poll_events(|event| {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), display) {
                ui.handle_event(event);
            }
            if let glium::glutin::Event::WindowEvent { event: glium::glutin::WindowEvent::Closed, .. } = event {
                eprintln!("closing...");
                open = false;
            }
        });
    }

        set_ui(&mut self.gain, self.window.ui.set_widgets(), &self.ids);

        send_float(0, self.gain);

        self.window.render();
        open
    }

    fn cleanup(&mut self) {
        eprintln!("Cleanup");
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

    for event in Slider::new(*gain, -90.0, 24.0)
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
