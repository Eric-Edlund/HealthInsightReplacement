use super::util::{double_unwrap, join_name};
use crate::schemav1;
use crate::schemav1::{AggregatePatient, Deceased, TimeResolution};
use fhir_model::DateTime;
use fhir_model::r4b::codes::{AddressType, EncounterStatus};
use fhir_model::r4b::resources::{Bundle, Resource};
use fhir_model::r4b::types::Reference;
use fhir_model::{
    Date,
    r4b::{
        codes::AddressUse,
        resources::{Encounter, Patient, PatientDeceased},
    },
};
use time::{OffsetDateTime, Time, macros::date};

#[derive(Debug)]
pub struct ConversionError {}

pub type ConversionResult<T> = Result<T, ConversionError>;

const DD: time::Date = date!(2025 - 01 - 01);

#[derive(Debug)]
pub struct ConvertedBundle {
    pub resources: Vec<schemav1::Resource>,
}

#[allow(dead_code)]
pub fn convert_bundle(src: &Bundle) -> ConversionResult<ConvertedBundle> {
    let mut result = vec![];

    for entry in double_unwrap(&src.entry) {
        if let Some(resource) = &entry.resource {
            result.push(match resource {
                Resource::Patient(res) => schemav1::Resource::Patient(convert_patient(res)?),
                Resource::Encounter(res) => schemav1::Resource::Encounter(convert_encounter(res)?),
                _ => continue,
            })
        }
    }
    Ok(ConvertedBundle { resources: result })
}

#[allow(dead_code)]
pub fn convert_encounter(src: &Encounter) -> ConversionResult<schemav1::Encounter> {
    let Some(encounter_id) = src.id.clone() else {
        return Err(ConversionError {});
    };

    let status = match src.status {
        EncounterStatus::Arrived => schemav1::EncounterStatus::Arrived,
        EncounterStatus::Cancelled => schemav1::EncounterStatus::Cancelled,
        EncounterStatus::EnteredInError => schemav1::EncounterStatus::EnteredInError,
        EncounterStatus::Finished => schemav1::EncounterStatus::Finished,
        EncounterStatus::InProgress => schemav1::EncounterStatus::InProgress,
        EncounterStatus::Onleave => schemav1::EncounterStatus::Onleave,
        EncounterStatus::Planned => schemav1::EncounterStatus::Planned,
        EncounterStatus::Triaged => schemav1::EncounterStatus::Triaged,
        EncounterStatus::Unknown => schemav1::EncounterStatus::Unknown,
    };

    let subject_id = if let Some(rref) = &src.subject {
        Some(parse_patient_reference(rref)?)
    } else {
        None
    };

    let (start, end): (Option<time::OffsetDateTime>, Option<time::OffsetDateTime>) =
        if let Some(period) = &src.period {
            (
                match &period.start {
                    Some(t) => Some(parse_datetime(t)?),
                    None => None,
                },
                match &period.end {
                    Some(t) => Some(parse_datetime(t)?),
                    None => None,
                },
            )
        } else {
            (None, None)
        };

    Ok(schemav1::Encounter {
        id: encounter_id,
        status,
        subject: subject_id.unwrap_or_default(),
        // TODO: Should we be using UNIX_EPOCH or some other special value?
        period_start: start.unwrap_or(OffsetDateTime::UNIX_EPOCH),
        period_end: end.unwrap_or(OffsetDateTime::UNIX_EPOCH),
        class_code: src.class.code.clone().unwrap_or_default(),
        class_description: src.class.display.clone().unwrap_or_default(),
        class_system: src.class.system.clone().unwrap_or_default(),
    })
}

fn parse_datetime(src: &DateTime) -> ConversionResult<OffsetDateTime> {
    match src {
        fhir_model::DateTime::Date(date) => match &date {
            Date::Year(year) => {
                let Ok(d) = DD.replace_year(*year) else {
                    return Err(ConversionError {});
                };
                Ok(d.with_time(Time::MIDNIGHT).assume_utc())
            }
            Date::YearMonth(year, month) => {
                let Ok(d) = DD.replace_year(*year) else {
                    return Err(ConversionError {});
                };
                let Ok(d) = d.replace_month(*month) else {
                    return Err(ConversionError {});
                };
                Ok(d.with_time(Time::MIDNIGHT).assume_utc())
            }
            Date::Date(date) => Ok(date.with_time(Time::MIDNIGHT).assume_utc()),
        },
        fhir_model::DateTime::DateTime(instant) => {
            let fhir_model::Instant(offsetdatetime) = instant;
            Ok(*offsetdatetime)
        }
    }
}

