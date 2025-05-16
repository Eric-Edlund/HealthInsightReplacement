use clickhouse::Row;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EncounterStatus {
    Arrived = 0,
    Cancelled = 1,
    EnteredInError = 2,
    Finished = 3,
    InProgress = 4,
    Onleave = 5,
    Planned = 6,
    Triaged = 7,
    Unknown = 8,
}


#[derive(Debug, Row, Serialize)]
pub struct Encounter {
    pub id: String,
    pub status: EncounterStatus,
    /// Patient id
    pub subject: String,

    #[serde(with = "clickhouse::serde::time::datetime")]
    pub period_start: time::OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime")]
    pub period_end: time::OffsetDateTime,

    pub class_code: String,
    pub class_description: String,
    pub class_system: String,

    // pub type_code: String,
    // pub type_description: String,
    // pub type_system: String,

}
//   "type": [
//     {
//       "coding": [
//         {
//           "system": "http://snomed.info/sct",
//           "code": "185347001",
//           "display": "Encounter for problem (procedure)"
//         }
//       ],
//       "text": "Encounter for problem (procedure)"
//     }
//   ],
//   "subject": {
//     "reference": "Patient/7d9aa431-cd72-8aa2-9559-5920937d9330",
//     "display": "Mr. Dwight645 Jamal145 Weimann465"
//   },
//   "participant": [
//     {
//       "type": [
//         {
//           "coding": [
//             {
//               "system": "http://terminology.hl7.org/CodeSystem/v3-ParticipationType",
//               "code": "PPRF",
//               "display": "primary performer"
//             }
//           ],
//           "text": "primary performer"
//         }
//       ],
//       "period": {
//         "start": "1998-04-16T15:59:37-04:00",
//         "end": "1998-04-16T19:44:37-04:00"
//       },
//       "individual": {
//         "reference": "Practitioner?identifier=http://hl7.org/fhir/sid/us-npi|9999996298",
//         "display": "Dr. Sallie654 Jast432"
//       }
//     }
//   ],
//   "period": {
//     "start": "1998-04-16T15:59:37-04:00",
//     "end": "1998-04-16T19:44:37-04:00"
//   },
//   "location": [
//     {
//       "location": {
//         "reference": "Location?identifier=https://github.com/synthetichealth/synthea|4483aff2-b48c-39f6-92ba-cc6c1cac502b",
//         "display": "OLATHE MEDICAL CENTER"
//       }
//     }
//   ],
//   "serviceProvider": {
//     "reference": "Organization?identifier=https://github.com/synthetichealth/synthea|55f984c8-3d2f-3b1f-9a24-d693ca144a28",
//     "display": "OLATHE MEDICAL CENTER"
//   }
// }
