CREATE TABLE IF NOT EXISTS attempt_1.AggregatePatient (
	-- mpiid? mrn + facility?
	mpiid String(255),
	name_given String DEFAULT '',
	name_family String DEFAULT '',
	ssn String,

	address, -- TODO
	birth_order UInt8,
	birth_time DateTime32,
	blank_name_reason String,
	created_time DateTime32,
	-- death_location 
	death_time DateTime32,
	is_dead Bool,
	-- enteredon?
	-- updatedon?
	primary_language_code LowCardinality(String),
	primary_language_description LowCardinality(String),
	primary_language_standard LowCardinality(String),
	
	-- birth_location
	-- address
	-- citizenship_code
	-- citizenship_description
	-- citizenship_standard

	-- death declared by address, person, ...

	-- entered by

	-- race_code
	-- race_description
	-- race_standard
	-- ethnic_code
	-- ethnic_description
	-- ethnic_standard

	-- sex_code
	-- sex_description
	-- sex_standard

	-- gender_identity_code
	-- ...

	-- marital_status_code
	-- ...

	-- mother, father

	-- occupation_code
	-- occupation_description
	-- ...

	-- religion_code
	-- ...


) ORDER BY ()
