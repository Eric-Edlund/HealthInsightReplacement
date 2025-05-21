mod patient;
mod encounter;
pub mod db_ops;

pub use patient::*;
pub use encounter::*;


#[derive(Debug)]
pub enum Resource {
    Patient(AggregatePatient),
    Encounter(Encounter),
}

