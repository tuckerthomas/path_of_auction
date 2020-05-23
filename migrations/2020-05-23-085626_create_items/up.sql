create domain uint2 as int4
    check(value >=0 and value < 65536);

create type item_line_content_value as (
    name varchar, 
    val uint2
);

create type item_requirement AS (
    name varchar,
    values item_line_content_value[],
    display_mode uint2,
    prop_type uint2,
    progress real
);

create type item_extended_data as (
    category varchar,
    subcategories text[],
    prefixes uint2,
    suffixes uint2,
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

create type socket AS (
    s_group uint2,
    attr varchar,
    s_colour varchar
);

create table items (
    id varchar primary key,
    stash_id varchar references stash_tabs(id),
    verified bool,
    w uint2,
    h uint2,
    ilvl uint2,
    icon varchar,
    league varchar,
    name varchar,
    type_line varchar,
    identified bool,
    extended item_extended_data,
    frame_type frame_type,
    x uint2,
    y uint2,
    note varchar,
    elder bool,
    shaper bool,
    fractured bool,
    dubplicated bool,
    sockets socket[],
    support bool,
    corrupted bool,
    requirements item_requirement[],
    properties item_requirement[],
    additional_properties item_requirement[],
    next_level_requirements item_requirement[],
    talisman_tier uint2,
    utility_mods text[],
    implicit_mods text[],
    explicit_mods text[],
    crafted_mods text[],
    cosmetic_mods text[],
    enchant_mods text[],
    fractured_mods text[],
    flavour_text text[],
    descr_text varchar,
    sec_descr_text varchar,
    prophecy_diff_text varchar,
    prophecy_text varchar,
    inventory_id varchar,
    is_relic bool,
    socket uint2,
    stack_size uint2  
)