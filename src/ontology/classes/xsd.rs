use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **xsd:boolean**
:"http://www.w3.org/2001/XMLSchema#boolean", Boolean,
120);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Boolean<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:decimal**
/// A subset of the real numbers, which can be represented by decimal numerals.
:"http://www.w3.org/2001/XMLSchema#decimal", Decimal,
121);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Decimal<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:float**
/// IEEE single-precision 32-bit floating point.
:"http://www.w3.org/2001/XMLSchema#float", Float,
122);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Float<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:int**
:"http://www.w3.org/2001/XMLSchema#int", Int,
123);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Int<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:nonNegativeInteger**
:"http://www.w3.org/2001/XMLSchema#nonNegativeInteger", NonNegativeInteger,
124);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NonNegativeInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:positiveInteger**
:"http://www.w3.org/2001/XMLSchema#positiveInteger", PositiveInteger,
125);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PositiveInteger<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:string**
/// A character string.
:"http://www.w3.org/2001/XMLSchema#string", String,
126);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, String<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **xsd:unsignedInt**
:"http://www.w3.org/2001/XMLSchema#unsignedInt", UnsignedInt,
127);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where G: graph::Graph<'g> {}
