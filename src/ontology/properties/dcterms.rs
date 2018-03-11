use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **dcterms:alternative**
/// An alternative name for the resource.
:"http://purl.org/dc/terms/alternative", Alternative, alternative,
ontology::classes::rdfs::Literal<'g, G>,
11);

property!(
/// **dcterms:available**
/// Date (often a range) that the resource became or will become available.
:"http://purl.org/dc/terms/available", Available, available,
ontology::classes::rdfs::Literal<'g, G>,
12);

property!(
/// **dcterms:conformsTo**
/// An established standard to which the described resource conforms.
:"http://purl.org/dc/terms/conformsTo", ConformsTo, conforms_to,
ontology::classes::dcterms::Standard<'g, G>,
13);

property!(
/// **dcterms:contributor**
/// An entity responsible for making contributions to the resource.
:"http://purl.org/dc/terms/contributor", Contributor, contributor,
ontology::classes::dcterms::Agent<'g, G>,
14);

property!(
/// **dcterms:created**
/// Date of creation of the resource.
:"http://purl.org/dc/terms/created", Created, created,
ontology::classes::rdfs::Literal<'g, G>,
15);

property!(
/// **dcterms:creator**
/// An entity primarily responsible for making the resource.
:"http://purl.org/dc/terms/creator", Creator, creator,
ontology::classes::dcterms::Agent<'g, G>,
16);

property!(
/// **dcterms:date**
/// A point or period of time associated with an event in the lifecycle of the resource.
:"http://purl.org/dc/terms/date", Date, date,
ontology::classes::rdfs::Literal<'g, G>,
17);

property!(
/// **dcterms:dateAccepted**
/// Date of acceptance of the resource.
:"http://purl.org/dc/terms/dateAccepted", DateAccepted, date_accepted,
ontology::classes::rdfs::Literal<'g, G>,
18);

property!(
/// **dcterms:dateCopyrighted**
/// Date of copyright.
:"http://purl.org/dc/terms/dateCopyrighted", DateCopyrighted, date_copyrighted,
ontology::classes::rdfs::Literal<'g, G>,
19);

property!(
/// **dcterms:dateSubmitted**
/// Date of submission of the resource.
:"http://purl.org/dc/terms/dateSubmitted", DateSubmitted, date_submitted,
ontology::classes::rdfs::Literal<'g, G>,
20);

property!(
/// **dcterms:format**
/// The file format, physical medium, or dimensions of the resource.
:"http://purl.org/dc/terms/format", Format, format,
ontology::classes::dcterms::MediaTypeOrExtent<'g, G>,
22);

property!(
/// **dcterms:issued**
/// Date of formal issuance (e.g., publication) of the resource.
:"http://purl.org/dc/terms/issued", Issued, issued,
ontology::classes::rdfs::Literal<'g, G>,
32);

property!(
/// **dcterms:language**
/// A language of the resource.
:"http://purl.org/dc/terms/language", Language, language,
ontology::classes::dcterms::LinguisticSystem<'g, G>,
33);

property!(
/// **dcterms:license**
/// A legal document giving official permission to do something with the resource.
:"http://purl.org/dc/terms/license", License, license,
ontology::classes::dcterms::LicenseDocument<'g, G>,
34);

property!(
/// **dcterms:modified**
/// Date on which the resource was changed.
:"http://purl.org/dc/terms/modified", Modified, modified,
ontology::classes::rdfs::Literal<'g, G>,
35);

property!(
/// **dcterms:publisher**
/// An entity responsible for making the resource available.
:"http://purl.org/dc/terms/publisher", Publisher, publisher,
ontology::classes::dcterms::Agent<'g, G>,
36);

property!(
/// **dcterms:rights**
/// Information about rights held in and over the resource.
:"http://purl.org/dc/terms/rights", Rights, rights,
ontology::classes::dcterms::RightsStatement<'g, G>,
41);

property!(
/// **dcterms:rightsHolder**
/// A person or organization owning or managing rights over the resource.
:"http://purl.org/dc/terms/rightsHolder", RightsHolder, rights_holder,
ontology::classes::dcterms::Agent<'g, G>,
42);

property!(
/// **dcterms:title**
/// A name given to the resource.
:"http://purl.org/dc/terms/title", Title, title,
ontology::classes::rdfs::Literal<'g, G>,
43);
