create table document
(
    id serial primary key,
    user_id varchar(255) not null, -- Not a Foreign Key since Amazong Cognito will be used
    title varchar(255) not null,
    content text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    is_synced boolean default false,
    last_synced_at timestamp
);