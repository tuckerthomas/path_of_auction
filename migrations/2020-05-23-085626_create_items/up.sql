create type item_line_content_value as (
    name varchar, 
    val int4
);

create type item_requirement AS (
    name varchar,
    values item_line_content_value[],
    display_mode int4,
    prop_type int4,
    progress real
);

create type item_extended_data as (
    category varchar,
    subcategories text[],
    prefixes int4,
    suffixes int4,
    base_type varchar
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
    attr varchar,
    s_colour varchar
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
    art_file_name varchar,
    category varchar not null,
    corrupted bool,
    cosmetic_mods text[],
    crafted_mods text[],
    descr_text varchar,
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
    icon varchar not null,
    id varchar primary key,
    identified bool not null,
    ilvl int4 not null,
    implicit_mods text[],
    influences influence_type,
    inventory_id varchar,
    is_relic bool,
    league varchar not null,
    locked_to_character bool,
    max_stack_size int4,
    name varchar not null,
    next_level_requirements item_requirement[],
    note varchar,
    properties item_requirement[],
    prophecy_diff_text varchar,
    prophecy_text varchar,
    requirements item_requirement[],
    sec_descr_text varchar,
    shaper bool,
    sockets socket_type[],
    stack_size int4,  
    stash_tab_id varchar references stash_tabs(id) not null,
    support bool,
    talisman_tier int4,
    type_line varchar not null,
    utility_mods text[],
    verified bool not null,
    w int4 not null,
    x int4 not null,
    y int4 not null
)