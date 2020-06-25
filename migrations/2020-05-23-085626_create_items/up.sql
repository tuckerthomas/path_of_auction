create table items (
    id char(64) primary key,
    stash_tab_id char(64) references stash_tabs(id) not null,
    item_data jsonb
)