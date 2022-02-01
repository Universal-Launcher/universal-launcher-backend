-- Your SQL goes here
create table modpacks(
       id integer not null primary key autoincrement,
       name varchar(100) not null unique,
       description varchar(4000),
       created_at timestamp not null default current_timestamp,
       updated_at timestamp not null default current_timestamp,
       enabled boolean not null default 'd'
);

CREATE TRIGGER modpacks_update AFTER UPDATE
ON modpacks
BEGIN
       UPDATE modpacks set updated_at = current_timestamp where id = old.id;
END;
