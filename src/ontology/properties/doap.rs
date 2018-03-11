use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **doap:anon-root**
/// Dépôt pour accès anonyme.
/// Repositorio para acceso anónimo.
/// Repository for anonymous access.
/// Repository für anonymen Zugriff
/// Úložiště pro anonymní přístup.
:"http://usefulinc.com/ns/doap#anon-root", Anon_root, anon_root,
ontology::classes::rdfs::Literal<'g, G>,
232);

property!(
/// **doap:audience**
/// Description of target user base
:"http://usefulinc.com/ns/doap#audience", Audience, audience,
ontology::classes::rdfs::Literal<'g, G>,
233);

property!(
/// **doap:blog**
/// URI of a blog related to a project
:"http://usefulinc.com/ns/doap#blog", Blog, blog,
ontology::classes::rdfs::Resource<'g, G>,
234);

property!(
/// **doap:created**
/// Date when something was created, in YYYY-MM-DD form. e.g. 2004-04-05
/// Date à laquelle a été créé quelque chose, au format AAAA-MM-JJ (par ex. 2004-04-05)
/// Datum, kdy bylo něco vytvořeno ve formátu RRRR-MM-DD, např. 2004-04-05
/// Erstellungsdatum von Irgendwas, angegeben im YYYY-MM-DD Format, z.B. 2004-04-05.
/// Fecha en la que algo fue creado, en formato AAAA-MM-DD. e.g. 2004-04-05
:"http://usefulinc.com/ns/doap#created", Created, created,
ontology::classes::rdfs::Literal<'g, G>,
235);

property!(
/// **doap:description**
/// Beschreibung eines Projekts als einfacher Text mit der Länge von 2 bis 4 Sätzen.
/// Descripción en texto plano de un proyecto, de 2 a 4 enunciados de longitud.
/// Plain text description of a project, of 2-4 sentences in length.
/// Texte descriptif d'un projet, long de 2 à 4 phrases.
/// Čistě textový, 2 až 4 věty dlouhý popis projektu.
:"http://usefulinc.com/ns/doap#description", Description, description,
ontology::classes::rdfs::Literal<'g, G>,
236);

property!(
/// **doap:developer**
/// Desarrollador de software para el proyecto.
/// Developer of software for the project.
/// Développeur pour le projet.
/// Software-Entwickler für eine Projekt.
/// Vývojář softwaru projektu.
:"http://usefulinc.com/ns/doap#developer", Developer, developer,
ontology::classes::foaf::Person<'g, G>,
237);

property!(
/// **doap:documenter**
/// Collaborateur à la documentation du projet.
/// Contributor of documentation to the project.
/// Mitarbeiter an der Dokumentation eines Projektes.
/// Proveedor de documentación para el proyecto.
/// Spoluautor dokumentace projektu.
:"http://usefulinc.com/ns/doap#documenter", Documenter, documenter,
ontology::classes::foaf::Person<'g, G>,
238);

property!(
/// **doap:helper**
/// Colaborador del proyecto.
/// Collaborateur au projet.
/// Project contributor.
/// Projekt-Mitarbeiter.
/// Spoluautor projektu.
:"http://usefulinc.com/ns/doap#helper", Helper, helper,
ontology::classes::foaf::Person<'g, G>,
239);

property!(
/// **doap:implements**
/// A specification that a project implements. Could be a standard, API or legally defined level of conformance.
:"http://usefulinc.com/ns/doap#implements", Implements, implements,
ontology::classes::doap::Specification<'g, G>,
240);

property!(
/// **doap:language**
/// ISO language code a project has been translated into
:"http://usefulinc.com/ns/doap#language", Language, language,
ontology::classes::rdfs::Literal<'g, G>,
241);

property!(
/// **doap:maintainer**
/// Desarrollador principal de un proyecto, un líder de proyecto.
/// Développeur principal d'un projet, un meneur du projet.
/// Hauptentwickler eines Projektes, der Projektleiter
/// Maintainer of a project, a project leader.
/// Správce projektu, vedoucí projektu.
:"http://usefulinc.com/ns/doap#maintainer", Maintainer, maintainer,
ontology::classes::foaf::Person<'g, G>,
242);

property!(
/// **doap:name**
/// A name of something.
/// Der Name von Irgendwas
/// El nombre de algo.
/// Jméno něčeho.
/// Le nom de quelque chose.
:"http://usefulinc.com/ns/doap#name", Name, name,
ontology::classes::rdfs::Literal<'g, G>,
243);

