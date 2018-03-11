use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **dct:alternative**
/// An alternative name for the resource.
:"http://purl.org/dc/terms/alternative", Alternative, alternative,
ontology::classes::rdfs::Literal<'g, G>,
199);

property!(
/// **dct:available**
/// Date (often a range) that the resource became or will become available.
:"http://purl.org/dc/terms/available", Available, available,
ontology::classes::rdfs::Literal<'g, G>,
200);

property!(
/// **dct:conformsTo**
/// An established standard to which the described resource conforms.
:"http://purl.org/dc/terms/conformsTo", ConformsTo, conforms_to,
ontology::classes::dct::Standard<'g, G>,
201);

property!(
/// **dct:contributor**
/// An entity responsible for making contributions to the resource.
:"http://purl.org/dc/terms/contributor", Contributor, contributor,
ontology::classes::dct::Agent<'g, G>,
202);

property!(
/// **dct:created**
/// Date of creation of the resource.
:"http://purl.org/dc/terms/created", Created, created,
ontology::classes::rdfs::Literal<'g, G>,
203);

property!(
/// **dct:creator**
/// An entity primarily responsible for making the resource.
:"http://purl.org/dc/terms/creator", Creator, creator,
ontology::classes::dct::Agent<'g, G>,
204);

property!(
/// **dct:date**
/// A point or period of time associated with an event in the lifecycle of the resource.
:"http://purl.org/dc/terms/date", Date, date,
ontology::classes::rdfs::Literal<'g, G>,
205);

property!(
/// **dct:dateAccepted**
/// Date of acceptance of the resource.
:"http://purl.org/dc/terms/dateAccepted", DateAccepted, date_accepted,
ontology::classes::rdfs::Literal<'g, G>,
206);

property!(
/// **dct:dateCopyrighted**
/// Date of copyright.
:"http://purl.org/dc/terms/dateCopyrighted", DateCopyrighted, date_copyrighted,
ontology::classes::rdfs::Literal<'g, G>,
207);

property!(
/// **dct:dateSubmitted**
/// Date of submission of the resource.
:"http://purl.org/dc/terms/dateSubmitted", DateSubmitted, date_submitted,
ontology::classes::rdfs::Literal<'g, G>,
208);

property!(
/// **dct:format**
/// The file format, physical medium, or dimensions of the resource.
:"http://purl.org/dc/terms/format", Format, format,
ontology::classes::dct::MediaTypeOrExtent<'g, G>,
210);

property!(
/// **dct:issued**
/// Date of formal issuance (e.g., publication) of the resource.
:"http://purl.org/dc/terms/issued", Issued, issued,
ontology::classes::rdfs::Literal<'g, G>,
220);

property!(
/// **dct:language**
/// A language of the resource.
:"http://purl.org/dc/terms/language", Language, language,
ontology::classes::dct::LinguisticSystem<'g, G>,
221);

property!(
/// **dct:license**
/// A legal document giving official permission to do something with the resource.
:"http://purl.org/dc/terms/license", License, license,
ontology::classes::dct::LicenseDocument<'g, G>,
222);

property!(
/// **dct:modified**
/// Date on which the resource was changed.
:"http://purl.org/dc/terms/modified", Modified, modified,
ontology::classes::rdfs::Literal<'g, G>,
223);

property!(
/// **dct:publisher**
/// An entity responsible for making the resource available.
:"http://purl.org/dc/terms/publisher", Publisher, publisher,
ontology::classes::dct::Agent<'g, G>,
224);

property!(
/// **dct:rights**
/// Information about rights held in and over the resource.
:"http://purl.org/dc/terms/rights", Rights, rights,
ontology::classes::dct::RightsStatement<'g, G>,
229);

property!(
/// **dct:rightsHolder**
/// A person or organization owning or managing rights over the resource.
:"http://purl.org/dc/terms/rightsHolder", RightsHolder, rights_holder,
ontology::classes::dct::Agent<'g, G>,
230);

property!(
/// **dct:title**
/// A name given to the resource.
:"http://purl.org/dc/terms/title", Title, title,
ontology::classes::rdfs::Literal<'g, G>,
231);
