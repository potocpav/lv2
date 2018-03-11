use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **xsd:fractionDigits**
/// The total number of digits to the right of the decimal point required to represent a value.
:"http://www.w3.org/2001/XMLSchema#fractionDigits", FractionDigits, fraction_digits,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
271);

property!(
/// **xsd:totalDigits**
/// The maximum number of decimal digits required to represent a value.
:"http://www.w3.org/2001/XMLSchema#totalDigits", TotalDigits, total_digits,
ontology::classes::xsd::PositiveInteger<'g, G>,
277);
