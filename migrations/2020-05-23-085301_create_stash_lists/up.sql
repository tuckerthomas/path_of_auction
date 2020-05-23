create table stash_lists (
    id serial primary key,
    character_id serial references characters(id)
)