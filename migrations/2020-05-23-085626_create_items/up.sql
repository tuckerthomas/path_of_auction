create type item_line_content_value as (
    name text, 
    val int4
);

create type item_requirement AS (
    name text,
    values item_line_content_value[],
    display_mode int4,
    prop_type int4,
    progress real
);

create type item_extended_data as (
    category text,
    subcategories text[],
    prefixes int4,
    suffixes int4,
    base_type text
);

create type frame_type as enum (
    'Normal',
    'Magic',
    'Rare',
    'Unique',
    'Gem',
    'Currency',
    'DivinationCard',
    'QuestItem',
    'Prophecy',
    'Relic'
);

create type socket_type as (
    s_group int4,
    attr text,
    s_colour text
);

create type influence_type as (
    shaper bool,
    elder bool,
    crusader bool,
    redeemer bool,
    hunter bool,
    warlord bool
);

create table items (
    abyss_jewel bool,
    additional_properties item_requirement[],
    art_file_name text,
    category text not null,
    corrupted bool,
    cosmetic_mods text[],
    crafted_mods text[],
    descr_text text,
    dubplicated bool,
    elder bool,
    enchant_mods text[],
    explicit_mods text[],
    extended item_extended_data not null,
    flavour_text text[],
    fractured bool,
    fractured_mods text[],
    frame_type frame_type not null,
    h int4 not null,
    icon text not null,
    id char(64) primary key,
    identified bool not null,
    ilvl int4 not null,
    implicit_mods text[],
    influences influence_type,
    inventory_id text,
    is_relic bool,
    league text not null,
    locked_to_character bool,
    max_stack_size int4,
    name text not null,
    next_level_requirements item_requirement[],
    note text,
    properties item_requirement[],
    prophecy_diff_text text,
    prophecy_text text,
    requirements item_requirement[],
    sec_descr_text text,
    shaper bool,
    sockets socket_type[],
    stack_size int4,  
    stash_tab_id text references stash_tabs(id) not null,
    support bool,
    talisman_tier int4,
    type_line text not null,
    utility_mods text[],
    verified bool not null,
    w int4 not null,
    x int4 not null,
    y int4 not null
)