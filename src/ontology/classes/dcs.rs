use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **dcs:Addition**
/// A change that added a new feature.
:"http://ontologi.es/doap-changeset#Addition", Addition,
70);

class!(
/// **dcs:BackCompat**
/// A change that breaks backwards compatibility, changing documented or tested behaviour.
:"http://ontologi.es/doap-changeset#BackCompat", BackCompat,
71);

class!(
/// **dcs:Bugfix**
/// A change that fixed a problem with an existing feature.
:"http://ontologi.es/doap-changeset#Bugfix", Bugfix,
72);

class!(
/// **dcs:Change**
/// A change to something. Use rdfs:label to briefly describe the change. Use rdfs:comment for additional information.
:"http://ontologi.es/doap-changeset#Change", Change,
73);

class!(
/// **dcs:ChangeSet**
/// A collection of changes. Not necessarily disjoint with foaf:Document!
:"http://ontologi.es/doap-changeset#ChangeSet", ChangeSet,
74);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ChangeSet<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ChangeSet<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **dcs:Documentation**
:"http://ontologi.es/doap-changeset#Documentation", Documentation,
75);

class!(
/// **dcs:Packaging**
:"http://ontologi.es/doap-changeset#Packaging", Packaging,
76);

class!(
/// **dcs:Regression**
/// A change that caused a problem with an existing feature.
:"http://ontologi.es/doap-changeset#Regression", Regression,
77);

class!(
/// **dcs:Removal**
/// A change that removed a feature.
:"http://ontologi.es/doap-changeset#Removal", Removal,
78);

class!(
/// **dcs:SecurityFix**
/// A change that fixed or improved a security problem.
:"http://ontologi.es/doap-changeset#SecurityFix", SecurityFix,
79);

class!(
/// **dcs:SecurityRegression**
/// A change that created or worsened a security problem.
:"http://ontologi.es/doap-changeset#SecurityRegression", SecurityRegression,
80);

class!(
/// **dcs:Tests**
/// A change to the test suite.
:"http://ontologi.es/doap-changeset#Tests", Tests,
81);

class!(
/// **dcs:ToDoList**
/// A collection of planned changes.
:"http://ontologi.es/doap-changeset#ToDoList", ToDoList,
82);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ToDoList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ToDoList<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **dcs:Update**
/// A change that brought a feature into compliance with policy or other changes in the external world.
:"http://ontologi.es/doap-changeset#Update", Update,
83);

class!(
/// **dcs:WishList**
/// A collection of desired changes.
:"http://ontologi.es/doap-changeset#WishList", WishList,
84);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for WishList<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, WishList<'g, G>> where G: graph::Graph<'g> {}
