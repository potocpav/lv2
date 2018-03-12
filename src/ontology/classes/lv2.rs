use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **lv2:AllpassPlugin**
:"http://lv2plug.in/ns/lv2core#AllpassPlugin", AllpassPlugin,
13);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AllpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AllpassPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:AmplifierPlugin**
:"http://lv2plug.in/ns/lv2core#AmplifierPlugin", AmplifierPlugin,
14);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:AnalyserPlugin**
/// Any plugin that analyses input to output some useful information.
:"http://lv2plug.in/ns/lv2core#AnalyserPlugin", AnalyserPlugin,
15);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:AudioPort**
:"http://lv2plug.in/ns/lv2core#AudioPort", AudioPort,
16);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for AudioPort<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, AudioPort<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:BandpassPlugin**
:"http://lv2plug.in/ns/lv2core#BandpassPlugin", BandpassPlugin,
17);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for BandpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, BandpassPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:CVPort**
:"http://lv2plug.in/ns/lv2core#CVPort", CVPort,
18);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for CVPort<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, CVPort<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Channel**
:"http://lv2plug.in/ns/lv2core#Channel", Channel,
19);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Channel<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Channel<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ChorusPlugin**
:"http://lv2plug.in/ns/lv2core#ChorusPlugin", ChorusPlugin,
20);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ChorusPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ChorusPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:CombPlugin**
:"http://lv2plug.in/ns/lv2core#CombPlugin", CombPlugin,
21);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:CompressorPlugin**
:"http://lv2plug.in/ns/lv2core#CompressorPlugin", CompressorPlugin,
22);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ConstantPlugin**
:"http://lv2plug.in/ns/lv2core#ConstantPlugin", ConstantPlugin,
23);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ControlPort**
:"http://lv2plug.in/ns/lv2core#ControlPort", ControlPort,
24);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for ControlPort<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, ControlPort<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ConverterPlugin**
/// Any plugin that converts some form of input into a different form of output.
:"http://lv2plug.in/ns/lv2core#ConverterPlugin", ConverterPlugin,
25);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ConverterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ConverterPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:DelayPlugin**
/// Plugins that intentionally delay their input signal as an effect.
:"http://lv2plug.in/ns/lv2core#DelayPlugin", DelayPlugin,
26);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Designation**
:"http://lv2plug.in/ns/lv2core#Designation", Designation,
27);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Designation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Designation<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:DistortionPlugin**
:"http://lv2plug.in/ns/lv2core#DistortionPlugin", DistortionPlugin,
28);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DistortionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DistortionPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:DynamicsPlugin**
/// Plugins that alter the envelope or dynamic range of audio.
:"http://lv2plug.in/ns/lv2core#DynamicsPlugin", DynamicsPlugin,
29);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:EQPlugin**
:"http://lv2plug.in/ns/lv2core#EQPlugin", EQPlugin,
30);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:EnvelopePlugin**
:"http://lv2plug.in/ns/lv2core#EnvelopePlugin", EnvelopePlugin,
31);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ExpanderPlugin**
:"http://lv2plug.in/ns/lv2core#ExpanderPlugin", ExpanderPlugin,
32);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ExtensionData**
:"http://lv2plug.in/ns/lv2core#ExtensionData", ExtensionData,
33);

class!(
/// **lv2:Feature**
/// An additional feature which a plugin or other resource may use or require.
:"http://lv2plug.in/ns/lv2core#Feature", Feature,
34);

