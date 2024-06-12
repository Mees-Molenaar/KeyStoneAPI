create table users -- user is a reserverd keyword in Postgres
(
    id serial primary key,
    username varchar(255) unique not null,
    password_hash varchar(255) not null
);