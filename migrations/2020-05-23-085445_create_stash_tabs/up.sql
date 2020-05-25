CREATE TABLE stash_tabs (
    id VARCHAR PRIMARY KEY,
    account_id serial references accounts(id) NOT NULL,
    public bool NOT NULL,
    stash VARCHAR,
    stash_type VARCHAR NOT NULL,
    league VARCHAR
);