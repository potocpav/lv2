
use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

pub struct ConrodWindow {
    pub events_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub ui: conrod::Ui,
    pub renderer: conrod::backend::glium::Renderer,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
}
/*
impl Drop for ConrodWindow {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}*/

impl ConrodWindow {
    pub fn new(title: &str) -> ConrodWindow {
        const WIDTH: u32 = 600;
        const HEIGHT: u32 = 300;

        // Build the window.
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(WIDTH, HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // construct our `Ui`.
        let ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        ConrodWindow {
            events_loop,
            display,
            ui,
            image_map,
            renderer,
        }
    }

    pub fn render(&mut self) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
