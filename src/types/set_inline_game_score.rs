use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Updates the game score of the specified user in a game; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetInlineGameScore {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline message identifier
    inline_message_id: String,
    /// True, if the message should be edited
    edit_message: bool,
    /// User identifier
    user_id: i32,
    /// The new score
    score: i32,
    /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
    force: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetInlineGameScore {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetInlineGameScore {}

impl SetInlineGameScore {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetInlineGameScoreBuilder {
        let mut inner = SetInlineGameScore::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setInlineGameScore".to_string();

        RTDSetInlineGameScoreBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn edit_message(&self) -> bool {
        self.edit_message
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn force(&self) -> bool {
        self.force
    }
}

#[doc(hidden)]
pub struct RTDSetInlineGameScoreBuilder {
    inner: SetInlineGameScore,
}

impl RTDSetInlineGameScoreBuilder {
    pub fn build(&self) -> SetInlineGameScore {
        self.inner.clone()
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }

    pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
        self.inner.edit_message = edit_message;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn score(&mut self, score: i32) -> &mut Self {
        self.inner.score = score;
        self
    }

    pub fn force(&mut self, force: bool) -> &mut Self {
        self.inner.force = force;
        self
    }
}

impl AsRef<SetInlineGameScore> for SetInlineGameScore {
    fn as_ref(&self) -> &SetInlineGameScore {
        self
    }
}

impl AsRef<SetInlineGameScore> for RTDSetInlineGameScoreBuilder {
    fn as_ref(&self) -> &SetInlineGameScore {
        &self.inner
    }
}
