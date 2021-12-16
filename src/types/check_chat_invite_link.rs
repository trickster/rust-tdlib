use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Checks the validity of an invite link for a chat and returns information about the corresponding chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Invite link to be checked; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/"
    invite_link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckChatInviteLink {}

impl CheckChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCheckChatInviteLinkBuilder {
        let mut inner = CheckChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkChatInviteLink".to_string();

        RTDCheckChatInviteLinkBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct RTDCheckChatInviteLinkBuilder {
    inner: CheckChatInviteLink,
}

impl RTDCheckChatInviteLinkBuilder {
    pub fn build(&self) -> CheckChatInviteLink {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<CheckChatInviteLink> for CheckChatInviteLink {
    fn as_ref(&self) -> &CheckChatInviteLink {
        self
    }
}

impl AsRef<CheckChatInviteLink> for RTDCheckChatInviteLinkBuilder {
    fn as_ref(&self) -> &CheckChatInviteLink {
        &self.inner
    }
}
