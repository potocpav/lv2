use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **lv2:appliesTo**
:"http://lv2plug.in/ns/lv2core#appliesTo", AppliesTo, applies_to,
ontology::classes::lv2::Plugin<'g, G>,
174);

property!(
/// **lv2:binary**
:"http://lv2plug.in/ns/lv2core#binary", Binary, binary,
ontology::classes::owl::Thing<'g, G>,
175);

property!(
/// **lv2:designation**
:"http://lv2plug.in/ns/lv2core#designation", Designation, designation,
ontology::classes::rdf::Property<'g, G>,
177);

property!(
/// **lv2:documentation**
:"http://lv2plug.in/ns/lv2core#documentation", Documentation, documentation,
ontology::classes::rdfs::Literal<'g, G>,
178);

property!(
/// **lv2:enabled**
:"http://lv2plug.in/ns/lv2core#enabled", Enabled, enabled,
ontology::classes::xsd::Int<'g, G>,
179);

property!(
/// **lv2:extensionData**
:"http://lv2plug.in/ns/lv2core#extensionData", ExtensionData, extension_data,
ontology::classes::lv2::ExtensionData<'g, G>,
180);

property!(
/// **lv2:freeWheeling**
:"http://lv2plug.in/ns/lv2core#freeWheeling", FreeWheeling, free_wheeling,
ontology::classes::xsd::Boolean<'g, G>,
181);

property!(
/// **lv2:index**
/// A non-negative zero-based 32-bit index.
:"http://lv2plug.in/ns/lv2core#index", Index, index,
ontology::classes::xsd::UnsignedInt<'g, G>,
182);

property!(
/// **lv2:latency**
/// The latency introduced, in frames.
:"http://lv2plug.in/ns/lv2core#latency", Latency, latency,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
183);

property!(
/// **lv2:microVersion**
:"http://lv2plug.in/ns/lv2core#microVersion", MicroVersion, micro_version,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
185);

property!(
/// **lv2:minorVersion**
:"http://lv2plug.in/ns/lv2core#minorVersion", MinorVersion, minor_version,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
187);

property!(
/// **lv2:name**
/// A display name for labeling in a user interface.  Unlike lv2:symbol this is unrestricted and may be translated.  The lv2:name MUST NOT be used as an identifier.This property is required for Ports, but MUST NOT be used by the host for port identification. The plugin author may change the values of this property without changing the Plugin URI.
:"http://lv2plug.in/ns/lv2core#name", Name, name,
ontology::classes::xsd::String<'g, G>,
188);

property!(
/// **lv2:optionalFeature**
:"http://lv2plug.in/ns/lv2core#optionalFeature", OptionalFeature, optional_feature,
ontology::classes::lv2::Feature<'g, G>,
189);

property!(
/// **lv2:port**
/// A port (input or output) on this plugin.
:"http://lv2plug.in/ns/lv2core#port", Port, port,
ontology::classes::lv2::Port<'g, G>,
190);

property!(
/// **lv2:portProperty**
/// Relates Ports to PortProperties. The PortProperty may be ignored without catastrophic effects, though it may be useful e.g. for providing a sensible interface for the port.
:"http://lv2plug.in/ns/lv2core#portProperty", PortProperty, port_property,
ontology::classes::lv2::PortProperty<'g, G>,
191);

property!(
/// **lv2:project**
:"http://lv2plug.in/ns/lv2core#project", Project, project,
ontology::classes::doap::Project<'g, G>,
192);

property!(
/// **lv2:requiredFeature**
:"http://lv2plug.in/ns/lv2core#requiredFeature", RequiredFeature, required_feature,
ontology::classes::lv2::Feature<'g, G>,
194);

property!(
/// **lv2:scalePoint**
/// A scale point of a port or parameter.
:"http://lv2plug.in/ns/lv2core#scalePoint", ScalePoint, scale_point,
ontology::classes::lv2::ScalePoint<'g, G>,
195);

property!(
/// **lv2:shortName**
/// A short display name for labeling in a user interface.The same rules for port names apply here, with the exception that short names should not be longer than 16 characters.
:"http://lv2plug.in/ns/lv2core#shortName", ShortName, short_name,
ontology::classes::xsd::String<'g, G>,
196);

property!(
/// **lv2:symbol**
:"http://lv2plug.in/ns/lv2core#symbol", Symbol, symbol,
ontology::classes::lv2::Symbol<'g, G>,
197);
