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

use anyhow::Result;
use derive_builder::Builder;
use serde::Serialize;
use serde_json::Value;

use crate::notifications::helpers::types::EventName;

#[derive(Debug, Builder, Clone, Serialize)]
pub struct Notification {
  pub id: String,
  pub _type: EventName,
  // status: NotificationStatus;
  // payload: IEvents;
  pub occurred_at: String,
  pub delivered_at: Option<String>,
  pub replayed_at: Option<String>,
  // origin: Origin;
  pub last_attempt_at: Option<String>,
  pub retry_at: Option<String>,
  pub times_attempted: i32,
  pub notification_setting_id: String,
}

impl TryFrom<Value> for Notification {
  type Error = anyhow::Error;

  fn try_from(_value: Value) -> Result<Self> {
    let builder = NotificationBuilder::default();
    Ok(builder.build()?)
  }
}

#[derive(Debug, Builder, Clone, Serialize)]
pub struct NotificationResponse {
  pub data: Notification,
}

impl TryFrom<Value> for NotificationResponse {
  type Error = anyhow::Error;

  fn try_from(_value: Value) -> Result<Self> {
    Ok(NotificationResponseBuilder::default().build()?)
  }
}