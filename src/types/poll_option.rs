use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes one answer option of a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Option text; 1-100 characters

    #[serde(default)]
    text: FormattedText,
    /// Number of voters for this option, available only for closed or voted polls

    #[serde(default)]
    voter_count: i32,
    /// The percentage of votes for this option; 0-100

    #[serde(default)]
    vote_percentage: i32,
    /// True, if the option was chosen by the user

    #[serde(default)]
    is_chosen: bool,
    /// True, if the option is being chosen by a pending setPollAnswer request

    #[serde(default)]
    is_being_chosen: bool,
}

impl RObject for PollOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PollOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PollOptionBuilder {
        let mut inner = PollOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PollOptionBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn voter_count(&self) -> i32 {
        self.voter_count
    }

    pub fn vote_percentage(&self) -> i32 {
        self.vote_percentage
    }

    pub fn is_chosen(&self) -> bool {
        self.is_chosen
    }

    pub fn is_being_chosen(&self) -> bool {
        self.is_being_chosen
    }
}

#[doc(hidden)]
pub struct PollOptionBuilder {
    inner: PollOption,
}

#[deprecated]
pub type RTDPollOptionBuilder = PollOptionBuilder;

impl PollOptionBuilder {
    pub fn build(&self) -> PollOption {
        self.inner.clone()
    }

    pub fn text(&mut self, text: FormattedText) -> &mut Self {
        self.inner.text = text;
        self
    }

    pub fn voter_count(&mut self, voter_count: i32) -> &mut Self {
        self.inner.voter_count = voter_count;
        self
    }

    pub fn vote_percentage(&mut self, vote_percentage: i32) -> &mut Self {
        self.inner.vote_percentage = vote_percentage;
        self
    }

    pub fn is_chosen(&mut self, is_chosen: bool) -> &mut Self {
        self.inner.is_chosen = is_chosen;
        self
    }

    pub fn is_being_chosen(&mut self, is_being_chosen: bool) -> &mut Self {
        self.inner.is_being_chosen = is_being_chosen;
        self
    }
}

impl AsRef<PollOption> for PollOption {
    fn as_ref(&self) -> &PollOption {
        self
    }
}

impl AsRef<PollOption> for PollOptionBuilder {
    fn as_ref(&self) -> &PollOption {
        &self.inner
    }
}
