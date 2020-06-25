CREATE TABLE stash_tabs (
    id VARCHAR PRIMARY KEY,
    account_id serial references accounts(id) NOT NULL,
    items char(64)[],
    stash_data jsonb
);