create table bookings (
    id uuid primary key,
    journey_id uuid not null references trips,
    reserved_seats integer not null
)
