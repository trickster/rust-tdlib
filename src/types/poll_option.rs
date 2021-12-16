use crate::errors::*;
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
    /// Option text, 1-100 characters
    text: String,
    /// Number of voters for this option, available only for closed or voted polls
    voter_count: i32,
    /// The percentage of votes for this option, 0-100
    vote_percentage: i32,
    /// True, if the option was chosen by the user
    is_chosen: bool,
    /// True, if the option is being chosen by a pending setPollAnswer request
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPollOptionBuilder {
        let mut inner = PollOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPollOptionBuilder { inner }
    }

    pub fn text(&self) -> &String {
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
pub struct RTDPollOptionBuilder {
    inner: PollOption,
}

impl RTDPollOptionBuilder {
    pub fn build(&self) -> PollOption {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
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

impl AsRef<PollOption> for RTDPollOptionBuilder {
    fn as_ref(&self) -> &PollOption {
        &self.inner
    }
}
