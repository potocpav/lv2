use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **extui:Widget**
:"http://kxstudio.sf.net/ns/lv2ext/external-ui#Widget", Widget,
1);
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for Widget<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::ui::PortNotification<'g> for resource::IRI<'g, Widget<'g, G>> where G: graph::Graph<'g> {}
