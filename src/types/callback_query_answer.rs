use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a bot's answer to a callback query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryAnswer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the answer
    text: String,
    /// True, if an alert should be shown to the user instead of a toast notification
    show_alert: bool,
    /// URL to be opened
    url: String,
}

impl RObject for CallbackQueryAnswer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl CallbackQueryAnswer {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryAnswerBuilder {
        let mut inner = CallbackQueryAnswer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallbackQueryAnswerBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn show_alert(&self) -> bool {
        self.show_alert
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDCallbackQueryAnswerBuilder {
    inner: CallbackQueryAnswer,
}

impl RTDCallbackQueryAnswerBuilder {
    pub fn build(&self) -> CallbackQueryAnswer {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn show_alert(&mut self, show_alert: bool) -> &mut Self {
        self.inner.show_alert = show_alert;
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<CallbackQueryAnswer> for CallbackQueryAnswer {
    fn as_ref(&self) -> &CallbackQueryAnswer {
        self
    }
}

impl AsRef<CallbackQueryAnswer> for RTDCallbackQueryAnswerBuilder {
    fn as_ref(&self) -> &CallbackQueryAnswer {
        &self.inner
    }
}
