use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a tg:// deep link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeepLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text to be shown to the user
    text: FormattedText,
    /// True, if user should be asked to update the application
    need_update_application: bool,
}

impl RObject for DeepLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DeepLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeepLinkInfoBuilder {
        let mut inner = DeepLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDDeepLinkInfoBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn need_update_application(&self) -> bool {
        self.need_update_application
    }
}

#[doc(hidden)]
pub struct RTDDeepLinkInfoBuilder {
    inner: DeepLinkInfo,
}

impl RTDDeepLinkInfoBuilder {
    pub fn build(&self) -> DeepLinkInfo {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn need_update_application(&mut self, need_update_application: bool) -> &mut Self {
        self.inner.need_update_application = need_update_application;
        self
    }
}

impl AsRef<DeepLinkInfo> for DeepLinkInfo {
    fn as_ref(&self) -> &DeepLinkInfo {
        self
    }
}

impl AsRef<DeepLinkInfo> for RTDDeepLinkInfoBuilder {
    fn as_ref(&self) -> &DeepLinkInfo {
        &self.inner
    }
}
