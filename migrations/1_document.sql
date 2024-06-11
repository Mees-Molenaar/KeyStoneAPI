create table "document"
(
    id serial primary key,
    user_id int not null,
    title varchar(255) not null,
    content text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    is_synced boolean default false,
    last_synced_at timestamp
);