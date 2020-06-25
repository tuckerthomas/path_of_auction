table! {
    use diesel::sql_types::*;
    use use crate::models::*;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
        last_character -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use use crate::models::*;

    items (id) {
        id -> Bpchar,
        stash_tab_id -> Bpchar,
        item_data -> Nullable<Jsonb>,
    }
}

table! {
    use diesel::sql_types::*;
    use use crate::models::*;

    stash_tabs (id) {
        id -> Varchar,
        account_id -> Int4,
        items -> Nullable<Array<Bpchar>>,
        stash_data -> Nullable<Jsonb>,
    }
}

joinable!(items -> stash_tabs (stash_tab_id));
joinable!(stash_tabs -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    items,
    stash_tabs,
);
