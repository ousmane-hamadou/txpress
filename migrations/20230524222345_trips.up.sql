create table trips (
    id uuid primary key,
    owner varchar(8) not null references taxis,
    origin uuid not null references stations,
    destination uuid not null references stations,
    constraint diff_origin_and_destination check (origin != destination),
    departure_schedule timestamp with time zone not null,
    finished boolean not null default false
);