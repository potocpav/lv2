
#[macro_use]
extern crate lv2;
#[macro_use]
extern crate conrod;

mod conrod_window;
use conrod_window::ConrodWindow;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

pub struct SamplerUI {
    window: ConrodWindow,
    ids: Ids,
}

impl lv2::PluginUI for SamplerUI {
    fn instantiate() -> Self {
        let mut window = ConrodWindow::new();
        SamplerUI {
            ids: Ids::new(window.ui.widget_id_generator()),
            window,
        }
    }

    fn show(&mut self) {

    }

    fn hide(&mut self) {
        eprintln!("hide");
    }

    fn idle(&mut self) {
        for event in self.window.events() {
            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    _ => (),
                },
                _ => (),
            }
        }

        set_ui(self.window.ui.set_widgets(), &self.ids);

        self.window.render();
    }
}

plugin_ui!(SamplerUI, b"http://example.org/eg-sampler-rs#ui\0");

// Generate a type that will produce a unique `widget::Id` for each widget.
widget_ids! {
    struct Ids {
        canvas,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
    }
}


fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::{Positionable, Widget};
    use conrod::widget::{Canvas, Circle, Line, Oval, PointPath, Polygon, Rectangle};
    use std::iter::once;

    // The background canvas upon which we'll place our widgets.
    Canvas::new().pad(80.0).set(ids.canvas, ui);

    Line::centred([-40.0, -40.0], [40.0, 40.0]).top_left_of(ids.canvas).set(ids.line, ui);

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    PointPath::centred(points).down(80.0).set(ids.point_path, ui);

    Rectangle::fill([80.0, 80.0]).down(80.0).set(ids.rectangle_fill, ui);

    Rectangle::outline([80.0, 80.0]).down(80.0).set(ids.rectangle_outline, ui);

    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    Polygon::centred_fill(points).right_from(ids.line, 80.0).set(ids.trapezoid, ui);

    Oval::fill([40.0, 80.0]).down(80.0).align_middle_x().set(ids.oval_fill, ui);

    Oval::outline([80.0, 40.0]).down(100.0).align_middle_x().set(ids.oval_outline, ui);

    Circle::fill(40.0).down(100.0).align_middle_x().set(ids.circle, ui);
}
