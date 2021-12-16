use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains statistics about messages sent by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageSenderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier
    user_id: i32,
    /// Number of sent messages
    sent_message_count: i32,
    /// Average number of characters in sent messages
    average_character_count: i32,
}

impl RObject for ChatStatisticsMessageSenderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatStatisticsMessageSenderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatStatisticsMessageSenderInfoBuilder {
        let mut inner = ChatStatisticsMessageSenderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatStatisticsMessageSenderInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn sent_message_count(&self) -> i32 {
        self.sent_message_count
    }

    pub fn average_character_count(&self) -> i32 {
        self.average_character_count
    }
}

#[doc(hidden)]
pub struct RTDChatStatisticsMessageSenderInfoBuilder {
    inner: ChatStatisticsMessageSenderInfo,
}

impl RTDChatStatisticsMessageSenderInfoBuilder {
    pub fn build(&self) -> ChatStatisticsMessageSenderInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn sent_message_count(&mut self, sent_message_count: i32) -> &mut Self {
        self.inner.sent_message_count = sent_message_count;
        self
    }

    pub fn average_character_count(&mut self, average_character_count: i32) -> &mut Self {
        self.inner.average_character_count = average_character_count;
        self
    }
}

impl AsRef<ChatStatisticsMessageSenderInfo> for ChatStatisticsMessageSenderInfo {
    fn as_ref(&self) -> &ChatStatisticsMessageSenderInfo {
        self
    }
}

impl AsRef<ChatStatisticsMessageSenderInfo> for RTDChatStatisticsMessageSenderInfoBuilder {
    fn as_ref(&self) -> &ChatStatisticsMessageSenderInfo {
        &self.inner
    }
}
