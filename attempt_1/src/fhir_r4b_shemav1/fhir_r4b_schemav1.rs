use super::util::{double_unwrap, join_name};
use crate::schemav1::{AggregatePatient, Deceased, TimeResolution};
use fhir_model::{
    Date,
    r4b::resources::{Patient, PatientDeceased},
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

    // let Some(first_addr) = double_unwrap(&src.address).first() else {
    //     todo!()
    // };

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
    })
}
