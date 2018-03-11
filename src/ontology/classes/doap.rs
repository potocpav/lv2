use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **doap:ArchRepository**
/// Dépôt GNU Arch du code source.
/// GNU Arch Quellcode-Versionierungssystem.
/// GNU Arch source code repository.
/// Repositorio GNU Arch del código fuente.
/// Úložiště zdrojových kódů GNU Arch.
:"http://usefulinc.com/ns/doap#ArchRepository", ArchRepository,
93);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for ArchRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, ArchRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:BKRepository**
/// BitKeeper Quellcode-Versionierungssystem.
/// BitKeeper source code repository.
/// Dépôt BitKeeper du code source.
/// Repositorio BitKeeper del código fuente.
/// Úložiště zdrojových kódů BitKeeper.
:"http://usefulinc.com/ns/doap#BKRepository", BKRepository,
94);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for BKRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, BKRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:BazaarBranch**
/// Bazaar source code branch.
:"http://usefulinc.com/ns/doap#BazaarBranch", BazaarBranch,
95);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for BazaarBranch<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, BazaarBranch<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:CVSRepository**
/// CVS Quellcode-Versionierungssystem.
/// CVS source code repository.
/// Dépôt CVS du code source.
/// Repositorio CVS del código fuente.
/// Úložiště zdrojových kódů CVS.
:"http://usefulinc.com/ns/doap#CVSRepository", CVSRepository,
96);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for CVSRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, CVSRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:DarcsRepository**
/// Dépôt darcs du code source.
/// Repositorio darcs del código fuente.
/// darcs source code repository.
:"http://usefulinc.com/ns/doap#DarcsRepository", DarcsRepository,
97);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for DarcsRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, DarcsRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:GitBranch**
/// Git source code branch.
:"http://usefulinc.com/ns/doap#GitBranch", GitBranch,
98);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for GitBranch<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, GitBranch<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:HgRepository**
/// Mercurial source code repository.
:"http://usefulinc.com/ns/doap#HgRepository", HgRepository,
99);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for HgRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, HgRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:Project**
/// A project.
/// Ein Projekt.
/// Projekt.
/// Un projet.
/// Un proyecto.
:"http://usefulinc.com/ns/doap#Project", Project,
100);
impl<'g, G: 'g> ontology::properties::doap::Audience<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Audience<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Blog<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Blog<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Developer<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Developer<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Documenter<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Documenter<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Helper<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Helper<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Implements<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Implements<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Language<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Language<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Maintainer<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Maintainer<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Programming_language<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Programming_language<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Release<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Release<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Repository<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Repository<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Service_endpoint<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Service_endpoint<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Tester<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Tester<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Translator<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Translator<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Vendor<'g> for Project<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Vendor<'g> for resource::IRI<'g, Project<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:Repository**
/// Dépôt du code source.
/// Quellcode-Versionierungssystem.
/// Repositorio del código fuente.
/// Source code repository.
/// Úložiště zdrojových kódů.
:"http://usefulinc.com/ns/doap#Repository", Repository,
101);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for Repository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, Repository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:SVNRepository**
/// Dépôt Subversion du code source.
/// Repositorio Subversion del código fuente.
/// Subversion Quellcode-Versionierungssystem.
/// Subversion source code repository.
/// Úložiště zdrojových kódů Subversion.
:"http://usefulinc.com/ns/doap#SVNRepository", SVNRepository,
102);
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for SVNRepository<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Anon_root<'g> for resource::IRI<'g, SVNRepository<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:Specification**
/// A specification of a system's aspects, technical or otherwise.
:"http://usefulinc.com/ns/doap#Specification", Specification,
103);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Specification<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Specification<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **doap:Version**
/// Détails sur une version d'une realease d'un projet.
/// Informace o uvolněné verzi projektu.
/// Información sobre la versión de un release del proyecto.
/// Version information of a project release.
/// Versionsinformation eines Projekt Releases.
:"http://usefulinc.com/ns/doap#Version", Version,
104);
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for Version<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Os<'g> for resource::IRI<'g, Version<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for Version<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Platform<'g> for resource::IRI<'g, Version<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Revision<'g> for Version<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::doap::Revision<'g> for resource::IRI<'g, Version<'g, G>> where G: graph::Graph<'g> {}
