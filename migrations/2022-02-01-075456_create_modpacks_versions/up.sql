-- Your SQL goes here
create table modpack_versions(
    id integer not null primary key autoincrement,
    modpack_id integer not null,
    version varchar(20) not null unique,
    description varchar(1000),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,

    foreign key (modpack_id) references modpacks(id)
);

create trigger modpack_versions_update after update
on modpack_versions
BEGIN
    update modpack_versions set updated_at = current_timestamp where id = old.id;
END;