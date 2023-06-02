create table trips (
    id uuid primary key,
    owner varchar(8) not null references taxis(number),
    origin uuid not null references taxi_ranks,
    destination uuid not null references taxi_ranks,
    constraint diff_origin_and_destination check (origin != destination),
    departure_schedule timestamp with time zone not null,
    closed boolean not null default false,
    reserved_seats integer not null default 0
);

create index idx_trips_owner on trips using hash(owner);
create index idx_journey_criteria on trips using btree(origin, destination, closed);
