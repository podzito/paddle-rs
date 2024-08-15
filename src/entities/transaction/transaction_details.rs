// /**
//  *  ! Autogenerated code !
//  *  Do not make changes to this file.
//  *  Changes may be overwritten as part of auto-generation.
//  */

// import { type ITransactionDetailsResponse } from '../../types';
// import {
//   TaxRatesUsed,
//   TransactionTotals,
//   TransactionTotalsAdjusted,
//   TransactionPayoutTotals,
//   TransactionPayoutTotalsAdjusted,
//   TransactionLineItem,
// } from '../index';

// export class TransactionDetails {
//   public readonly taxRatesUsed: TaxRatesUsed[];
//   public readonly totals: TransactionTotals | null;
//   public readonly adjustedTotals: TransactionTotalsAdjusted | null;
//   public readonly payoutTotals: TransactionPayoutTotals | null;
//   public readonly adjustedPayoutTotals: TransactionPayoutTotalsAdjusted | null;
//   public readonly lineItems: TransactionLineItem[];

//   constructor(transactionDetails: ITransactionDetailsResponse) {
//     this.taxRatesUsed = transactionDetails.tax_rates_used.map((tax_rates_used) => new TaxRatesUsed(tax_rates_used));
//     this.totals = transactionDetails.totals ? new TransactionTotals(transactionDetails.totals) : null;
//     this.adjustedTotals = transactionDetails.adjusted_totals
//       ? new TransactionTotalsAdjusted(transactionDetails.adjusted_totals)
//       : null;
//     this.payoutTotals = transactionDetails.payout_totals
//       ? new TransactionPayoutTotals(transactionDetails.payout_totals)
//       : null;
//     this.adjustedPayoutTotals = transactionDetails.adjusted_payout_totals
//       ? new TransactionPayoutTotalsAdjusted(transactionDetails.adjusted_payout_totals)
//       : null;
//     this.lineItems = transactionDetails.line_items.map((line_item) => new TransactionLineItem(line_item));
//   }
// }
