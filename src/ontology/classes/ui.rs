use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **ui:CocoaUI**
/// A UI where the LV2_Widget is a pointer to a NSView, the basic view type for the Cocoa API (formerly OpenStep).  This is the native UI type on Mac OS X.
:"http://lv2plug.in/ns/extensions/ui#CocoaUI", CocoaUI,
2);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for CocoaUI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, CocoaUI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:Gtk3UI**
/// A UI where the LV2_Widget is a pointer to a Gtk+ 3.0 compatible GtkWidget, and the host guarantees that the Gtk+ library has been initialised and the Glib main loop is running before a UI of this type is instantiated.
:"http://lv2plug.in/ns/extensions/ui#Gtk3UI", Gtk3UI,
3);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for Gtk3UI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, Gtk3UI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:GtkUI**
/// A UI where the LV2_Widget is a pointer to a Gtk+ 2.0 compatible GtkWidget, and the host guarantees that the Gtk+ library has been initialised and the Glib main loop is running before a UI of this type is instantiated.
:"http://lv2plug.in/ns/extensions/ui#GtkUI", GtkUI,
4);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for GtkUI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, GtkUI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:PortNotification**
:"http://lv2plug.in/ns/extensions/ui#PortNotification", PortNotification,
5);
impl<'g, G: 'g> ontology::properties::ui::Plugin<'g> for PortNotification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Plugin<'g> for resource::IRI<'g, PortNotification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortIndex<'g> for PortNotification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortIndex<'g> for resource::IRI<'g, PortNotification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Protocol<'g> for PortNotification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::Protocol<'g> for resource::IRI<'g, PortNotification<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:PortProtocol**
:"http://lv2plug.in/ns/extensions/ui#PortProtocol", PortProtocol,
6);

class!(
/// **ui:Qt4UI**
/// A UI where the LV2_Widget is a pointer to a Qt4 compatible QWidget, and the host guarantees that the Qt4 library has been initialised and the Qt4 main loop is running before a UI of this type is instantiated.
:"http://lv2plug.in/ns/extensions/ui#Qt4UI", Qt4UI,
7);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for Qt4UI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, Qt4UI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:Qt5UI**
/// A UI where the LV2_Widget is a pointer to a Qt5 compatible QWidget, and the host guarantees that the Qt5 library has been initialised and the Qt5 main loop is running before a UI of this type is instantiated.
:"http://lv2plug.in/ns/extensions/ui#Qt5UI", Qt5UI,
8);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for Qt5UI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, Qt5UI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:UI**
/// A UI for an LV2 plugin
:"http://lv2plug.in/ns/extensions/ui#UI", UI,
9);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for UI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, UI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:WindowsUI**
/// A UI where the LV2_Widget is a Windows HWND window ID.  Note this is actually an unsigned 32-bit integer, i.e. the LV2_Widget is not a pointer to a HWND but should be interpreted as an HWND itself.  This is the native UI type on Microsoft Windows.
:"http://lv2plug.in/ns/extensions/ui#WindowsUI", WindowsUI,
10);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for WindowsUI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, WindowsUI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:X11UI**
/// A UI where the LV2_Widget is an X11 window ID.  Note this is actually an integer, i.e. the LV2_Widget is not a pointer to an X11 window ID, but should be itself taken as an integer value.  This is the native UI type on most POSIX systems.
:"http://lv2plug.in/ns/extensions/ui#X11UI", X11UI,
11);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for X11UI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, X11UI<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **ui:external**
:"http://lv2plug.in/ns/extensions/ui#external", External,
12);
