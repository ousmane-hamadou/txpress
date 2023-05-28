-- Add up migration script here
create index idx_journey_criteria on trips using btree(origin, destination, finished);
