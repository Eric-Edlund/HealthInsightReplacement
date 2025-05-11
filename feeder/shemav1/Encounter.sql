CREATE TABLE IF NOT EXISTS Encounter (

-- foreign key, zero means null
attending_clinicians UInt32,
consulting_clinicians UInt32,

source_facility LowCardinality(String),
mrn String,

-- Emergency, Outpatient, Inpatient
class LowCardinaty(String),
-- Encounter.status https://www.hl7.org/fhir/R4/encounter-definitions.html
-- planned | arrived | triaged | in-progress | onleave | finished | cancelled +
status LowCardinality(String),
status_history UInt32

-- e-mail consultation, surgical day-care, skilled nursing, rehabilitation
_type LowCardinality(String),
service_type_code LowCardinality(String),
service_type_display String,
service_type_system LowCardinality(String),
service_type_text String,
priority


	


) ORDER BY ()