class!(
/// **lv2:FilterPlugin**
:"http://lv2plug.in/ns/lv2core#FilterPlugin", FilterPlugin,
35);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for FilterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, FilterPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:FlangerPlugin**
:"http://lv2plug.in/ns/lv2core#FlangerPlugin", FlangerPlugin,
36);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:FunctionPlugin**
:"http://lv2plug.in/ns/lv2core#FunctionPlugin", FunctionPlugin,
37);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:GatePlugin**
:"http://lv2plug.in/ns/lv2core#GatePlugin", GatePlugin,
38);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:GeneratorPlugin**
/// Any plugin that generates sound internally, rather than processing its input.
:"http://lv2plug.in/ns/lv2core#GeneratorPlugin", GeneratorPlugin,
39);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:HighpassPlugin**
:"http://lv2plug.in/ns/lv2core#HighpassPlugin", HighpassPlugin,
40);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:InputPort**
/// Ports of this type will be connected to a pointer to some value, which will be read by the plugin during their run method.
:"http://lv2plug.in/ns/lv2core#InputPort", InputPort,
41);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for InputPort<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, InputPort<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:InstrumentPlugin**
/// Any plugin that is intended to be played as a musical instrument.
:"http://lv2plug.in/ns/lv2core#InstrumentPlugin", InstrumentPlugin,
42);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for InstrumentPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, InstrumentPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:LimiterPlugin**
:"http://lv2plug.in/ns/lv2core#LimiterPlugin", LimiterPlugin,
43);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:LowpassPlugin**
:"http://lv2plug.in/ns/lv2core#LowpassPlugin", LowpassPlugin,
44);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MIDIPlugin**
:"http://lv2plug.in/ns/lv2core#MIDIPlugin", MIDIPlugin,
45);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MixerPlugin**
/// A plugin which mixes some number of inputs into some number of outputs.
:"http://lv2plug.in/ns/lv2core#MixerPlugin", MixerPlugin,
46);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ModulatorPlugin**
:"http://lv2plug.in/ns/lv2core#ModulatorPlugin", ModulatorPlugin,
47);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MultiEQPlugin**
:"http://lv2plug.in/ns/lv2core#MultiEQPlugin", MultiEQPlugin,
48);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:OscillatorPlugin**
:"http://lv2plug.in/ns/lv2core#OscillatorPlugin", OscillatorPlugin,
49);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:OutputPort**
/// Ports of this type will be connected to a pointer to some value, which will be written to by the plugin during their run method.
:"http://lv2plug.in/ns/lv2core#OutputPort", OutputPort,
50);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for OutputPort<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, OutputPort<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ParaEQPlugin**
:"http://lv2plug.in/ns/lv2core#ParaEQPlugin", ParaEQPlugin,
51);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ParaEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ParaEQPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Parameter**
:"http://lv2plug.in/ns/lv2core#Parameter", Parameter,
52);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Parameter<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Parameter<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:PhaserPlugin**
:"http://lv2plug.in/ns/lv2core#PhaserPlugin", PhaserPlugin,
53);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PhaserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PhaserPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:PitchPlugin**
:"http://lv2plug.in/ns/lv2core#PitchPlugin", PitchPlugin,
54);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Plugin**
:"http://lv2plug.in/ns/lv2core#Plugin", Plugin,
55);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:PluginBase**
:"http://lv2plug.in/ns/lv2core#PluginBase", PluginBase,
56);
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Point**
:"http://lv2plug.in/ns/lv2core#Point", Point,
57);

class!(
/// **lv2:Port**
:"http://lv2plug.in/ns/lv2core#Port", Port,
58);
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for Port<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::PortProperty<'g> for resource::IRI<'g, Port<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:PortProperty**
/// A property of this port that allows a host to make more sensible decisions (e.g. to provide a better interface).
:"http://lv2plug.in/ns/lv2core#PortProperty", PortProperty,
59);

class!(
/// **lv2:ReverbPlugin**
:"http://lv2plug.in/ns/lv2core#ReverbPlugin", ReverbPlugin,
60);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ReverbPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ReverbPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ScalePoint**
/// A single float Point (for control inputs).
:"http://lv2plug.in/ns/lv2core#ScalePoint", ScalePoint,
61);

class!(
/// **lv2:SimulatorPlugin**
/// Plugins that aim to duplicate the effect of some environmental effect or musical equipment.
:"http://lv2plug.in/ns/lv2core#SimulatorPlugin", SimulatorPlugin,
62);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for SimulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, SimulatorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:SpatialPlugin**
/// Plugins that manipulate the position of audio in space (e.g. panning,stereo width, surround encoding, etc.).
:"http://lv2plug.in/ns/lv2core#SpatialPlugin", SpatialPlugin,
63);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Specification**
:"http://lv2plug.in/ns/lv2core#Specification", Specification,
64);
impl<'g, G: 'g> ontology::properties::doap::Audience<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Audience<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Blog<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Blog<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Developer<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Developer<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Documenter<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Documenter<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Helper<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Helper<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Implements<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Implements<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Language<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Language<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Maintainer<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Maintainer<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Programming_language<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Programming_language<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Release<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Release<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Repository<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Repository<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Service_endpoint<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Service_endpoint<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Tester<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Tester<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Translator<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Translator<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Vendor<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Vendor<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:SpectralPlugin**
/// Plugins that alter the spectral properties (e.g. frequency) of audio.
:"http://lv2plug.in/ns/lv2core#SpectralPlugin", SpectralPlugin,
65);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for SpectralPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, SpectralPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Symbol**
/// A short restricted name used as a machine and human readable identifier.The first character must be one of _, a-z or A-Z and subsequent characters can be from _, a-z, A-Z and 0-9.  This is a valid C identifier, and compatible in most other contexts with restricted string identifiers (e.g. file paths).
:"http://lv2plug.in/ns/lv2core#Symbol", Symbol,
66);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Symbol<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Symbol<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:UtilityPlugin**
/// Includes things like mathematical functions and non-musical delays.
:"http://lv2plug.in/ns/lv2core#UtilityPlugin", UtilityPlugin,
67);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UtilityPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UtilityPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:WaveshaperPlugin**
:"http://lv2plug.in/ns/lv2core#WaveshaperPlugin", WaveshaperPlugin,
68);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
