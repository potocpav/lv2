
use std::fmt;

#[derive(Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub category: Option<Category>,

    pub ports: Vec<Port>,
    pub ui: Option<PluginUI>,
}

#[derive(Clone)]
pub enum Category {
    AmplifierPlugin,
}

#[derive(Clone)]
pub struct Port {
    pub direction: Direction,
    pub ty: PortType,
    pub symbol: String,
    pub name: String,
}

#[derive(Clone)]
pub enum Direction {
    Input, Output,
}

#[derive(Clone)]
pub enum PortType {
    ControlPort(ControlPortSpec),
    AudioPort,
}

#[derive(Clone)]
pub struct ControlPortSpec {
    pub default: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub scale_points: Vec<ScalePoint>,
}

#[derive(Clone)]
pub struct ScalePoint {
    pub label: String,
    pub value: f32,
}

#[derive(Clone)]
pub struct PluginUI {

}


impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Category::AmplifierPlugin => "AmplifierPlugin",
        })
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Direction::Input => "InputPort",
            &Direction::Output => "OutputPort",
        })
    }
}

// TODO: use a different trait: Display is reserved for unique mappings AFAIK
impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &PortType::AudioPort => "AudioPort",
            &PortType::ControlPort(_) => "ControlPort",
        })
    }
}
