use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **dcterms:Agent**
/// A resource that acts or has the power to act.
:"http://purl.org/dc/terms/Agent", Agent,
1);

class!(
/// **dcterms:AgentClass**
/// A group of agents.
:"http://purl.org/dc/terms/AgentClass", AgentClass,
2);

class!(
/// **dcterms:LicenseDocument**
/// A legal document giving official permission to do something with a Resource.
:"http://purl.org/dc/terms/LicenseDocument", LicenseDocument,
3);

class!(
/// **dcterms:LinguisticSystem**
/// A system of signs, symbols, sounds, gestures, or rules used in communication.
:"http://purl.org/dc/terms/LinguisticSystem", LinguisticSystem,
4);

class!(
/// **dcterms:MediaType**
/// A file format or physical medium.
:"http://purl.org/dc/terms/MediaType", MediaType,
5);

class!(
/// **dcterms:MediaTypeOrExtent**
/// A media type or extent.
:"http://purl.org/dc/terms/MediaTypeOrExtent", MediaTypeOrExtent,
6);

class!(
/// **dcterms:RightsStatement**
/// A statement about the intellectual property rights (IPR) held in or over a Resource, a legal document giving official permission to do something with a resource, or a statement about access rights.
:"http://purl.org/dc/terms/RightsStatement", RightsStatement,
7);

class!(
/// **dcterms:Standard**
/// A basis for comparison; a reference point against which other things can be evaluated.
:"http://purl.org/dc/terms/Standard", Standard,
8);
