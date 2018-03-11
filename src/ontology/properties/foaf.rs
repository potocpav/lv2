use std;
use rome::graph;
use rome::resource;
use ontology;

property!(
/// **foaf:account**
/// Indicates an account held by this agent.
:"http://xmlns.com/foaf/0.1/account", Account, account,
ontology::classes::foaf::OnlineAccount<'g, G>,
312);

property!(
/// **foaf:accountName**
/// Indicates the name (identifier) associated with this online account.
:"http://xmlns.com/foaf/0.1/accountName", AccountName, account_name,
ontology::classes::rdfs::Literal<'g, G>,
313);

property!(
/// **foaf:accountServiceHomepage**
/// Indicates a homepage of the service provide for this online account.
:"http://xmlns.com/foaf/0.1/accountServiceHomepage", AccountServiceHomepage, account_service_homepage,
ontology::classes::foaf::Document<'g, G>,
314);

property!(
/// **foaf:age**
/// The age in years of some agent.
:"http://xmlns.com/foaf/0.1/age", Age, age,
ontology::classes::rdfs::Literal<'g, G>,
315);

property!(
/// **foaf:aimChatID**
/// An AIM chat ID
:"http://xmlns.com/foaf/0.1/aimChatID", AimChatID, aim_chat_i_d,
ontology::classes::rdfs::Literal<'g, G>,
316);

property!(
/// **foaf:birthday**
/// The birthday of this Agent, represented in mm-dd string form, eg. '12-31'.
:"http://xmlns.com/foaf/0.1/birthday", Birthday, birthday,
ontology::classes::rdfs::Literal<'g, G>,
317);

property!(
/// **foaf:currentProject**
/// A current project this person works on.
:"http://xmlns.com/foaf/0.1/currentProject", CurrentProject, current_project,
ontology::classes::owl::Thing<'g, G>,
318);

property!(
/// **foaf:depiction**
/// A depiction of some thing.
:"http://xmlns.com/foaf/0.1/depiction", Depiction, depiction,
ontology::classes::foaf::Image<'g, G>,
319);

property!(
/// **foaf:depicts**
/// A thing depicted in this representation.
:"http://xmlns.com/foaf/0.1/depicts", Depicts, depicts,
ontology::classes::owl::Thing<'g, G>,
320);

property!(
/// **foaf:dnaChecksum**
/// A checksum for the DNA of some thing. Joke.
:"http://xmlns.com/foaf/0.1/dnaChecksum", DnaChecksum, dna_checksum,
ontology::classes::rdfs::Literal<'g, G>,
321);

property!(
/// **foaf:familyName**
/// The family name of some person.
:"http://xmlns.com/foaf/0.1/familyName", FamilyName, family_name,
ontology::classes::rdfs::Literal<'g, G>,
322);

property!(
/// **foaf:firstName**
/// The first name of a person.
:"http://xmlns.com/foaf/0.1/firstName", FirstName, first_name,
ontology::classes::rdfs::Literal<'g, G>,
323);

property!(
/// **foaf:fundedBy**
/// An organization funding a project or person.
:"http://xmlns.com/foaf/0.1/fundedBy", FundedBy, funded_by,
ontology::classes::owl::Thing<'g, G>,
324);

property!(
/// **foaf:geekcode**
/// A textual geekcode for this person, see http://www.geekcode.com/geek.html
:"http://xmlns.com/foaf/0.1/geekcode", Geekcode, geekcode,
ontology::classes::rdfs::Literal<'g, G>,
325);

property!(
/// **foaf:gender**
/// The gender of this Agent (typically but not necessarily 'male' or 'female').
:"http://xmlns.com/foaf/0.1/gender", Gender, gender,
ontology::classes::rdfs::Literal<'g, G>,
326);

property!(
/// **foaf:holdsAccount**
/// Indicates an account held by this agent.
:"http://xmlns.com/foaf/0.1/holdsAccount", HoldsAccount, holds_account,
ontology::classes::foaf::OnlineAccount<'g, G>,
329);

property!(
/// **foaf:homepage**
/// A homepage for some thing.
:"http://xmlns.com/foaf/0.1/homepage", Homepage, homepage,
ontology::classes::foaf::Document<'g, G>,
330);