fn parse_patient_reference(reff: &Reference) -> ConversionResult<String> {
    if let Some(ty) = &reff.r#type {
        match ty.as_str() {
            "Patient" => {}
            _ => return Err(ConversionError {}),
        }
    };
    if let Some(ref_str) = &reff.reference {
        // TODO: Here we assume it's a relative url
        let parts: Vec<&str> = ref_str.split("/").collect();
        if parts.len() < 2 {
            return Err(ConversionError {});
        }
        if parts[0] != "Patient" {
            return Err(ConversionError {});
        }
        return Ok(parts[1].to_string());
    }

    Err(ConversionError {})
}

#[allow(dead_code)]
pub fn convert_patient(src: &Patient) -> ConversionResult<AggregatePatient> {
    let names = double_unwrap(&src.name);
    let first = names.first();

    let (birth_time, birth_time_resolution): (Option<time::Date>, Option<TimeResolution>) =
        match &src.birth_date {
            Some(birth_date) => match birth_date {
                Date::Year(year) => {
                    let Ok(d) = DD.replace_year(*year) else {
                        return Err(ConversionError {});
                    };
                    (Some(d), Some(TimeResolution::Year))
                }
                Date::YearMonth(year, month) => {
                    let Ok(d) = DD.replace_year(*year) else {
                        return Err(ConversionError {});
                    };
                    let Ok(d) = d.replace_month(*month) else {
                        return Err(ConversionError {});
                    };
                    (Some(d), Some(TimeResolution::Month))
                }
                Date::Date(date) => (Some(*date), Some(TimeResolution::Day)),
            },
            None => (None, None),
        };

    let (deceased, death_time): (Deceased, Option<OffsetDateTime>) = match &src.deceased {
        Some(deceased) => match deceased {
            PatientDeceased::Boolean(died) => (
                if *died {
                    Deceased::Dead
                } else {
                    Deceased::Alive
                },
                None,
            ),
            PatientDeceased::DateTime(death_time) => {
                (Deceased::Dead, Some(parse_datetime(death_time)?))
            }
        },
        None => (Deceased::Unknown, None),
    };

    let (uses, types, cities, lines, states, countries): AddressList =
        parse_addresses(double_unwrap(&src.address))?;

    let Some(patient_id) = &src.id else {
        return Err(ConversionError {});
    };

    Ok(AggregatePatient {
        id: patient_id.clone(),
        name_given: first
            .map(|first| join_name(&first.given))
            .unwrap_or("".to_string()),
        name_family: first
            .and_then(|name| name.family.clone())
            .unwrap_or("".to_string()),
        birth_time: birth_time.unwrap(),
        birth_time_resolution: birth_time_resolution.unwrap(),
        death_time,
        deceased,
        addresses_use: uses,
        addresses_type: types,
        addresses_city: cities,
        addresses_line: lines,
    })
}

type AddressList = (
    Vec<schemav1::AddressUse>,
    Vec<schemav1::AddressType>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
);

fn parse_addresses(
    addresses: Vec<fhir_model::r4b::types::Address>,
) -> ConversionResult<AddressList> {
    let (mut uses, mut types, mut cities, mut lines, mut states, mut countries): AddressList =
        (vec![], vec![], vec![], vec![], vec![], vec![]);
    for addr in addresses {
        uses.push(match addr.r#use {
            None => schemav1::AddressUse::Unknown,
            Some(addr_use) => match addr_use {
                AddressUse::Old => schemav1::AddressUse::Old,
                AddressUse::Home => schemav1::AddressUse::Home,
                AddressUse::Temp => schemav1::AddressUse::Temp,
                AddressUse::Work => schemav1::AddressUse::Work,
                AddressUse::Billing => schemav1::AddressUse::Billing,
            },
        });
        types.push(match addr.r#type {
            None => schemav1::AddressType::Unknown,
            Some(addr_type) => match addr_type {
                AddressType::Physical => schemav1::AddressType::Physical,
                AddressType::Postal => schemav1::AddressType::Postal,
                AddressType::Both => schemav1::AddressType::Both,
            },
        });
        cities.push(match addr.city {
            None => "".to_string(),
            Some(ref city) => city.clone(),
        });
        lines.push(join_name(&addr.line));
        // TODO: Handle address text
        states.push(addr.state.clone().unwrap_or("".to_string()));
        countries.push(addr.country.clone().unwrap_or("".to_string()));
        // postal_codes.push(addr.postal_code.clone().unwrap_or("".to_string()));
        // periods.push
        // addr.period
    }
    assert_eq!(uses.len(), types.len());
    assert_eq!(uses.len(), cities.len());
    assert_eq!(uses.len(), lines.len());
    assert_eq!(uses.len(), states.len());
    assert_eq!(uses.len(), countries.len());

    Ok((uses, types, cities, lines, states, countries))
}
