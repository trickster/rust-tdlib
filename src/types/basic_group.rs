use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group identifier
    id: i32,
    /// Number of members in the group
    member_count: i32,
    /// Status of the current user in the group

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,
    /// True, if the group is active
    is_active: bool,
    /// Identifier of the supergroup to which this group was upgraded; 0 if none
    upgraded_to_supergroup_id: i32,
}

impl RObject for BasicGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BasicGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBasicGroupBuilder {
        let mut inner = BasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBasicGroupBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn member_count(&self) -> i32 {
        self.member_count
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn upgraded_to_supergroup_id(&self) -> i32 {
        self.upgraded_to_supergroup_id
    }
}

#[doc(hidden)]
pub struct RTDBasicGroupBuilder {
    inner: BasicGroup,
}

impl RTDBasicGroupBuilder {
    pub fn build(&self) -> BasicGroup {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn member_count(&mut self, member_count: i32) -> &mut Self {
        self.inner.member_count = member_count;
        self
    }

    pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.inner.is_active = is_active;
        self
    }

    pub fn upgraded_to_supergroup_id(&mut self, upgraded_to_supergroup_id: i32) -> &mut Self {
        self.inner.upgraded_to_supergroup_id = upgraded_to_supergroup_id;
        self
    }
}

impl AsRef<BasicGroup> for BasicGroup {
    fn as_ref(&self) -> &BasicGroup {
        self
    }
}

impl AsRef<BasicGroup> for RTDBasicGroupBuilder {
    fn as_ref(&self) -> &BasicGroup {
        &self.inner
    }
}
