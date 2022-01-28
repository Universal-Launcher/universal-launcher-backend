-- Your SQL goes here
create table modpacks(
       id integer not null primary key autoincrement,
       name varchar(100) not null,
       description varchar(4000),
       created_at timestamp not null default current_timestamp,
       updated_at timestamp not null default current_timestamp,
       enabled boolean not null default 'd'
);
