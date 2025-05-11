# Overview

This program is a longrunning daemon which does a few things:

1. Accepts patient data into clickhouse.
    a. Accepts fhir ndjson strings from a kafka queue.
    b. Parses them into fhr r4b models
    c. Saves the models to clickhouse
    d. Does that as fast as flipping possible

2. It serves a basic ass website for statistics about how fast it's going





