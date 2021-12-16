use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns game high scores and some part of the high score table in the range of the specified user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInlineGameHighScores {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline message identifier
    inline_message_id: String,
    /// User identifier
    user_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInlineGameHighScores {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInlineGameHighScores {}

impl GetInlineGameHighScores {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetInlineGameHighScoresBuilder {
        let mut inner = GetInlineGameHighScores::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInlineGameHighScores".to_string();

        RTDGetInlineGameHighScoresBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDGetInlineGameHighScoresBuilder {
    inner: GetInlineGameHighScores,
}

impl RTDGetInlineGameHighScoresBuilder {
    pub fn build(&self) -> GetInlineGameHighScores {
        self.inner.clone()
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetInlineGameHighScores> for GetInlineGameHighScores {
    fn as_ref(&self) -> &GetInlineGameHighScores {
        self
    }
}

impl AsRef<GetInlineGameHighScores> for RTDGetInlineGameHighScoresBuilder {
    fn as_ref(&self) -> &GetInlineGameHighScores {
        &self.inner
    }
}
