-- Your SQL goes here
create table modpacks(
       id serial not null primary key,
       name varchar(100) not null unique,
       description varchar(4000),
       created_at timestamp not null default current_timestamp,
       updated_at timestamp not null default current_timestamp,
       enabled boolean not null default false
);
