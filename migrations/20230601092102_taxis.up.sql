create table taxis (
    id uuid primary key,
    number varchar(8) unique,
    brand varchar(50) not null,
    number_of_seats integer not null
);
