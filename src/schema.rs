table! {
    use diesel::sql_types::*;
    use crate::models::*;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    characters (id) {
        id -> Int4,
        account_id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    items (id) {
        abyss_jewel -> Nullable<Bool>,
        additional_properties -> Nullable<Array<ItemRequirementsType>>,
        art_file_name -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
        corrupted -> Nullable<Bool>,
        cosmetic_mods -> Nullable<Array<Text>>,
        crafted_mods -> Nullable<Array<Text>>,
        descr_text -> Nullable<Varchar>,
        dubplicated -> Nullable<Bool>,
        elder -> Nullable<Bool>,
        enchant_mods -> Nullable<Array<Text>>,
        explicit_mods -> Nullable<Array<Text>>,
        extended -> Nullable<ItemExtendedDataType>,
        flavour_text -> Nullable<Array<Text>>,
        fractured -> Nullable<Bool>,
        fractured_mods -> Nullable<Array<Text>>,
        frame_type -> Nullable<FrameTypeType>,
        h -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
        id -> Varchar,
        identified -> Nullable<Bool>,
        ilvl -> Nullable<Int4>,
        implicit_mods -> Nullable<Array<Text>>,
        influences -> Nullable<InfluenceType>,
        inventory_id -> Nullable<Varchar>,
        is_relic -> Nullable<Bool>,
        league -> Nullable<Varchar>,
        locked_to_character -> Nullable<Bool>,
        max_stack_size -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        next_level_requirements -> Nullable<Array<ItemRequirementsType>>,
        note -> Nullable<Varchar>,
        properties -> Nullable<Array<ItemRequirementsType>>,
        prophecy_diff_text -> Nullable<Varchar>,
        prophecy_text -> Nullable<Varchar>,
        requirements -> Nullable<Array<ItemRequirementsType>>,
        sec_descr_text -> Nullable<Varchar>,
        shaper -> Nullable<Bool>,
        sockets -> Nullable<Array<SocketType>>,
        stack_size -> Nullable<Int4>,
        stash_tab_id -> Nullable<Varchar>,
        support -> Nullable<Bool>,
        talisman_tier -> Nullable<Int4>,
        type_line -> Nullable<Varchar>,
        utility_mods -> Nullable<Array<Text>>,
        verified -> Nullable<Bool>,
        w -> Nullable<Int4>,
        x -> Nullable<Int4>,
        y -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    stash_lists (id) {
        id -> Int4,
        character_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    stash_tabs (id) {
        id -> Varchar,
        stash_list_id -> Int4,
        public -> Bool,
        stash -> Nullable<Varchar>,
        stash_type -> Varchar,
        league -> Nullable<Varchar>,
    }
}

joinable!(characters -> accounts (account_id));
joinable!(items -> stash_tabs (stash_tab_id));
joinable!(stash_lists -> characters (character_id));
joinable!(stash_tabs -> stash_lists (stash_list_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    characters,
    items,
    stash_lists,
    stash_tabs,
);