property!(
/// **foaf:icqChatID**
/// An ICQ chat ID
:"http://xmlns.com/foaf/0.1/icqChatID", IcqChatID, icq_chat_i_d,
ontology::classes::rdfs::Literal<'g, G>,
331);

property!(
/// **foaf:img**
/// An image that can be used to represent some thing (ie. those depictions which are particularly representative of something, eg. one's photo on a homepage).
:"http://xmlns.com/foaf/0.1/img", Img, img,
ontology::classes::foaf::Image<'g, G>,
332);

property!(
/// **foaf:interest**
/// A page about a topic of interest to this person.
:"http://xmlns.com/foaf/0.1/interest", Interest, interest,
ontology::classes::foaf::Document<'g, G>,
333);

property!(
/// **foaf:isPrimaryTopicOf**
/// A document that this thing is the primary topic of.
:"http://xmlns.com/foaf/0.1/isPrimaryTopicOf", IsPrimaryTopicOf, is_primary_topic_of,
ontology::classes::foaf::Document<'g, G>,
334);

property!(
/// **foaf:jabberID**
/// A jabber ID for something.
:"http://xmlns.com/foaf/0.1/jabberID", JabberID, jabber_i_d,
ontology::classes::rdfs::Literal<'g, G>,
335);

property!(
/// **foaf:knows**
/// A person known by this person (indicating some level of reciprocated interaction between the parties).
:"http://xmlns.com/foaf/0.1/knows", Knows, knows,
ontology::classes::foaf::Person<'g, G>,
336);

property!(
/// **foaf:lastName**
/// The last name of a person.
:"http://xmlns.com/foaf/0.1/lastName", LastName, last_name,
ontology::classes::rdfs::Literal<'g, G>,
337);

property!(
/// **foaf:logo**
/// A logo representing some thing.
:"http://xmlns.com/foaf/0.1/logo", Logo, logo,
ontology::classes::owl::Thing<'g, G>,
338);

property!(
/// **foaf:made**
/// Something that was made by this agent.
:"http://xmlns.com/foaf/0.1/made", Made, made,
ontology::classes::owl::Thing<'g, G>,
339);

property!(
/// **foaf:maker**
/// An agent that made this thing.
:"http://xmlns.com/foaf/0.1/maker", Maker, maker,
ontology::classes::foaf::Agent<'g, G>,
340);

property!(
/// **foaf:mbox**
/// A personal mailbox, ie. an Internet mailbox associated with exactly one owner, the first owner of this mailbox. This is a 'static inverse functional property', in that there is (across time and change) at most one individual that ever has any particular value for foaf:mbox.
:"http://xmlns.com/foaf/0.1/mbox", Mbox, mbox,
ontology::classes::owl::Thing<'g, G>,
341);

property!(
/// **foaf:mbox_sha1sum**
/// The sha1sum of the URI of an Internet mailbox associated with exactly one owner, the first owner of the mailbox.
:"http://xmlns.com/foaf/0.1/mbox_sha1sum", Mbox_sha1sum, mbox_sha1sum,
ontology::classes::rdfs::Literal<'g, G>,
342);

property!(
/// **foaf:member**
/// Indicates a member of a Group
:"http://xmlns.com/foaf/0.1/member", Member, member,
ontology::classes::foaf::Agent<'g, G>,
343);

property!(
/// **foaf:msnChatID**
/// An MSN chat ID
:"http://xmlns.com/foaf/0.1/msnChatID", MsnChatID, msn_chat_i_d,
ontology::classes::rdfs::Literal<'g, G>,
345);

property!(
/// **foaf:myersBriggs**
/// A Myers Briggs (MBTI) personality classification.
:"http://xmlns.com/foaf/0.1/myersBriggs", MyersBriggs, myers_briggs,
ontology::classes::rdfs::Literal<'g, G>,
346);

property!(
/// **foaf:name**
/// A name for some thing.
:"http://xmlns.com/foaf/0.1/name", Name, name,
ontology::classes::rdfs::Literal<'g, G>,
347);

property!(
/// **foaf:openid**
/// An OpenID for an Agent.
:"http://xmlns.com/foaf/0.1/openid", Openid, openid,
ontology::classes::foaf::Document<'g, G>,
349);

