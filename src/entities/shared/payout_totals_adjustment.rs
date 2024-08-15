// /**
//  *  ! Autogenerated code !
//  *  Do not make changes to this file.
//  *  Changes may be overwritten as part of auto-generation.
//  */

// import { type IPayoutTotalsAdjustmentResponse } from '../../types';
// import { ChargebackFee } from '../index';
// import { type PayoutCurrencyCode } from '../../enums';

// export class PayoutTotalsAdjustment {
//   public readonly subtotal: string;
//   public readonly tax: string;
//   public readonly total: string;
//   public readonly fee: string;
//   public readonly chargebackFee: ChargebackFee | null;
//   public readonly earnings: string;
//   public readonly currencyCode: PayoutCurrencyCode;

//   constructor(payoutTotalsAdjustment: IPayoutTotalsAdjustmentResponse) {
//     this.subtotal = payoutTotalsAdjustment.subtotal;
//     this.tax = payoutTotalsAdjustment.tax;
//     this.total = payoutTotalsAdjustment.total;
//     this.fee = payoutTotalsAdjustment.fee;
//     this.chargebackFee = payoutTotalsAdjustment.chargeback_fee
//       ? new ChargebackFee(payoutTotalsAdjustment.chargeback_fee)
//       : null;
//     this.earnings = payoutTotalsAdjustment.earnings;
//     this.currencyCode = payoutTotalsAdjustment.currency_code;
//   }
// }
