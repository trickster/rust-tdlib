use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes format of the thumbnail
pub trait TDThumbnailFormat: Debug + RObject {}

/// Describes format of the thumbnail
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThumbnailFormat {
    #[doc(hidden)]
    _Default,
    /// The thumbnail is in static GIF format. It will be used only for some bot inline results
    #[serde(rename(serialize = "thumbnailFormatGif", deserialize = "thumbnailFormatGif"))]
    Gif(ThumbnailFormatGif),
    /// The thumbnail is in JPEG format
    #[serde(rename(serialize = "thumbnailFormatJpeg", deserialize = "thumbnailFormatJpeg"))]
    Jpeg(ThumbnailFormatJpeg),
    /// The thumbnail is in MPEG4 format. It will be used only for some animations and videos
    #[serde(rename(
        serialize = "thumbnailFormatMpeg4",
        deserialize = "thumbnailFormatMpeg4"
    ))]
    Mpeg4(ThumbnailFormatMpeg4),
    /// The thumbnail is in PNG format. It will be used only for background patterns
    #[serde(rename(serialize = "thumbnailFormatPng", deserialize = "thumbnailFormatPng"))]
    Png(ThumbnailFormatPng),
    /// The thumbnail is in TGS format. It will be used only for animated sticker sets
    #[serde(rename(serialize = "thumbnailFormatTgs", deserialize = "thumbnailFormatTgs"))]
    Tgs(ThumbnailFormatTgs),
    /// The thumbnail is in WEBP format. It will be used only for some stickers
    #[serde(rename(serialize = "thumbnailFormatWebp", deserialize = "thumbnailFormatWebp"))]
    Webp(ThumbnailFormatWebp),
}

impl Default for ThumbnailFormat {
    fn default() -> Self {
        ThumbnailFormat::_Default
    }
}

impl RObject for ThumbnailFormat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ThumbnailFormat::Gif(t) => t.extra(),
            ThumbnailFormat::Jpeg(t) => t.extra(),
            ThumbnailFormat::Mpeg4(t) => t.extra(),
            ThumbnailFormat::Png(t) => t.extra(),
            ThumbnailFormat::Tgs(t) => t.extra(),
            ThumbnailFormat::Webp(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ThumbnailFormat::Gif(t) => t.client_id(),
            ThumbnailFormat::Jpeg(t) => t.client_id(),
            ThumbnailFormat::Mpeg4(t) => t.client_id(),
            ThumbnailFormat::Png(t) => t.client_id(),
            ThumbnailFormat::Tgs(t) => t.client_id(),
            ThumbnailFormat::Webp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ThumbnailFormat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ThumbnailFormat::_Default)
    }
}

impl AsRef<ThumbnailFormat> for ThumbnailFormat {
    fn as_ref(&self) -> &ThumbnailFormat {
        self
    }
}

/// The thumbnail is in static GIF format. It will be used only for some bot inline results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatGif {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatGif {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatGif {}

