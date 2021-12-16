use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Provides information about a bot and its supported commands
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Provides information about a bot and its supported commands
    description: String,
    /// A list of commands supported by the bot
    commands: Vec<BotCommand>,
}

impl RObject for BotInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BotInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotInfoBuilder {
        let mut inner = BotInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotInfoBuilder { inner }
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn commands(&self) -> &Vec<BotCommand> {
        &self.commands
    }
}

#[doc(hidden)]
pub struct RTDBotInfoBuilder {
    inner: BotInfo,
}

impl RTDBotInfoBuilder {
    pub fn build(&self) -> BotInfo {
        self.inner.clone()
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
        self.inner.commands = commands;
        self
    }
}

impl AsRef<BotInfo> for BotInfo {
    fn as_ref(&self) -> &BotInfo {
        self
    }
}

impl AsRef<BotInfo> for RTDBotInfoBuilder {
    fn as_ref(&self) -> &BotInfo {
        &self.inner
    }
}
