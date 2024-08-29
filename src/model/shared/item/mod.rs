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

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use self::{price::Price, product::Product};

pub mod price;
pub mod product;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "trialing")]
    Trialing,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "past_due")]
    PastDue,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrialDates {
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ends_at: OffsetDateTime,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

pub struct Item {
    pub price: Price,
    pub product: Product,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "crate::util::serde::option_rfc3339")]
    pub next_billed_at: Option<OffsetDateTime>,
    #[serde(with = "crate::util::serde::option_rfc3339")]
    pub previously_billed_at: Option<OffsetDateTime>,
    pub quantity: u32,
    pub recurring: bool,
    pub status: Status,
    pub trial_dates: Option<TrialDates>,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}
