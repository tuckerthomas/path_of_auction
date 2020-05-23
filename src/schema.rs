table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    characters (id) {
        id -> Int4,
        account_id -> Int4,
        name -> Varchar,
    }
}

table! {
    items (id) {
        id -> Varchar,
        stash_id -> Nullable<Varchar>,
        verified -> Nullable<Bool>,
        w -> Nullable<Int4>,
        h -> Nullable<Int4>,
        ilvl -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
        league -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        type_line -> Nullable<Varchar>,
        identified -> Nullable<Bool>,
        extended -> Nullable<Item_extended_data>,
        frame_type -> Nullable<Frame_type>,
        x -> Nullable<Int4>,
        y -> Nullable<Int4>,
        note -> Nullable<Varchar>,
        elder -> Nullable<Bool>,
        shaper -> Nullable<Bool>,
        fractured -> Nullable<Bool>,
        dubplicated -> Nullable<Bool>,
        sockets -> Nullable<Array<Socket>>,
        support -> Nullable<Bool>,
        corrupted -> Nullable<Bool>,
        requirements -> Nullable<Array<Item_requirement>>,
        properties -> Nullable<Array<Item_requirement>>,
        additional_properties -> Nullable<Array<Item_requirement>>,
        next_level_requirements -> Nullable<Array<Item_requirement>>,
        talisman_tier -> Nullable<Int4>,
        utility_mods -> Nullable<Array<Text>>,
        implicit_mods -> Nullable<Array<Text>>,
        explicit_mods -> Nullable<Array<Text>>,
        crafted_mods -> Nullable<Array<Text>>,
        cosmetic_mods -> Nullable<Array<Text>>,
        enchant_mods -> Nullable<Array<Text>>,
        fractured_mods -> Nullable<Array<Text>>,
        flavour_text -> Nullable<Array<Text>>,
        descr_text -> Nullable<Varchar>,
        sec_descr_text -> Nullable<Varchar>,
        prophecy_diff_text -> Nullable<Varchar>,
        prophecy_text -> Nullable<Varchar>,
        inventory_id -> Nullable<Varchar>,
        is_relic -> Nullable<Bool>,
        socket -> Nullable<Int4>,
        stack_size -> Nullable<Int4>,
    }
}

table! {
    stash_lists (id) {
        id -> Int4,
        character_id -> Int4,
    }
}

table! {
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
joinable!(items -> stash_tabs (stash_id));
joinable!(stash_lists -> characters (character_id));
joinable!(stash_tabs -> stash_lists (stash_list_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    characters,
    items,
    stash_lists,
    stash_tabs,
);
