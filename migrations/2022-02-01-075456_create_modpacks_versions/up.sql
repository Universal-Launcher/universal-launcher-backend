-- Your SQL goes here
create table modpack_versions(
    id serial not null primary key,
    modpack_id integer not null,
    version varchar(20) not null unique,
    description varchar(1000),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,

    foreign key (modpack_id) references modpacks(id)
);