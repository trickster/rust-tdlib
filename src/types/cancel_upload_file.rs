use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelUploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to stop uploading

    #[serde(default)]
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelUploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelUploadFile {}

impl CancelUploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CancelUploadFileBuilder {
        let mut inner = CancelUploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelUploadFile".to_string();

        CancelUploadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct CancelUploadFileBuilder {
    inner: CancelUploadFile,
}

#[deprecated]
pub type RTDCancelUploadFileBuilder = CancelUploadFileBuilder;

impl CancelUploadFileBuilder {
    pub fn build(&self) -> CancelUploadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<CancelUploadFile> for CancelUploadFile {
    fn as_ref(&self) -> &CancelUploadFile {
        self
    }
}

impl AsRef<CancelUploadFile> for CancelUploadFileBuilder {
    fn as_ref(&self) -> &CancelUploadFile {
        &self.inner
    }
}

//-------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelPreliminaryUploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to stop uploading

    #[serde(default)]
    file_id: i32,
    // #[serde(default)]
    // only_if_pending: bool,
    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelPreliminaryUploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelPreliminaryUploadFile {}

impl CancelPreliminaryUploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CancelPreliminaryUploadFileBuilder {
        let mut inner = CancelPreliminaryUploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelPreliminaryUploadFile".to_string();

        CancelPreliminaryUploadFileBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
    // pub fn only_if_pending(&self) -> bool {
    //     self.only_if_pending
    // }
}

#[doc(hidden)]
pub struct CancelPreliminaryUploadFileBuilder {
    inner: CancelPreliminaryUploadFile,
}

#[deprecated]
pub type RTDCancelPreliminaryUploadFileBuilder = CancelPreliminaryUploadFileBuilder;

impl CancelPreliminaryUploadFileBuilder {
    pub fn build(&self) -> CancelPreliminaryUploadFile {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;

        self
    }
    // pub fn only_if_pending(&mut self, only_if_pending: bool) -> &mut Self {
    //     self.inner.only_if_pending = only_if_pending;
    //     self
    // }
}

impl AsRef<CancelPreliminaryUploadFile> for CancelPreliminaryUploadFile {
    fn as_ref(&self) -> &CancelPreliminaryUploadFile {
        self
    }
}

impl AsRef<CancelPreliminaryUploadFile> for CancelPreliminaryUploadFileBuilder {
    fn as_ref(&self) -> &CancelPreliminaryUploadFile {
        &self.inner
    }
}
