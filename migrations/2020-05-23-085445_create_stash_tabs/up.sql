CREATE TABLE stash_tabs (
    id VARCHAR PRIMARY KEY,
    stash_list_id serial references stash_lists(id),
    public bool NOT NULL,
    stash VARCHAR,
    stash_type VARCHAR NOT NULL,
    league VARCHAR
);