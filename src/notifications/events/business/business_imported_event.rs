// /**
//  *  ! Autogenerated code !
//  *  Do not make changes to this file.
//  *  Changes may be overwritten as part of auto-generation.
//  */

// import { Event } from '../../../entities/events/event';
// import { BusinessNotification } from '../../entities';
// import { type IEventsResponse } from '../../../types';
// import { EventName } from '../../helpers';
// import { type IBusinessNotificationResponse } from '../../types';

// export class BusinessImportedEvent extends Event {
//   public override readonly eventType = EventName.BusinessImported;
//   public override readonly data: BusinessNotification;

//   constructor(response: IEventsResponse<IBusinessNotificationResponse>) {
//     super(response);
//     this.data = new BusinessNotification(response.data);
//   }
// }
