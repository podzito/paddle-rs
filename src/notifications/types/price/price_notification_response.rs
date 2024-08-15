// /**
//  *  ! Autogenerated code !
//  *  Do not make changes to this file.
//  *  Changes may be overwritten as part of auto-generation.
//  */

// import {
//   type ITimePeriodNotification,
//   type IMoneyNotificationResponse,
//   type IUnitPriceOverrideNotificationResponse,
//   type IPriceQuantityNotification,
//   type ISharedProductNotificationResponse,
//   type IImportMetaNotificationResaponse,
// } from '../index';
// import { type TaxMode, type Status, type CatalogType } from '../../../enums';
// import { type ICustomData } from '../../../types';

// export interface IPriceNotificationResponse {
//   id: string;
//   product_id: string;
//   description: string;
//   type?: CatalogType | null;
//   name?: string | null;
//   billing_cycle?: ITimePeriodNotification | null;
//   trial_period?: ITimePeriodNotification | null;
//   tax_mode: TaxMode;
//   unit_price: IMoneyNotificationResponse;
//   unit_price_overrides: IUnitPriceOverrideNotificationResponse[] | null;
//   quantity: IPriceQuantityNotification;
//   status: Status;
//   created_at?: string | null;
//   updated_at?: string | null;
//   custom_data?: ICustomData | null;
//   import_meta?: IImportMetaNotificationResponse | null;
//   product?: ISharedProductNotificationResponse | null;
// }
