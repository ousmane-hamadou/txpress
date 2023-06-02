create table taxi_owners (
    id varchar(8) primary key references taxis(number),
    full_name varchar(80) not null,
    password text not null
);
