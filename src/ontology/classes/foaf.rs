use std;
use rome::graph;
use rome::resource;
use rome::ontology_adapter;
use ontology;

class!(
/// **foaf:Agent**
/// An agent (eg. person, group, software or physical artifact).
:"http://xmlns.com/foaf/0.1/Agent", Agent,
153);
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for Agent<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for resource::IRI<'g, Agent<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Document**
/// A document.
:"http://xmlns.com/foaf/0.1/Document", Document,
154);
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for Document<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for resource::IRI<'g, Document<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for Document<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for resource::IRI<'g, Document<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Group**
/// A class of Agents.
:"http://xmlns.com/foaf/0.1/Group", Group,
155);
impl<'g, G: 'g> ontology::properties::foaf::Member<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Member<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for Group<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for resource::IRI<'g, Group<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Image**
/// An image.
:"http://xmlns.com/foaf/0.1/Image", Image,
156);
impl<'g, G: 'g> ontology::properties::foaf::Depicts<'g> for Image<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depicts<'g> for resource::IRI<'g, Image<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Thumbnail<'g> for Image<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Thumbnail<'g> for resource::IRI<'g, Image<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for Image<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for resource::IRI<'g, Image<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for Image<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for resource::IRI<'g, Image<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:LabelProperty**
/// A foaf:LabelProperty is any RDF property with texual values that serve as labels.
:"http://xmlns.com/foaf/0.1/LabelProperty", LabelProperty,
157);

class!(
/// **foaf:OnlineAccount**
/// An online account.
:"http://xmlns.com/foaf/0.1/OnlineAccount", OnlineAccount,
158);
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for OnlineAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for resource::IRI<'g, OnlineAccount<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:OnlineChatAccount**
/// An online chat account.
:"http://xmlns.com/foaf/0.1/OnlineChatAccount", OnlineChatAccount,
159);
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for OnlineChatAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for resource::IRI<'g, OnlineChatAccount<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:OnlineEcommerceAccount**
/// An online e-commerce account.
:"http://xmlns.com/foaf/0.1/OnlineEcommerceAccount", OnlineEcommerceAccount,
160);
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for OnlineEcommerceAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for resource::IRI<'g, OnlineEcommerceAccount<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:OnlineGamingAccount**
/// An online gaming account.
:"http://xmlns.com/foaf/0.1/OnlineGamingAccount", OnlineGamingAccount,
161);
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountName<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AccountServiceHomepage<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Depiction<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FundedBy<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Homepage<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IsPrimaryTopicOf<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Logo<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Maker<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Name<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Page<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for OnlineGamingAccount<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Theme<'g> for resource::IRI<'g, OnlineGamingAccount<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Organization**
/// An organization.
:"http://xmlns.com/foaf/0.1/Organization", Organization,
162);
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for Organization<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for resource::IRI<'g, Organization<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Person**
/// A person.
:"http://xmlns.com/foaf/0.1/Person", Person,
163);
impl<'g, G: 'g> ontology::properties::foaf::CurrentProject<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::CurrentProject<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FamilyName<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FamilyName<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FirstName<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::FirstName<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Geekcode<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Geekcode<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Img<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Img<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Knows<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Knows<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::LastName<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::LastName<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MyersBriggs<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MyersBriggs<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PastProject<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PastProject<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Plan<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Plan<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Publications<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Publications<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SchoolHomepage<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SchoolHomepage<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Surname<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Surname<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::WorkInfoHomepage<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::WorkInfoHomepage<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::WorkplaceHomepage<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::WorkplaceHomepage<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Account<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Age<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::AimChatID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Birthday<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Gender<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::HoldsAccount<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::IcqChatID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Interest<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::JabberID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Made<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Mbox_sha1sum<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::MsnChatID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Openid<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::SkypeID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Status<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Tipjar<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic_interest<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Weblog<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for Person<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::YahooChatID<'g> for resource::IRI<'g, Person<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:PersonalProfileDocument**
/// A personal profile RDF document.
:"http://xmlns.com/foaf/0.1/PersonalProfileDocument", PersonalProfileDocument,
164);
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for PersonalProfileDocument<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::PrimaryTopic<'g> for resource::IRI<'g, PersonalProfileDocument<'g, G>> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for PersonalProfileDocument<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::foaf::Topic<'g> for resource::IRI<'g, PersonalProfileDocument<'g, G>> where G: graph::Graph<'g> {}

class!(
/// **foaf:Project**
/// A project (a collective endeavour of some kind).
:"http://xmlns.com/foaf/0.1/Project", Project,
165);
