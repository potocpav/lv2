
use ttl::def::*;

use inflector::Inflector;

pub struct PluginBuilder {
    p: Plugin,
}

impl PluginBuilder {
    pub fn new(id: &str) -> PluginBuilder {
        PluginBuilder { p: Plugin {
            id: id.to_string(),
            name: id.to_string(),
            uri: "http://example.org/".to_string() + id,
            category: None,
            ports: Vec::new(),
            ui: None,
        } }
    }

    pub fn name(&mut self, name_: &str) -> &mut Self {
        self.p.name = name_.to_string();
        self
    }

    pub fn uri(&mut self, uri_: &str) -> &mut Self {
        self.p.uri = uri_.to_string();
        self
    }

    pub fn category(&mut self, category_: Category) -> &mut Self {
        self.p.category = Some(category_);
        self
    }

    pub fn ui(&mut self, ui_: PluginUI) -> &mut Self {
        self.p.ui = Some(ui_);
        self
    }

    pub fn input_port(&mut self, name: &str) -> &mut Self {
        let port = Port {
            direction: Direction::Input,
            ty: PortType::AudioPort,
            symbol: name.to_snake_case(),
            name: name.to_title_case(),
        };
        self.p.ports.push(port);
        self
    }

    pub fn output_port(&mut self, name: &str) -> &mut Self {
        let port = Port {
            direction: Direction::Output,
            ty: PortType::AudioPort,
            symbol: name.to_snake_case(),
            name: name.to_title_case(),
        };
        self.p.ports.push(port);
        self
    }

    pub fn control_port(&mut self, name: &str, default: f32, minimum: f32, maximum: f32) -> &mut Self {
        let spec = ControlPortSpec {
            default, minimum, maximum,
            scale_points: Vec::new(),
        };
        let port = Port {
            direction: Direction::Input,
            ty: PortType::ControlPort(spec),
            symbol: name.to_snake_case(),
            name: name.to_title_case(),
        };
        self.p.ports.push(port);
        self
    }

    pub fn finish(&self) -> Result<Plugin, String> {
        if self.p.ports.len() == 0 {
            return Err("At least one port must be specified.".into());
        }
        Ok(self.p.clone())
    }
}
