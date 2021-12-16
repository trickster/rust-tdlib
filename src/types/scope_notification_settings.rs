use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for several chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before notifications will be unmuted, in seconds
    mute_for: i32,
    /// The name of an audio file to be used for notification sounds; only applies to iOS applications
    sound: String,
    /// True, if message content should be displayed in notifications
    show_preview: bool,
    /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message
    disable_pinned_message_notifications: bool,
    /// True, if notifications for messages with mentions will be created as for an ordinary unread message
    disable_mention_notifications: bool,
}

impl RObject for ScopeNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ScopeNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDScopeNotificationSettingsBuilder {
        let mut inner = ScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDScopeNotificationSettingsBuilder { inner }
    }

    pub fn mute_for(&self) -> i32 {
        self.mute_for
    }

    pub fn sound(&self) -> &String {
        &self.sound
    }

    pub fn show_preview(&self) -> bool {
        self.show_preview
    }

    pub fn disable_pinned_message_notifications(&self) -> bool {
        self.disable_pinned_message_notifications
    }

    pub fn disable_mention_notifications(&self) -> bool {
        self.disable_mention_notifications
    }
}

#[doc(hidden)]
pub struct RTDScopeNotificationSettingsBuilder {
    inner: ScopeNotificationSettings,
}

impl RTDScopeNotificationSettingsBuilder {
    pub fn build(&self) -> ScopeNotificationSettings {
        self.inner.clone()
    }

    pub fn mute_for(&mut self, mute_for: i32) -> &mut Self {
        self.inner.mute_for = mute_for;
        self
    }

    pub fn sound<T: AsRef<str>>(&mut self, sound: T) -> &mut Self {
        self.inner.sound = sound.as_ref().to_string();
        self
    }

    pub fn show_preview(&mut self, show_preview: bool) -> &mut Self {
        self.inner.show_preview = show_preview;
        self
    }

    pub fn disable_pinned_message_notifications(
        &mut self,
        disable_pinned_message_notifications: bool,
    ) -> &mut Self {
        self.inner.disable_pinned_message_notifications = disable_pinned_message_notifications;
        self
    }

    pub fn disable_mention_notifications(
        &mut self,
        disable_mention_notifications: bool,
    ) -> &mut Self {
        self.inner.disable_mention_notifications = disable_mention_notifications;
        self
    }
}

impl AsRef<ScopeNotificationSettings> for ScopeNotificationSettings {
    fn as_ref(&self) -> &ScopeNotificationSettings {
        self
    }
}

impl AsRef<ScopeNotificationSettings> for RTDScopeNotificationSettingsBuilder {
    fn as_ref(&self) -> &ScopeNotificationSettings {
        &self.inner
    }
}
