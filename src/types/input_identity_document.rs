use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// An identity document to be saved to Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputIdentityDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Document number; 1-24 characters
    number: String,
    /// Document expiry date, if available
    expiry_date: Date,
    /// Front side of the document

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    front_side: InputFile,
    /// Reverse side of the document; only for driver license and identity card

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    reverse_side: InputFile,
    /// Selfie with the document, if available

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    selfie: InputFile,
    /// List of files containing a certified English translation of the document
    translation: Vec<InputFile>,
}

impl RObject for InputIdentityDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputIdentityDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputIdentityDocumentBuilder {
        let mut inner = InputIdentityDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputIdentityDocumentBuilder { inner }
    }

    pub fn number(&self) -> &String {
        &self.number
    }

    pub fn expiry_date(&self) -> &Date {
        &self.expiry_date
    }

    pub fn front_side(&self) -> &InputFile {
        &self.front_side
    }

    pub fn reverse_side(&self) -> &InputFile {
        &self.reverse_side
    }

    pub fn selfie(&self) -> &InputFile {
        &self.selfie
    }

    pub fn translation(&self) -> &Vec<InputFile> {
        &self.translation
    }
}

#[doc(hidden)]
pub struct RTDInputIdentityDocumentBuilder {
    inner: InputIdentityDocument,
}

impl RTDInputIdentityDocumentBuilder {
    pub fn build(&self) -> InputIdentityDocument {
        self.inner.clone()
    }

    pub fn number<T: AsRef<str>>(&mut self, number: T) -> &mut Self {
        self.inner.number = number.as_ref().to_string();
        self
    }

    pub fn expiry_date<T: AsRef<Date>>(&mut self, expiry_date: T) -> &mut Self {
        self.inner.expiry_date = expiry_date.as_ref().clone();
        self
    }

    pub fn front_side<T: AsRef<InputFile>>(&mut self, front_side: T) -> &mut Self {
        self.inner.front_side = front_side.as_ref().clone();
        self
    }

    pub fn reverse_side<T: AsRef<InputFile>>(&mut self, reverse_side: T) -> &mut Self {
        self.inner.reverse_side = reverse_side.as_ref().clone();
        self
    }

    pub fn selfie<T: AsRef<InputFile>>(&mut self, selfie: T) -> &mut Self {
        self.inner.selfie = selfie.as_ref().clone();
        self
    }

    pub fn translation(&mut self, translation: Vec<InputFile>) -> &mut Self {
        self.inner.translation = translation;
        self
    }
}

impl AsRef<InputIdentityDocument> for InputIdentityDocument {
    fn as_ref(&self) -> &InputIdentityDocument {
        self
    }
}

impl AsRef<InputIdentityDocument> for RTDInputIdentityDocumentBuilder {
    fn as_ref(&self) -> &InputIdentityDocument {
        &self.inner
    }
}
