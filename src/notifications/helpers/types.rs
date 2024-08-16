// MIT License

// Copyright (c) 2024 PodZito

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

// export type EventEntity =
//   | AddressCreatedEvent
//   | AddressUpdatedEvent
//   | AddressImportedEvent
//   | AdjustmentUpdatedEvent
//   | AdjustmentCreatedEvent
//   | BusinessCreatedEvent
//   | BusinessUpdatedEvent
//   | BusinessImportedEvent
//   | CustomerCreatedEvent
//   | CustomerUpdatedEvent
//   | CustomerImportedEvent
//   | DiscountCreatedEvent
//   | DiscountUpdatedEvent
//   | DiscountImportedEvent
//   | PayoutCreatedEvent
//   | PayoutPaidEvent
//   | PriceCreatedEvent
//   | PriceUpdatedEvent
//   | PriceImportedEvent
//   | ProductCreatedEvent
//   | ProductUpdatedEvent
//   | ProductImportedEvent
//   | SubscriptionActivatedEvent
//   | SubscriptionCanceledEvent
//   | SubscriptionCreatedEvent
//   | SubscriptionImportedEvent
//   | SubscriptionPastDueEvent
//   | SubscriptionPausedEvent
//   | SubscriptionResumedEvent
//   | SubscriptionTrialingEvent
//   | SubscriptionUpdatedEvent
//   | TransactionBilledEvent
//   | TransactionCanceledEvent
//   | TransactionCompletedEvent
//   | TransactionCreatedEvent
//   | TransactionPaidEvent
//   | TransactionPastDueEvent
//   | TransactionPaymentFailedEvent
//   | TransactionReadyEvent
//   | TransactionUpdatedEvent
//   | ReportUpdatedEvent
//   | ReportCreatedEvent;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventName {
AddressCreated,
AddressUpdated,
AddressImported,
AdjustmentCreated,
AdjustmentUpdated,
BusinessCreated,
BusinessImported,
BusinessUpdated,
CustomerCreated,
CustomerUpdated,
CustomerImported,
DiscountCreated,
DiscountUpdated,
DiscountImported,
PayoutCreated,
PayoutPaid,
PriceCreated,
PriceUpdated,
PriceImported,
ProductCreated,
ProductUpdated,
ProductImported,
SubscriptionActivated,
SubscriptionCanceled,
SubscriptionImported,
SubscriptionCreated,
SubscriptionPastDue,
SubscriptionPaused,
SubscriptionResumed,
SubscriptionTrialing,
SubscriptionUpdated,
TransactionBilled,
TransactionCanceled,
TransactionCompleted,
TransactionPaid,
TransactionCreated,
TransactionPastDue,
TransactionPaymentFailed,
TransactionReady,
TransactionUpdated,
ReportCreated,
ReportUpdated,
}

impl From<&str> for EventName {
  fn from(s: &str) -> Self {
    match s {
      "address.created" => EventName::AddressCreated,
        "address.updated" => EventName::AddressUpdated,
  "address.imported" => EventName::AddressImported,
  "adjustment.created" => EventName::AdjustmentCreated,
  "adjustment.updated" => EventName::AdjustmentUpdated,
  "business.created" => EventName::BusinessCreated,
  "business.imported" => EventName::BusinessImported,
  "business.updated" => EventName::BusinessUpdated,
  "customer.created" => EventName::CustomerCreated,
  "customer.updated" => EventName::CustomerUpdated,
  "customer.imported" => EventName::CustomerImported,
  "discount.created" => EventName::DiscountCreated,
  "discount.updated" => EventName::DiscountUpdated,
  "discount.imported" => EventName::DiscountImported,
  "payout.created" => EventName::PayoutCreated,
  "payout.paid" => EventName::PayoutPaid,
  "price.created" => EventName::PriceCreated,
  "price.updated" => EventName::PriceUpdated,
  "price.imported" => EventName::PriceImported,
  "product.created" => EventName::ProductCreated,
  "product.updated" => EventName::ProductUpdated,
  "product.imported" => EventName::ProductImported,
  "subscription.activated" => EventName::SubscriptionActivated,
  "subscription.canceled" => EventName::SubscriptionCanceled,
  "subscription.imported" => EventName::SubscriptionImported,
  "subscription.created" => EventName::SubscriptionCreated,
  "subscription.past_due" => EventName::SubscriptionPastDue,
  "subscription.paused" => EventName::SubscriptionPaused,
  "subscription.resumed" => EventName::SubscriptionResumed,
  "subscription.trialing" => EventName::SubscriptionTrialing,
  "subscription.updated" => EventName::SubscriptionUpdated,
  "transaction.billed" => EventName::TransactionBilled,
  "transaction.canceled" => EventName::TransactionCanceled,
  "transaction.completed" => EventName::TransactionCompleted,
  "transaction.paid" => EventName::TransactionPaid,
  "transaction.created" => EventName::TransactionCreated,
  "transaction.past_due" => EventName::TransactionPastDue,
  "transaction.payment_failed" => EventName::TransactionPaymentFailed,
  "transaction.ready" => EventName::TransactionReady,
  "transaction.updated" => EventName::TransactionUpdated,
  "report.created" => EventName::ReportCreated,
  "report.updated" => EventName::ReportUpdated,
      _ => panic!("Invalid event name"),
    }
  }
}
