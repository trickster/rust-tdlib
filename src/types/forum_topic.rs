use crate::types::{ChatNotificationSettings, DraftMessage, Message, MessageSender};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(tag = "forum_topic")]
pub struct ForumTopic {
    #[serde(rename(serialize = "@type"))]
    #[serde(default)]
    td_type: String,
    #[serde(default)]
    info: ForumTopicInfo,
    #[serde(default)]
    unread_count: i32,
    #[serde(default)]
    last_read_inbox_message_id: i64,
    #[serde(default)]
    last_read_outbox_message_id: i64,
    #[serde(default)]
    unread_mention_count: i64,
    #[serde(default)]
    unread_reaction_count: i64,
    #[serde(default)]
    notification_settings: ChatNotificationSettings,
    // notification_settings: serde_json::Value,
    last_message: Option<Message>,
    draft_message: Option<DraftMessage>,
    #[serde(default)]
    is_pinned: bool,
}
impl ForumTopic {
    pub fn info(&self) -> &ForumTopicInfo {
        &self.info
    }
    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }
    pub fn last_read_inbox_message_id(&self) -> i64 {
        self.last_read_inbox_message_id
    }
    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }
    pub fn unread_mention_count(&self) -> i64 {
        self.unread_mention_count
    }
    pub fn unread_reaction_count(&self) -> i64 {
        self.unread_reaction_count
    }
    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }
    pub fn last_message(&self) -> &Option<Message> {
        &self.last_message
    }
    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }
    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}
//-------------------------------------

//-------------------------------------

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(tag = "forumTopics")]
pub struct ForumTopicInfo {
    //forumTopicInfo
    #[serde(default)]
    message_thread_id: i64,
    #[serde(default)]
    name: String,

    icon: Option<ForumTopicIcon>,
    #[serde(default)]
    creation_date: i64,

    creator_id: MessageSender,
    #[serde(default)]
    is_general: bool,
    #[serde(default)]
    is_outgoing: bool,
    #[serde(default)]
    is_closed: bool,
    #[serde(default)]
    is_hidden: bool,
}
impl ForumTopicInfo {
    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    fn icon(&self) -> &Option<ForumTopicIcon> {
        &self.icon
    }
    fn creation_date(&self) -> i64 {
        self.creation_date
    }

    fn creator_id(&self) -> &MessageSender {
        &self.creator_id
    }
    fn is_general(&self) -> bool {
        self.is_general
    }
    fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }
    fn is_closed(&self) -> bool {
        self.is_closed
    }
    fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}
//-------------------------------------
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(tag = "forumTopics")]
pub struct ForumTopicIcon {
    // ID: typing.Literal["forumTopicIcon"] = Field("forumTopicIcon", validation_alias="@type", alias="@type")
    #[serde(default)]
    color: i32,
    #[serde(default)]
    custom_emoji_id: String,
}
impl ForumTopicIcon {
    fn color(&self) -> i32 {
        self.color
    }
    fn custom_emoji_id(&self) -> &String {
        &self.custom_emoji_id
    }
}

//-------------------------------------
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
// #[serde(tag = "forum_topic")]
pub struct ForumTopics {
    // ID: typing.Literal["forumTopics"] = Field("forumTopics", validation_alias="@type", alias="@type")
    #[serde(default)]
    total_count: i32,
    #[serde(default)]
    topics: Vec<ForumTopic>,
    #[serde(default)]
    next_offset_date: i32,
    #[serde(default)]
    next_offset_message_id: i64,
    #[serde(default)]
    next_offset_message_thread_id: i64,
}
impl ForumTopics {
    pub fn total_count(&self) -> i32 {
        self.total_count
    }
    pub fn topics(&self) -> &Vec<ForumTopic> {
        &self.topics
    }
    pub fn next_offset_date(&self) -> i32 {
        self.next_offset_date
    }
    pub fn next_offset_message_id(&self) -> i64 {
        self.next_offset_message_id
    }
    pub fn next_offset_message_thread_id(&self) -> i64 {
        self.next_offset_message_thread_id
    }
}
//-------------------------------------
// GetForumTopics
