use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns current verbosity level of the internal logging of TDLib. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogVerbosityLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLogVerbosityLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLogVerbosityLevel {}

impl GetLogVerbosityLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLogVerbosityLevelBuilder {
        let mut inner = GetLogVerbosityLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogVerbosityLevel".to_string();

        RTDGetLogVerbosityLevelBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetLogVerbosityLevelBuilder {
    inner: GetLogVerbosityLevel,
}

impl RTDGetLogVerbosityLevelBuilder {
    pub fn build(&self) -> GetLogVerbosityLevel {
        self.inner.clone()
    }
}

impl AsRef<GetLogVerbosityLevel> for GetLogVerbosityLevel {
    fn as_ref(&self) -> &GetLogVerbosityLevel {
        self
    }
}

impl AsRef<GetLogVerbosityLevel> for RTDGetLogVerbosityLevelBuilder {
    fn as_ref(&self) -> &GetLogVerbosityLevel {
        &self.inner
    }
}
