use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a fill of a background
pub trait TDBackgroundFill: Debug + RObject {}

/// Describes a fill of a background
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundFill {
    #[doc(hidden)]
    _Default,

    #[serde(rename(
        serialize = "backgroundFillFreeformGradient",
        deserialize = "backgroundFillFreeformGradient"
    ))]
    FreeformGradient(BackgroundFillFreeformGradient),

    /// Describes a gradient fill of a background
    #[serde(rename(
        serialize = "backgroundFillGradient",
        deserialize = "backgroundFillGradient"
    ))]
    Gradient(BackgroundFillGradient),
    /// Describes a solid fill of a background
    #[serde(rename(serialize = "backgroundFillSolid", deserialize = "backgroundFillSolid"))]
    Solid(BackgroundFillSolid),
}

impl Default for BackgroundFill {
    fn default() -> Self {
        BackgroundFill::_Default
    }
}

impl RObject for BackgroundFill {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            BackgroundFill::Gradient(t) => t.extra(),
            BackgroundFill::Solid(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            BackgroundFill::Gradient(t) => t.client_id(),
            BackgroundFill::Solid(t) => t.client_id(),

            _ => None,
        }
    }
}

impl BackgroundFill {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BackgroundFill::_Default)
    }
}

impl AsRef<BackgroundFill> for BackgroundFill {
    fn as_ref(&self) -> &BackgroundFill {
        self
    }
}

/// Describes a gradient fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A top color of the background in the RGB24 format
    top_color: i32,
    /// A bottom color of the background in the RGB24 format
    bottom_color: i32,
    /// Clockwise rotation angle of the gradient, in degrees; 0-359. Should be always divisible by 45
    rotation_angle: i32,
}

impl RObject for BackgroundFillGradient {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundFill for BackgroundFillGradient {}

impl BackgroundFillGradient {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundFillGradientBuilder {
        let mut inner = BackgroundFillGradient::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundFillGradientBuilder { inner }
    }

    pub fn top_color(&self) -> i32 {
        self.top_color
    }

    pub fn bottom_color(&self) -> i32 {
        self.bottom_color
    }

    pub fn rotation_angle(&self) -> i32 {
        self.rotation_angle
    }
}

#[doc(hidden)]
pub struct RTDBackgroundFillGradientBuilder {
    inner: BackgroundFillGradient,
}

impl RTDBackgroundFillGradientBuilder {
    pub fn build(&self) -> BackgroundFillGradient {
        self.inner.clone()
    }

    pub fn top_color(&mut self, top_color: i32) -> &mut Self {
        self.inner.top_color = top_color;
        self
    }

    pub fn bottom_color(&mut self, bottom_color: i32) -> &mut Self {
        self.inner.bottom_color = bottom_color;
        self
    }

    pub fn rotation_angle(&mut self, rotation_angle: i32) -> &mut Self {
        self.inner.rotation_angle = rotation_angle;
        self
    }
}

impl AsRef<BackgroundFillGradient> for BackgroundFillGradient {
    fn as_ref(&self) -> &BackgroundFillGradient {
        self
    }
}

impl AsRef<BackgroundFillGradient> for RTDBackgroundFillGradientBuilder {
    fn as_ref(&self) -> &BackgroundFillGradient {
        &self.inner
    }
}

/// Describes a solid fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A color of the background in the RGB24 format
    color: i32,
}

impl RObject for BackgroundFillSolid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundFill for BackgroundFillSolid {}

impl BackgroundFillSolid {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundFillSolidBuilder {
        let mut inner = BackgroundFillSolid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundFillSolidBuilder { inner }
    }

    pub fn color(&self) -> i32 {
        self.color
    }
}

#[doc(hidden)]
pub struct RTDBackgroundFillSolidBuilder {
    inner: BackgroundFillSolid,
}

impl RTDBackgroundFillSolidBuilder {
    pub fn build(&self) -> BackgroundFillSolid {
        self.inner.clone()
    }

    pub fn color(&mut self, color: i32) -> &mut Self {
        self.inner.color = color;
        self
    }
}

impl AsRef<BackgroundFillSolid> for BackgroundFillSolid {
    fn as_ref(&self) -> &BackgroundFillSolid {
        self
    }
}

impl AsRef<BackgroundFillSolid> for RTDBackgroundFillSolidBuilder {
    fn as_ref(&self) -> &BackgroundFillSolid {
        &self.inner
    }
}

//-----------begin for tdlib1.8--------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillFreeformGradient {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    /// A list of 3 or 4 colors of the freeform gradients in the RGB24 format
    colors: Vec<i64>,
}

impl RObject for BackgroundFillFreeformGradient {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBackgroundFill for BackgroundFillFreeformGradient {}

impl BackgroundFillFreeformGradient {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundFillFreeformGradientBuilder {
        let mut inner = BackgroundFillFreeformGradient::default();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDBackgroundFillFreeformGradientBuilder { inner }
    }

    pub fn colors(&self) -> &Vec<i64> {
        &self.colors
    }
}

#[doc(hidden)]
pub struct RTDBackgroundFillFreeformGradientBuilder {
    inner: BackgroundFillFreeformGradient,
}

impl RTDBackgroundFillFreeformGradientBuilder {
    pub fn build(&self) -> BackgroundFillFreeformGradient {
        self.inner.clone()
    }

    pub fn colors(&mut self, colors: Vec<i64>) -> &mut Self {
        self.inner.colors = colors;
        self
    }
}

impl AsRef<BackgroundFillFreeformGradient> for BackgroundFillFreeformGradient {
    fn as_ref(&self) -> &BackgroundFillFreeformGradient {
        self
    }
}

impl AsRef<BackgroundFillFreeformGradient> for RTDBackgroundFillFreeformGradientBuilder {
    fn as_ref(&self) -> &BackgroundFillFreeformGradient {
        &self.inner
    }
}
//---end--------form tdlib1.8
