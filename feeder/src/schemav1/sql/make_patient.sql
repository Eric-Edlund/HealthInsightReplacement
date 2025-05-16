
CREATE TABLE IF NOT EXISTS AggregatePatient (
id String,
name_given String,
name_family String,
birth_time Date32,
birth_time_resolution LowCardinality(String),
death_time Nullable(DateTime32),
deceased Enum('unknown' = 1, 'alive' = 2, 'dead' = 3),
addresses Nested(
  use Enum( 'unknown' = 0, 'billing' = 1, 'home' = 2, 'old' = 3, 'temp' = 4, 'work' = 5),
  type Enum( 'unknown' = 0, 'physical' = 1, 'postal' = 2, 'both' = 3 ),
  city String,
  line String,
  country String,
  postal_code String,
),
) ORDER BY ()
