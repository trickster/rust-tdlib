#[allow(unused_imports)]
use std::collections::HashSet;

use bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PremiumPaymentOption {
    // ID: typing.Literal["premiumPaymentOption"] = "premiumPaymentOption"
    pub currency: String,
    pub amount: i64,
    pub discount_percentage: i32,
    pub month_count: i32,
    pub store_product_id: String,
    pub payment_link: InternalLinkType,
}

type InternalLinkType = Document;