impl ThumbnailFormatGif {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatGifBuilder {
        let mut inner = ThumbnailFormatGif::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatGifBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatGifBuilder {
    inner: ThumbnailFormatGif,
}

impl RTDThumbnailFormatGifBuilder {
    pub fn build(&self) -> ThumbnailFormatGif {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatGif> for ThumbnailFormatGif {
    fn as_ref(&self) -> &ThumbnailFormatGif {
        self
    }
}

impl AsRef<ThumbnailFormatGif> for RTDThumbnailFormatGifBuilder {
    fn as_ref(&self) -> &ThumbnailFormatGif {
        &self.inner
    }
}

/// The thumbnail is in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatJpeg {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatJpeg {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatJpeg {}

impl ThumbnailFormatJpeg {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatJpegBuilder {
        let mut inner = ThumbnailFormatJpeg::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatJpegBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatJpegBuilder {
    inner: ThumbnailFormatJpeg,
}

impl RTDThumbnailFormatJpegBuilder {
    pub fn build(&self) -> ThumbnailFormatJpeg {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatJpeg> for ThumbnailFormatJpeg {
    fn as_ref(&self) -> &ThumbnailFormatJpeg {
        self
    }
}

impl AsRef<ThumbnailFormatJpeg> for RTDThumbnailFormatJpegBuilder {
    fn as_ref(&self) -> &ThumbnailFormatJpeg {
        &self.inner
    }
}

/// The thumbnail is in MPEG4 format. It will be used only for some animations and videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatMpeg4 {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatMpeg4 {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatMpeg4 {}

impl ThumbnailFormatMpeg4 {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatMpeg4Builder {
        let mut inner = ThumbnailFormatMpeg4::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatMpeg4Builder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatMpeg4Builder {
    inner: ThumbnailFormatMpeg4,
}

impl RTDThumbnailFormatMpeg4Builder {
    pub fn build(&self) -> ThumbnailFormatMpeg4 {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatMpeg4> for ThumbnailFormatMpeg4 {
    fn as_ref(&self) -> &ThumbnailFormatMpeg4 {
        self
    }
}

impl AsRef<ThumbnailFormatMpeg4> for RTDThumbnailFormatMpeg4Builder {
    fn as_ref(&self) -> &ThumbnailFormatMpeg4 {
        &self.inner
    }
}

/// The thumbnail is in PNG format. It will be used only for background patterns
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatPng {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatPng {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatPng {}

impl ThumbnailFormatPng {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatPngBuilder {
        let mut inner = ThumbnailFormatPng::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatPngBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatPngBuilder {
    inner: ThumbnailFormatPng,
}

impl RTDThumbnailFormatPngBuilder {
    pub fn build(&self) -> ThumbnailFormatPng {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatPng> for ThumbnailFormatPng {
    fn as_ref(&self) -> &ThumbnailFormatPng {
        self
    }
}

impl AsRef<ThumbnailFormatPng> for RTDThumbnailFormatPngBuilder {
    fn as_ref(&self) -> &ThumbnailFormatPng {
        &self.inner
    }
}

/// The thumbnail is in TGS format. It will be used only for animated sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatTgs {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatTgs {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatTgs {}

impl ThumbnailFormatTgs {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatTgsBuilder {
        let mut inner = ThumbnailFormatTgs::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatTgsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatTgsBuilder {
    inner: ThumbnailFormatTgs,
}

impl RTDThumbnailFormatTgsBuilder {
    pub fn build(&self) -> ThumbnailFormatTgs {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatTgs> for ThumbnailFormatTgs {
    fn as_ref(&self) -> &ThumbnailFormatTgs {
        self
    }
}

impl AsRef<ThumbnailFormatTgs> for RTDThumbnailFormatTgsBuilder {
    fn as_ref(&self) -> &ThumbnailFormatTgs {
        &self.inner
    }
}

/// The thumbnail is in WEBP format. It will be used only for some stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatWebp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ThumbnailFormatWebp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDThumbnailFormat for ThumbnailFormatWebp {}

impl ThumbnailFormatWebp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailFormatWebpBuilder {
        let mut inner = ThumbnailFormatWebp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailFormatWebpBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDThumbnailFormatWebpBuilder {
    inner: ThumbnailFormatWebp,
}

impl RTDThumbnailFormatWebpBuilder {
    pub fn build(&self) -> ThumbnailFormatWebp {
        self.inner.clone()
    }
}

impl AsRef<ThumbnailFormatWebp> for ThumbnailFormatWebp {
    fn as_ref(&self) -> &ThumbnailFormatWebp {
        self
    }
}

impl AsRef<ThumbnailFormatWebp> for RTDThumbnailFormatWebpBuilder {
    fn as_ref(&self) -> &ThumbnailFormatWebp {
        &self.inner
    }
}
