-- Your SQL goes here
create table users (
    id serial not null,
    username varchar(25) not null,
    password varchar(1024) not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null,
    email varchar(250) not null,
    email_verified boolean not null default false,

    primary key (id),
    unique(username),
    unique(email)
);