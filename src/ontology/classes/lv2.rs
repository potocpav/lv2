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

class!(
/// **lv2:AmplifierPlugin**
:"http://lv2plug.in/ns/lv2core#AmplifierPlugin", AmplifierPlugin,
14);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for AmplifierPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, AmplifierPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:AnalyserPlugin**
/// Any plugin that analyses input to output some useful information.
:"http://lv2plug.in/ns/lv2core#AnalyserPlugin", AnalyserPlugin,
15);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for AnalyserPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, AnalyserPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:CombPlugin**
:"http://lv2plug.in/ns/lv2core#CombPlugin", CombPlugin,
21);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for CombPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, CombPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:CompressorPlugin**
:"http://lv2plug.in/ns/lv2core#CompressorPlugin", CompressorPlugin,
22);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for CompressorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, CompressorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ConstantPlugin**
:"http://lv2plug.in/ns/lv2core#ConstantPlugin", ConstantPlugin,
23);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ConstantPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ConstantPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:DelayPlugin**
/// Plugins that intentionally delay their input signal as an effect.
:"http://lv2plug.in/ns/lv2core#DelayPlugin", DelayPlugin,
26);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for DelayPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, DelayPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:DynamicsPlugin**
/// Plugins that alter the envelope or dynamic range of audio.
:"http://lv2plug.in/ns/lv2core#DynamicsPlugin", DynamicsPlugin,
29);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for DynamicsPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, DynamicsPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:EQPlugin**
:"http://lv2plug.in/ns/lv2core#EQPlugin", EQPlugin,
30);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for EQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, EQPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:EnvelopePlugin**
:"http://lv2plug.in/ns/lv2core#EnvelopePlugin", EnvelopePlugin,
31);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for EnvelopePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, EnvelopePlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ExpanderPlugin**
:"http://lv2plug.in/ns/lv2core#ExpanderPlugin", ExpanderPlugin,
32);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ExpanderPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ExpanderPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:FlangerPlugin**
:"http://lv2plug.in/ns/lv2core#FlangerPlugin", FlangerPlugin,
36);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for FlangerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, FlangerPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:FunctionPlugin**
:"http://lv2plug.in/ns/lv2core#FunctionPlugin", FunctionPlugin,
37);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for FunctionPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, FunctionPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:GatePlugin**
:"http://lv2plug.in/ns/lv2core#GatePlugin", GatePlugin,
38);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for GatePlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, GatePlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:GeneratorPlugin**
/// Any plugin that generates sound internally, rather than processing its input.
:"http://lv2plug.in/ns/lv2core#GeneratorPlugin", GeneratorPlugin,
39);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for GeneratorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, GeneratorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:HighpassPlugin**
:"http://lv2plug.in/ns/lv2core#HighpassPlugin", HighpassPlugin,
40);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for HighpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, HighpassPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:LimiterPlugin**
:"http://lv2plug.in/ns/lv2core#LimiterPlugin", LimiterPlugin,
43);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for LimiterPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, LimiterPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:LowpassPlugin**
:"http://lv2plug.in/ns/lv2core#LowpassPlugin", LowpassPlugin,
44);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for LowpassPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, LowpassPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MIDIPlugin**
:"http://lv2plug.in/ns/lv2core#MIDIPlugin", MIDIPlugin,
45);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MIDIPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MIDIPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MixerPlugin**
/// A plugin which mixes some number of inputs into some number of outputs.
:"http://lv2plug.in/ns/lv2core#MixerPlugin", MixerPlugin,
46);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MixerPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MixerPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:ModulatorPlugin**
:"http://lv2plug.in/ns/lv2core#ModulatorPlugin", ModulatorPlugin,
47);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for ModulatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, ModulatorPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:MultiEQPlugin**
:"http://lv2plug.in/ns/lv2core#MultiEQPlugin", MultiEQPlugin,
48);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for MultiEQPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, MultiEQPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:OscillatorPlugin**
:"http://lv2plug.in/ns/lv2core#OscillatorPlugin", OscillatorPlugin,
49);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for OscillatorPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, OscillatorPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:PitchPlugin**
:"http://lv2plug.in/ns/lv2core#PitchPlugin", PitchPlugin,
54);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for PitchPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, PitchPlugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:Plugin**
:"http://lv2plug.in/ns/lv2core#Plugin", Plugin,
55);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for Plugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, Plugin<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **lv2:PluginBase**
:"http://lv2plug.in/ns/lv2core#PluginBase", PluginBase,
56);
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for PluginBase<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, PluginBase<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:SpatialPlugin**
/// Plugins that manipulate the position of audio in space (e.g. panning,stereo width, surround encoding, etc.).
:"http://lv2plug.in/ns/lv2core#SpatialPlugin", SpatialPlugin,
63);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for SpatialPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, SpatialPlugin<'g, G>> where G: graph::Graph<'g> {}

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

class!(
/// **lv2:WaveshaperPlugin**
:"http://lv2plug.in/ns/lv2core#WaveshaperPlugin", WaveshaperPlugin,
68);
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Ui<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for WaveshaperPlugin<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::lv2::Port<'g> for resource::IRI<'g, WaveshaperPlugin<'g, G>> where G: graph::Graph<'g> {}
