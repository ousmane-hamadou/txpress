create table taxi_owners (
    id varchar(8) not null references taxis,
    full_name varchar(80) not null,
    password text not null
);