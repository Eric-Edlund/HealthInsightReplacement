mod patient;
mod encounter;
pub mod db_ops;

pub use patient::*;
pub use encounter::*;


pub enum Resource {
    Patient(AggregatePatient),
    Encounter(Encounter),
}

