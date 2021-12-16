use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAttachedStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File identifier
    file_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAttachedStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAttachedStickerSets {}

impl GetAttachedStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetAttachedStickerSetsBuilder {
        let mut inner = GetAttachedStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAttachedStickerSets".to_string();

        RTDGetAttachedStickerSetsBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[doc(hidden)]
pub struct RTDGetAttachedStickerSetsBuilder {
    inner: GetAttachedStickerSets,
}

impl RTDGetAttachedStickerSetsBuilder {
    pub fn build(&self) -> GetAttachedStickerSets {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }
}

impl AsRef<GetAttachedStickerSets> for GetAttachedStickerSets {
    fn as_ref(&self) -> &GetAttachedStickerSets {
        self
    }
}

impl AsRef<GetAttachedStickerSets> for RTDGetAttachedStickerSetsBuilder {
    fn as_ref(&self) -> &GetAttachedStickerSets {
        &self.inner
    }
}
