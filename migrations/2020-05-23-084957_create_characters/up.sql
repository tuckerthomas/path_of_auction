create table characters (
    id serial primary key,
    account_id serial references accounts(id),
    name varchar not null
);