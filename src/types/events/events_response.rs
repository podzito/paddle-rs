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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsResponse<T> {
  pub event_id: String,
  pub notification_id: String,
  pub event_type: String,
  pub occurred_at: String,
  pub data: T
}

// interface IAddressCreated extends IEventsResponse<IAddressNotificationResponse> {
//   event_type: EventName.AddressCreated;
// }

// interface IAddressUpdated extends IEventsResponse<IAddressNotificationResponse> {
//   event_type: EventName.AddressUpdated;
// }

// interface IAddressImported extends IEventsResponse<IAddressNotificationResponse> {
//   event_type: EventName.AddressImported;
// }

// interface IAdjustmentCreated extends IEventsResponse<IAdjustmentNotificationResponse> {
//   event_type: EventName.AdjustmentCreated;
// }

// interface IAdjustmentUpdated extends IEventsResponse<IAdjustmentNotificationResponse> {
//   event_type: EventName.AdjustmentUpdated;
// }

// interface IBusinessCreated extends IEventsResponse<IBusinessNotificationResponse> {
//   event_type: EventName.BusinessCreated;
// }

// interface IBusinessUpdated extends IEventsResponse<IBusinessNotificationResponse> {
//   event_type: EventName.BusinessUpdated;
// }

// interface IBusinessImported extends IEventsResponse<IBusinessNotificationResponse> {
//   event_type: EventName.BusinessImported;
// }

// interface ICustomerCreated extends IEventsResponse<ICustomerNotificationResponse> {
//   event_type: EventName.CustomerCreated;
// }

// interface ICustomerUpdated extends IEventsResponse<ICustomerNotificationResponse> {
//   event_type: EventName.CustomerUpdated;
// }

// interface ICustomerImported extends IEventsResponse<ICustomerNotificationResponse> {
//   event_type: EventName.CustomerImported;
// }

// interface IDiscountCreated extends IEventsResponse<IDiscountNotificationResponse> {
//   event_type: EventName.DiscountCreated;
// }

// interface IDiscountUpdated extends IEventsResponse<IDiscountNotificationResponse> {
//   event_type: EventName.DiscountUpdated;
// }

// interface IDiscountImported extends IEventsResponse<IDiscountNotificationResponse> {
//   event_type: EventName.DiscountImported;
// }

// interface IPayoutCreated extends IEventsResponse<IPayoutNotificationResponse> {
//   event_type: EventName.PayoutCreated;
// }

// interface IPayoutPaid extends IEventsResponse<IPayoutNotificationResponse> {
//   event_type: EventName.PayoutPaid;
// }

// interface IPriceCreated extends IEventsResponse<IPriceNotificationResponse> {
//   event_type: EventName.PriceCreated;
// }

// interface IPriceUpdated extends IEventsResponse<IPriceNotificationResponse> {
//   event_type: EventName.PriceUpdated;
// }

// interface IPriceImported extends IEventsResponse<IPriceNotificationResponse> {
//   event_type: EventName.PriceImported;
// }

// interface IProductCreated extends IEventsResponse<IProductNotificationResponse> {
//   event_type: EventName.ProductCreated;
// }

// interface IProductUpdated extends IEventsResponse<IProductNotificationResponse> {
//   event_type: EventName.ProductUpdated;
// }

// interface IProductImported extends IEventsResponse<IProductNotificationResponse> {
//   event_type: EventName.ProductImported;
// }

// interface ISubscriptionActivated extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionActivated;
// }

// interface ISubscriptionCanceled extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionCanceled;
// }

// interface ISubscriptionCreated extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionCreated;
// }

// interface ISubscriptionImported extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionImported;
// }

// interface ISubscriptionPastDue extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionPastDue;
// }

// interface ISubscriptionPaused extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionPaused;
// }

// interface ISubscriptionResumed extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionResumed;
// }

// interface ISubscriptionTrialing extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionTrialing;
// }

// interface ISubscriptionUpdated extends IEventsResponse<ISubscriptionNotificationResponse> {
//   event_type: EventName.SubscriptionUpdated;
// }

// interface ITransactionBilled extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionBilled;
// }

// interface ITransactionCanceled extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionCanceled;
// }

// interface ITransactionCompleted extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionCompleted;
// }

// interface ITransactionCreated extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionCreated;
// }

// interface ITransactionPaid extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionPaid;
// }

// interface ITransactionPastDue extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionPastDue;
// }

// interface ITransactionPaymentFailed extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionPaymentFailed;
// }

// interface ITransactionReady extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionReady;
// }

// interface ITransactionUpdated extends IEventsResponse<ITransactionNotificationResponse> {
//   event_type: EventName.TransactionUpdated;
// }

// interface IReportCreated extends IEventsResponse<IReportNotificationResponse> {
//   event_type: EventName.ReportCreated;
// }

// interface IReportUpdated extends IEventsResponse<IReportNotificationResponse> {
//   event_type: EventName.ReportUpdated;
// }

pub enum Events {
  AddressCreated,
  AddressUpdated,
  AddressImported,
  AdjustmentCreated,
  AdjustmentUpdated,
  BusinessCreated,
  BusinessUpdated,
  BusinessImported,
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
  SubscriptionCreated,
  SubscriptionImported,
  SubscriptionPastDue,
  SubscriptionPaused,
  SubscriptionResumed,
  SubscriptionTrialing,
  SubscriptionUpdated,
  TransactionBilled,
  TransactionCanceled,
  TransactionCompleted,
  TransactionCreated,
  TransactionPaid,
  TransactionPastDue,
  TransactionPaymentFailed,
  TransactionReady,
  TransactionUpdated,
  ReportCreated,
  ReportUpdated,
}