use uuid::Uuid;

pub enum ConversationType{
	Direct,
	Group
}
pub struct GroupConversation{
	members: Vec<Uuid>,
	admin: Uuid
}