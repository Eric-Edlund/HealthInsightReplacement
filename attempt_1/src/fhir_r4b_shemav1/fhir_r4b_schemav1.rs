use super::util::{double_unwrap, join_name};
use crate::schemav1::{AggregatePatient, Deceased, TimeResolution};
use crate::schemav1;
use fhir_model::{
    r4b::{codes::AddressUse, resources::{Patient, PatientDeceased}}, Date
};
use time::{OffsetDateTime, Time, macros::date};

#[derive(Debug)]
pub struct ConversionError {}

pub type ConversionResult<T> = Result<T, ConversionError>;

const DD: time::Date = date!(2025 - 01 - 01);

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
            PatientDeceased::DateTime(death_time) => match death_time {
                fhir_model::DateTime::Date(date) => match &date {
                    Date::Year(year) => {
                        let Ok(d) = DD.replace_year(*year) else {
                            return Err(ConversionError {});
                        };
                        (Deceased::Dead, Some(d.with_time(Time::MIDNIGHT).assume_utc()))
                    }
                    Date::YearMonth(year, month) => {
                        let Ok(d) = DD.replace_year(*year) else {
                            return Err(ConversionError {});
                        };
                        let Ok(d) = d.replace_month(*month) else {
                            return Err(ConversionError {});
                        };
                        (Deceased::Dead, Some(d.with_time(Time::MIDNIGHT).assume_utc()))
                    }
                    Date::Date(date) => (
                        Deceased::Dead,
                        Some(date.with_time(Time::MIDNIGHT).assume_utc()),
                    ),
                },
                fhir_model::DateTime::DateTime(instant) => {
                    let fhir_model::Instant(offsetdatetime) = instant;
                    (Deceased::Dead, Some(*offsetdatetime))
                }
            },
        },
        None => (Deceased::Unknown, None),
    };

    let (mut uses,): (Vec<schemav1::AddressUse>,) = (vec![],);
    for addr in double_unwrap(&src.address) {
        uses.push(match addr.r#use {
            None => schemav1::AddressUse::Unknown,
            Some(addr_use) => match addr_use {
                AddressUse::Old => schemav1::AddressUse::Old,
                AddressUse::Home => schemav1::AddressUse::Home,
                AddressUse::Temp => schemav1::AddressUse::Temp,
                AddressUse::Work => schemav1::AddressUse::Work,
                AddressUse::Billing => schemav1::AddressUse::Billing,
            }
        });
    }

    Ok(AggregatePatient {
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
    })
}
