use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddLocalMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Target chat
    chat_id: i64,
    /// The sender sender of the message

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender: MessageSender,
    /// Identifier of the message to reply to or 0
    reply_to_message_id: i64,
    /// Pass true to disable notification for the message
    disable_notification: bool,
    /// The content of the message to be added

    #[serde(skip_serializing_if = "InputMessageContent::_is_default")]
    input_message_content: InputMessageContent,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddLocalMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddLocalMessage {}

impl AddLocalMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddLocalMessageBuilder {
        let mut inner = AddLocalMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addLocalMessage".to_string();

        RTDAddLocalMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn sender(&self) -> &MessageSender {
        &self.sender
    }

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    pub fn disable_notification(&self) -> bool {
        self.disable_notification
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDAddLocalMessageBuilder {
    inner: AddLocalMessage,
}

impl RTDAddLocalMessageBuilder {
    pub fn build(&self) -> AddLocalMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn sender<T: AsRef<MessageSender>>(&mut self, sender: T) -> &mut Self {
        self.inner.sender = sender.as_ref().clone();
        self
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }

    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.inner.disable_notification = disable_notification;
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<AddLocalMessage> for AddLocalMessage {
    fn as_ref(&self) -> &AddLocalMessage {
        self
    }
}

impl AsRef<AddLocalMessage> for RTDAddLocalMessageBuilder {
    fn as_ref(&self) -> &AddLocalMessage {
        &self.inner
    }
}
