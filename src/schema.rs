table! {
    use diesel::sql_types::*;
    use use crate::public_stash_tabs::*;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
        last_character -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use use crate::public_stash_tabs::*;

    items (id) {
        abyss_jewel -> Nullable<Bool>,
        additional_properties -> Nullable<Array<ItemRequirementsType>>,
        art_file_name -> Nullable<Text>,
        category -> Text,
        corrupted -> Nullable<Bool>,
        cosmetic_mods -> Nullable<Array<Text>>,
        crafted_mods -> Nullable<Array<Text>>,
        descr_text -> Nullable<Text>,
        dubplicated -> Nullable<Bool>,
        elder -> Nullable<Bool>,
        enchant_mods -> Nullable<Array<Text>>,
        explicit_mods -> Nullable<Array<Text>>,
        extended -> ItemExtendedDataType,
        flavour_text -> Nullable<Array<Text>>,
        fractured -> Nullable<Bool>,
        fractured_mods -> Nullable<Array<Text>>,
        frame_type -> FrameTypeType,
        h -> Int4,
        icon -> Text,
        id -> Text,
        identified -> Bool,
        ilvl -> Int4,
        implicit_mods -> Nullable<Array<Text>>,
        influences -> Nullable<InfluenceType>,
        inventory_id -> Nullable<Text>,
        is_relic -> Nullable<Bool>,
        league -> Text,
        locked_to_character -> Nullable<Bool>,
        max_stack_size -> Nullable<Int4>,
        name -> Text,
        next_level_requirements -> Nullable<Array<ItemRequirementsType>>,
        note -> Nullable<Text>,
        properties -> Nullable<Array<ItemRequirementsType>>,
        prophecy_diff_text -> Nullable<Text>,
        prophecy_text -> Nullable<Text>,
        requirements -> Nullable<Array<ItemRequirementsType>>,
        sec_descr_text -> Nullable<Text>,
        shaper -> Nullable<Bool>,
        sockets -> Nullable<Array<SocketType>>,
        stack_size -> Nullable<Int4>,
        stash_tab_id -> Text,
        support -> Nullable<Bool>,
        talisman_tier -> Nullable<Int4>,
        type_line -> Text,
        utility_mods -> Nullable<Array<Text>>,
        verified -> Bool,
        w -> Int4,
        x -> Int4,
        y -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use use crate::public_stash_tabs::*;

    stash_tabs (id) {
        id -> Varchar,
        account_id -> Int4,
        public -> Bool,
        stash -> Nullable<Varchar>,
        stash_type -> Varchar,
        league -> Nullable<Varchar>,
    }
}

joinable!(items -> stash_tabs (stash_tab_id));
joinable!(stash_tabs -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    items,
    stash_tabs,
);
