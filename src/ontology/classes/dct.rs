use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **dct:Agent**
/// A resource that acts or has the power to act.
:"http://purl.org/dc/terms/Agent", Agent,
85);

class!(
/// **dct:AgentClass**
/// A group of agents.
:"http://purl.org/dc/terms/AgentClass", AgentClass,
86);

class!(
/// **dct:LicenseDocument**
/// A legal document giving official permission to do something with a Resource.
:"http://purl.org/dc/terms/LicenseDocument", LicenseDocument,
87);

class!(
/// **dct:LinguisticSystem**
/// A system of signs, symbols, sounds, gestures, or rules used in communication.
:"http://purl.org/dc/terms/LinguisticSystem", LinguisticSystem,
88);

class!(
/// **dct:MediaType**
/// A file format or physical medium.
:"http://purl.org/dc/terms/MediaType", MediaType,
89);

class!(
/// **dct:MediaTypeOrExtent**
/// A media type or extent.
:"http://purl.org/dc/terms/MediaTypeOrExtent", MediaTypeOrExtent,
90);

class!(
/// **dct:RightsStatement**
/// A statement about the intellectual property rights (IPR) held in or over a Resource, a legal document giving official permission to do something with a resource, or a statement about access rights.
:"http://purl.org/dc/terms/RightsStatement", RightsStatement,
91);

class!(
/// **dct:Standard**
/// A basis for comparison; a reference point against which other things can be evaluated.
:"http://purl.org/dc/terms/Standard", Standard,
92);
