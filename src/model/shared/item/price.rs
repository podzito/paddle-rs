// MIT License
//
// Copyright (c) 2024 podzito
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::shared::subscription::{BillingCycle, Interval, Quantity};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Currency {
    USD,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ItemPrice {
    pub amount: String,
    pub currency_code: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxMode {
    #[serde(rename = "account_setting")]
    AccountSetting,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrialPeriod {
    frequency: u32,
    interval: Interval,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "standard")]
    Standard,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Price {
    pub billing_cycle: BillingCycle,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub custom_data: Option<HashMap<String, String>>,
    pub description: String,
    pub id: String,
    pub import_meta: Option<String>,
    pub name: String,
    pub product_id: String,
    pub quantity: Quantity,
    pub seller_id: String,
    pub status: Status,
    pub tax_mode: TaxMode,
    pub trial_period: TrialPeriod,
    #[serde(rename = "type")]
    pub _type: ItemType,
    pub unit_price: ItemPrice,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}