property!(
/// **doap:os**
/// Betriebssystem auf dem das Projekt eingesetzt werden kann. Diese Eigenschaft kann ausgelassen werden, wenn das Projekt nicht BS-spezifisch ist.
/// Operating system that a project is limited to.  Omit this property if the project is not OS-specific.
/// Operační systém, na jehož použití je projekt limitován. Vynechejte tuto vlastnost, pokud je projekt nezávislý na operačním systému.
/// Sistema opertivo al cuál está limitado el proyecto.  Omita esta propiedad si el proyecto no es específico		de un sistema opertaivo en particular.
/// Système d'exploitation auquel est limité le projet. Omettez cette propriété si le		projet n'est pas limité à un système d'exploitation.
:"http://usefulinc.com/ns/doap#os", Os, os,
ontology::classes::rdfs::Literal<'g, G>,
244);

property!(
/// **doap:platform**
/// Indicator of software platform (non-OS specific), e.g. Java, Firefox, ECMA CLR
:"http://usefulinc.com/ns/doap#platform", Platform, platform,
ontology::classes::rdfs::Literal<'g, G>,
245);

property!(
/// **doap:programming-language**
/// Langage de programmation avec lequel un projet est implémenté,		ou avec lequel il est prévu de l'utiliser.
/// Lenguaje de programación en el que un proyecto es implementado o con el cuál pretende usarse.
/// Programmiersprache in der ein Projekt implementiert ist oder intendiert wird zu benutzen.
/// Programming language a project is implemented in or intended for use with.
/// Programovací jazyk, ve kterém je projekt implementován nebo pro který je zamýšlen k použití.
:"http://usefulinc.com/ns/doap#programming-language", Programming_language, programming_language,
ontology::classes::rdfs::Literal<'g, G>,
246);

property!(
/// **doap:release**
/// A project release.
/// Ein Release (Version) eines Projekts.
/// Relase (verze) projektu.
/// Un release (versión) de un proyecto.
/// Une release (révision) d'un projet.
:"http://usefulinc.com/ns/doap#release", Release, release,
ontology::classes::doap::Version<'g, G>,
247);

property!(
/// **doap:repository**
/// Dépôt du code source.
/// Quellcode-Versionierungssystem.
/// Repositorio del código fuente.
/// Source code repository.
/// Úložiště zdrojových kódů.
:"http://usefulinc.com/ns/doap#repository", Repository, repository,
ontology::classes::doap::Repository<'g, G>,
248);

property!(
/// **doap:revision**
/// Identifiant de révision d'une release du programme.
/// Identifikátor zpřístupněné revize softwaru.
/// Indentificador de la versión de un release de software.
/// Revision identifier of a software release.
/// Versionsidentifikator eines Software-Releases.
:"http://usefulinc.com/ns/doap#revision", Revision, revision,
ontology::classes::rdfs::Literal<'g, G>,
249);

property!(
/// **doap:service-endpoint**
/// The URI of a web service endpoint where software as a service may be accessed
:"http://usefulinc.com/ns/doap#service-endpoint", Service_endpoint, service_endpoint,
ontology::classes::rdfs::Resource<'g, G>,
250);

property!(
/// **doap:shortdesc**
/// Descripción corta (8 o 9 palabras) en texto plano de un proyecto.
/// Krátký (8 nebo 9 slov) čistě textový popis projektu.
/// Kurzbeschreibung (8 oder 9 Wörter) eines Projects als einfacher Text.
/// Short (8 or 9 words) plain text description of a project.
/// Texte descriptif concis (8 ou 9 mots) d'un projet.
:"http://usefulinc.com/ns/doap#shortdesc", Shortdesc, shortdesc,
ontology::classes::rdfs::Literal<'g, G>,
251);

property!(
/// **doap:tester**
/// A tester or other quality control contributor.
/// Ein Tester oder anderer Mitarbeiter der Qualitätskontrolle.
/// Tester nebo jiný spoluautor kontrolující kvalitu.
/// Un tester u otro proveedor de control de calidad.
/// Un testeur ou un collaborateur au contrôle qualité.
:"http://usefulinc.com/ns/doap#tester", Tester, tester,
ontology::classes::foaf::Person<'g, G>,
252);

property!(
/// **doap:translator**
/// Collaborateur à la traduction du projet.
/// Contributor of translations to the project.
/// Mitarbeiter an den Übersetzungen eines Projektes.
/// Proveedor de traducciones al proyecto.
/// Spoluautor překladu projektu.
:"http://usefulinc.com/ns/doap#translator", Translator, translator,
ontology::classes::foaf::Person<'g, G>,
253);

property!(
/// **doap:vendor**
/// Vendor organization: commercial, free or otherwise
:"http://usefulinc.com/ns/doap#vendor", Vendor, vendor,
ontology::classes::foaf::Organization<'g, G>,
254);
