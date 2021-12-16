use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// One shipping option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Shipping option identifier
    id: String,
    /// Option title
    title: String,
    /// A list of objects used to calculate the total shipping costs
    price_parts: Vec<LabeledPricePart>,
}

impl RObject for ShippingOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ShippingOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDShippingOptionBuilder {
        let mut inner = ShippingOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDShippingOptionBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn price_parts(&self) -> &Vec<LabeledPricePart> {
        &self.price_parts
    }
}

#[doc(hidden)]
pub struct RTDShippingOptionBuilder {
    inner: ShippingOption,
}

impl RTDShippingOptionBuilder {
    pub fn build(&self) -> ShippingOption {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn price_parts(&mut self, price_parts: Vec<LabeledPricePart>) -> &mut Self {
        self.inner.price_parts = price_parts;
        self
    }
}

impl AsRef<ShippingOption> for ShippingOption {
    fn as_ref(&self) -> &ShippingOption {
        self
    }
}

impl AsRef<ShippingOption> for RTDShippingOptionBuilder {
    fn as_ref(&self) -> &ShippingOption {
        &self.inner
    }
}
