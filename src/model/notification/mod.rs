// MIT License

// Copyright (c) 2024 podzito

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{anyhow, Result};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::get_str;

use self::subscription::{
    subscription_canceled::SubscriptionCanceled, subscription_created::SubscriptionCreated,
    subscription_updated::SubscriptionUpdated,
};

pub mod subscription;

#[derive(Debug, Builder, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub event_id: String,
    pub notification_id: String,
    pub event: Event,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
}

impl TryFrom<Value> for Notification {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        let event_id = get_str(&value, "event_id")?;
        let notification_id = get_str(&value, "notification_id")?;
        let occurred_at: OffsetDateTime =
            OffsetDateTime::parse(&get_str(&value, "occurred_at")?, &Rfc3339)?;

        let event = value.try_into()?;

        let notification = NotificationBuilder::default()
            .event_id(event_id)
            .notification_id(notification_id)
            .occurred_at(occurred_at)
            .event(event)
            .build()?;

        Ok(notification)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Event {
    SubscriptionCreated(SubscriptionCreated),
    SubscriptionCanceled(SubscriptionCanceled),
    SubscriptionUpdated(SubscriptionUpdated),
}

impl TryFrom<Value> for Event {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let event_type = get_str(&value, "event_type")?;
        let data = value.get("data").ok_or(anyhow!("Missing data"))?;

        if event_type == "subscription.created" {
            let subscription_created = serde_json::from_value(data.clone())?;
            Ok(Event::SubscriptionCreated(subscription_created))
        } else if event_type == "subscription.canceled" {
            let subscription_canceled = serde_json::from_value(data.clone())?;
            Ok(Event::SubscriptionCanceled(subscription_canceled))
        } else if event_type == "subscription.updated" {
            let subscription_updated = serde_json::from_value(data.clone())?;
            Ok(Event::SubscriptionUpdated(subscription_updated))
        } else {
            Err(anyhow!("Unknown event type: {}", event_type))
        }
    }
}
