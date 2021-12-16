use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns recommended chat filters for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecommendedChatFilters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecommendedChatFilters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecommendedChatFilters {}

impl GetRecommendedChatFilters {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetRecommendedChatFiltersBuilder {
        let mut inner = GetRecommendedChatFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecommendedChatFilters".to_string();

        RTDGetRecommendedChatFiltersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetRecommendedChatFiltersBuilder {
    inner: GetRecommendedChatFilters,
}

impl RTDGetRecommendedChatFiltersBuilder {
    pub fn build(&self) -> GetRecommendedChatFilters {
        self.inner.clone()
    }
}

impl AsRef<GetRecommendedChatFilters> for GetRecommendedChatFilters {
    fn as_ref(&self) -> &GetRecommendedChatFilters {
        self
    }
}

impl AsRef<GetRecommendedChatFilters> for RTDGetRecommendedChatFiltersBuilder {
    fn as_ref(&self) -> &GetRecommendedChatFilters {
        &self.inner
    }
}