property!(
/// **foaf:page**
/// A page or document about this thing.
:"http://xmlns.com/foaf/0.1/page", Page, page,
ontology::classes::foaf::Document<'g, G>,
350);

property!(
/// **foaf:pastProject**
/// A project this person has previously worked on.
:"http://xmlns.com/foaf/0.1/pastProject", PastProject, past_project,
ontology::classes::owl::Thing<'g, G>,
351);

property!(
/// **foaf:plan**
/// A .plan comment, in the tradition of finger and '.plan' files.
:"http://xmlns.com/foaf/0.1/plan", Plan, plan,
ontology::classes::rdfs::Literal<'g, G>,
353);

property!(
/// **foaf:primaryTopic**
/// The primary topic of some page or document.
:"http://xmlns.com/foaf/0.1/primaryTopic", PrimaryTopic, primary_topic,
ontology::classes::owl::Thing<'g, G>,
354);

property!(
/// **foaf:publications**
/// A link to the publications of this person.
:"http://xmlns.com/foaf/0.1/publications", Publications, publications,
ontology::classes::foaf::Document<'g, G>,
355);

property!(
/// **foaf:schoolHomepage**
/// A homepage of a school attended by the person.
:"http://xmlns.com/foaf/0.1/schoolHomepage", SchoolHomepage, school_homepage,
ontology::classes::foaf::Document<'g, G>,
356);

property!(
/// **foaf:skypeID**
/// A Skype ID
:"http://xmlns.com/foaf/0.1/skypeID", SkypeID, skype_i_d,
ontology::classes::rdfs::Literal<'g, G>,
357);

property!(
/// **foaf:status**
/// A string expressing what the user is happy for the general public (normally) to know about their current activity.
:"http://xmlns.com/foaf/0.1/status", Status, status,
ontology::classes::rdfs::Literal<'g, G>,
358);

property!(
/// **foaf:surname**
/// The surname of some person.
:"http://xmlns.com/foaf/0.1/surname", Surname, surname,
ontology::classes::rdfs::Literal<'g, G>,
359);

property!(
/// **foaf:theme**
/// A theme.
:"http://xmlns.com/foaf/0.1/theme", Theme, theme,
ontology::classes::owl::Thing<'g, G>,
360);

property!(
/// **foaf:thumbnail**
/// A derived thumbnail image.
:"http://xmlns.com/foaf/0.1/thumbnail", Thumbnail, thumbnail,
ontology::classes::foaf::Image<'g, G>,
361);

property!(
/// **foaf:tipjar**
/// A tipjar document for this agent, describing means for payment and reward.
:"http://xmlns.com/foaf/0.1/tipjar", Tipjar, tipjar,
ontology::classes::foaf::Document<'g, G>,
362);

property!(
/// **foaf:topic**
/// A topic of some page or document.
:"http://xmlns.com/foaf/0.1/topic", Topic, topic,
ontology::classes::owl::Thing<'g, G>,
364);

property!(
/// **foaf:topic_interest**
/// A thing of interest to this person.
:"http://xmlns.com/foaf/0.1/topic_interest", Topic_interest, topic_interest,
ontology::classes::owl::Thing<'g, G>,
365);

property!(
/// **foaf:weblog**
/// A weblog of some thing (whether person, group, company etc.).
:"http://xmlns.com/foaf/0.1/weblog", Weblog, weblog,
ontology::classes::foaf::Document<'g, G>,
366);

property!(
/// **foaf:workInfoHomepage**
/// A work info homepage of some person; a page about their work for some organization.
:"http://xmlns.com/foaf/0.1/workInfoHomepage", WorkInfoHomepage, work_info_homepage,
ontology::classes::foaf::Document<'g, G>,
367);

property!(
/// **foaf:workplaceHomepage**
/// A workplace homepage of some person; the homepage of an organization they work for.
:"http://xmlns.com/foaf/0.1/workplaceHomepage", WorkplaceHomepage, workplace_homepage,
ontology::classes::foaf::Document<'g, G>,
368);

property!(
/// **foaf:yahooChatID**
/// A Yahoo chat ID
:"http://xmlns.com/foaf/0.1/yahooChatID", YahooChatID, yahoo_chat_i_d,
ontology::classes::rdfs::Literal<'g, G>,
369);
