
use std::ffi::OsStr;
use std::path::Path;
use std::fs::{OpenOptions, create_dir};
use std::io::{Write,Result,Error,ErrorKind};

use inflector::Inflector;

use ttl::def::*;

pub fn generate(plugin: &Plugin, folder: &str) -> Result<()> {
    let path = Path::new(folder);
    if path.metadata().is_err() {
        create_dir(path);
    }
    if !path.metadata()?.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Target is not a folder."));
    }

    // write the manifest
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join("manifest.ttl"))?;
    file.write_all(&gen_manifest(&plugin).as_bytes())?;

    // write the plugin
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join(plugin.id.clone() + ".ttl"))?;
    file.write_all(&gen_plugin(&plugin).as_bytes())?;

    Ok(())
}

fn gen_manifest(plugin: &Plugin) -> String {
    // @prefix lines
    let mut s = String::new();
    for &prefix in &[PREFIX_LV2, PREFIX_RDFS, PREFIX_UI, PREFIX_KX] {
        s.push_str(&gen_prefix(prefix));
    }
    s.push('\n');

    // plugin
    s.push_str(&format!("<{uri}>
    a lv2:Plugin ;
    lv2:binary <lib{id_snake}.so> ;
    rdfs:seeAlso <{id}.ttl> .
", uri=plugin.uri, id_snake=plugin.id.to_snake_case(), id=plugin.id));

    s.push('\n');

    // plugin ui
    if let Some(ref ui) = plugin.ui {
        s.push_str(&format!("<{uri}#ui>
    a kx:Widget ;
    ui:binary <lib{id}_ui.so> ;
    rdfs:seeAlso <{id}.ttl> .
", uri=plugin.uri, id=plugin.id));
    }
    s
}

fn gen_plugin(plugin: &Plugin) -> String {
    // @prefix lines
    let mut s = String::new();
    for &prefix in &[PREFIX_DOAP, PREFIX_LV2, PREFIX_RDF, PREFIX_RDFS,
                     PREFIX_UNITS, PREFIX_ATOM, PREFIX_PATCH, PREFIX_STATE,
                     PREFIX_URID, PREFIX_WORK, PREFIX_PARAM, PREFIX_ALL] {
        s.push_str(&gen_prefix(prefix));
    }
    s.push('\n');

    // the plugin definitions
    let class = if let Some(ref cat) = plugin.category {
        format!("a lv2:Plugin ,\n        lv2:{} ;", cat)
    } else {
        format!("a lv2:Plugin ;")
    };
    s.push_str(&format!("<{uri}>
    {class}

    lv2:project <{uri}> ;

    doap:name \"{name}\" ;
    lv2:project <http://lv2plug.in/ns/lv2> ;
    lv2:requiredFeature urid:map ,
            work:schedule ;
    lv2:optionalFeature lv2:hardRTCapable ,
            state:loadDefaultState ;
    lv2:extensionData state:interface ,
            work:interface ;
", uri=plugin.uri, class=class, name=plugin.name));

    if plugin.ui.is_some() {
        s.push_str("    ui:ui <{uri}#ui> ;\n");
    }

    // plugin port definitions
    s.push_str("\n    lv2:port ");
    for (i, port) in plugin.ports.iter().enumerate() {
        if i > 0 {
            s.push_str(" , ");
        }
        s.push_str(&match &port.ty {
            &PortType::AudioPort => format!("[
        a lv2:{dir} ,
            lv2:{ty} ;
        lv2:index {i} ;
        lv2:symbol {symbol:?} ;
        lv2:name {name:?}
    ]", dir=port.direction, ty=port.ty, i=i, symbol=port.symbol, name=port.name),
            &PortType::ControlPort(ref s) => format!("[
        a lv2:{dir} ,
            lv2:{ty} ;
        lv2:index {i} ;
        lv2:symbol {symbol:?} ;
        lv2:name {name:?} ;

        lv2:default {def} ;
        lv2:minimum {min} ;
        lv2:maximum {max} ;
    ]", dir=port.direction, ty=port.ty, i=i, symbol=port.symbol, name=port.name, def=s.default, min=s.minimum, max=s.maximum),
        });
    }
    s.push_str(" .\n");

    // TODO: state

    s.push('\n');

    if let Some(ref ui) = plugin.ui {
        s.push_str(&format!("<{uri}#ui>
    a kx:Widget ;
    lv2:requiredFeature kx:Host .
", uri=plugin.uri));
    }
    s
}


const PREFIX_DOAP: (&str, &str) = ("doap", "http://usefulinc.com/ns/doap#");
const PREFIX_KX: (&str, &str) = ("kx", "http://kxstudio.sf.net/ns/lv2ext/external-ui#");
const PREFIX_LV2: (&str, &str) = ("lv2", "http://lv2plug.in/ns/lv2core#");
const PREFIX_RDF: (&str, &str) = ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
const PREFIX_RDFS: (&str, &str) = ("rdfs", "http://www.w3.org/2000/01/rdf-schema#");
const PREFIX_UI: (&str, &str) = ("ui", "http://lv2plug.in/ns/extensions/ui#");
const PREFIX_UNITS: (&str, &str) = ("units", "http://lv2plug.in/ns/extensions/units#");

const PREFIX_ATOM: (&str, &str) = ("atom", "http://lv2plug.in/ns/ext/atom#");
const PREFIX_PATCH: (&str, &str) = ("patch", "http://lv2plug.in/ns/ext/patch#");
const PREFIX_STATE: (&str, &str) = ("state", "http://lv2plug.in/ns/ext/state#");
const PREFIX_URID: (&str, &str) = ("urid", "http://lv2plug.in/ns/ext/urid#");
const PREFIX_WORK: (&str, &str) = ("work", "http://lv2plug.in/ns/ext/worker#");
const PREFIX_PARAM: (&str, &str) = ("param", "http://lv2plug.in/ns/ext/parameters#");
const PREFIX_ALL: (&str, &str) = ("all", "http://open-music-kontrollers.ch/lv2/alluis#");


fn gen_prefix((id, uri): (&str, &str)) -> String {
    format!("@prefix {}: <{}> .\n", id, uri)
}
