
use rome;
use rome::graphs::tel;
use rome::graph::{BlankNodeOrIRI, GraphWriter, Graph, Triple, IRIPtr, Resource, LiteralPtr};
use rome::io::TurtleParser;

use ontology;

use std::fs;
use std::io;
use std::io::Read;

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn load_file(input: &str) -> rome::Result<tel::Graph128> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    let data = read_file(input)?;
    let mut base = String::from("file:");
    base.push_str(input);
    {
        let mut triples = TurtleParser::new(data.as_str(), &base, &mut writer)?;
        while let Some(step) = triples.next() {
            step?;
        }
    }
    let graph = writer.collect();
    Ok(graph)
}

fn get_subject_by_type<'a, G: Graph<'a>>(graph: &'a G, ty: &str) -> Option<<G as Graph<'a>>::IRIPtr> {
    let iri = graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap();
    let iri_plugin = graph.find_iri(ty).unwrap();
    graph.iter_o_p(&Resource::IRI(iri_plugin), &iri)
         .filter_map(|triple| triple.subject().as_iri().cloned()).next()
}

fn get_see_also<'a, G: Graph<'a>>(graph: &'a G, subject: <G as Graph<'a>>::IRIPtr) -> Option<<G as Graph<'a>>::IRIPtr> {
    let pred = graph.find_iri("http://www.w3.org/2000/01/rdf-schema#seeAlso")?;
    graph.iter_s_p(&BlankNodeOrIRI::IRI(subject), &pred)
         .filter_map(|triple| triple.object().as_iri().cloned()).next()
}

fn print_triples<'a, G: Graph<'a>>(graph: &'a G) {
    for triple in graph.iter() {
        let subject = match triple.subject() {
            BlankNodeOrIRI::BlankNode(..) => String::from("blank"),
            BlankNodeOrIRI::IRI(i) => String::from(i.as_str()),
        };
        let predicate = String::from(triple.predicate().as_str());
        let object = match triple.object() {
            Resource::BlankNode(..) => String::from("blank"),
            Resource::IRI(i) => String::from(i.as_str()),
            Resource::Literal(l) => format!("str: {:?}, data: {:?}, lang: {:?}", l.as_str(), l.datatype_str(), l.language()),
        };
        println!("subject: {:?}, pred: {:?}, object: {:?}", subject, predicate, object);
    }
}

#[test]
fn test() {
    let graph = load_file("examples/eg-sampler/manifest.ttl").unwrap();
    print_triples(&graph);

    let plugin_iri = get_subject_by_type(&graph, "http://lv2plug.in/ns/lv2core#Plugin").unwrap();
    let plugin_ui_iri = get_subject_by_type(&graph, "http://kxstudio.sf.net/ns/lv2ext/external-ui#Widget").unwrap();
    println!("PLUGIN: {:?}, UI: {:?}", plugin_iri.as_str(), plugin_ui_iri.as_str());
    println!("seeAlso: {:?}", get_see_also(&graph, plugin_iri).unwrap().as_str());

    let adapter = ontology::adapter(&graph);

    let graph = load_file("examples/eg-sampler/sampler.ttl").unwrap();
    print_triples(&graph);

    panic!()
}
