use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestQrCodeAuthentication {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of user identifiers of other users currently using the application
    other_user_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RequestQrCodeAuthentication {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RequestQrCodeAuthentication {}

impl RequestQrCodeAuthentication {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRequestQrCodeAuthenticationBuilder {
        let mut inner = RequestQrCodeAuthentication::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "requestQrCodeAuthentication".to_string();

        RTDRequestQrCodeAuthenticationBuilder { inner }
    }

    pub fn other_user_ids(&self) -> &Vec<i32> {
        &self.other_user_ids
    }
}

#[doc(hidden)]
pub struct RTDRequestQrCodeAuthenticationBuilder {
    inner: RequestQrCodeAuthentication,
}

impl RTDRequestQrCodeAuthenticationBuilder {
    pub fn build(&self) -> RequestQrCodeAuthentication {
        self.inner.clone()
    }

    pub fn other_user_ids(&mut self, other_user_ids: Vec<i32>) -> &mut Self {
        self.inner.other_user_ids = other_user_ids;
        self
    }
}

impl AsRef<RequestQrCodeAuthentication> for RequestQrCodeAuthentication {
    fn as_ref(&self) -> &RequestQrCodeAuthentication {
        self
    }
}

impl AsRef<RequestQrCodeAuthentication> for RTDRequestQrCodeAuthenticationBuilder {
    fn as_ref(&self) -> &RequestQrCodeAuthentication {
        &self.inner
    }
}
