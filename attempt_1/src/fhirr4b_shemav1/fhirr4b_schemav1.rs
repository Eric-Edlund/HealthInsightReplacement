use fhir_model::r4b::resources::Patient;
use crate::schemav1::AggregatePatient;
use super::util::{double_unwrap, join_name};


pub fn convert_patient(src: &Patient) -> AggregatePatient {
    let names = double_unwrap(&src.name);
    let first = names.first();

    AggregatePatient {
        name_given: first
            .map(|first| join_name(&first.given))
            .unwrap_or("".to_string()),
        name_family: first
            .and_then(|name| name.family.clone())
            .unwrap_or("".to_string()),
    }
}

