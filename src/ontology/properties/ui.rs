use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **ui:plugin**
/// The plugin a portNotification applies to.
:"http://lv2plug.in/ns/extensions/ui#plugin", Plugin, plugin,
ontology::classes::lv2::Plugin<'g, G>,
167);

property!(
/// **ui:portIndex**
/// The index of the port a portNotification applies to.
:"http://lv2plug.in/ns/extensions/ui#portIndex", PortIndex, port_index,
ontology::classes::xsd::Decimal<'g, G>,
168);

property!(
/// **ui:portNotification**
:"http://lv2plug.in/ns/extensions/ui#portNotification", PortNotification, port_notification,
ontology::classes::ui::PortNotification<'g, G>,
169);

property!(
/// **ui:protocol**
/// The protocol to be used for this notification.
:"http://lv2plug.in/ns/extensions/ui#protocol", Protocol, protocol,
ontology::classes::ui::PortProtocol<'g, G>,
170);

property!(
/// **ui:ui**
/// Relates a plugin to a UI that applies to it.
:"http://lv2plug.in/ns/extensions/ui#ui", Ui, ui,
ontology::classes::ui::UI<'g, G>,
171);

property!(
/// **ui:updateRate**
/// The target rate, in Hz, to send updates to the UI.
:"http://lv2plug.in/ns/extensions/ui#updateRate", UpdateRate, update_rate,
ontology::classes::xsd::Float<'g, G>,
172);

property!(
/// **ui:windowTitle**
/// The title for the window shown by LV2UI_Show_Interface.
:"http://lv2plug.in/ns/extensions/ui#windowTitle", WindowTitle, window_title,
ontology::classes::xsd::String<'g, G>,
173);
