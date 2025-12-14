-- ordering of table columns to optimize alignment:
-- sizes are in bytes
--
-- uuid (including fks) - 16
-- bigint, bigserial, timestamptz (including created_at) 8
-- date, enum, int, serial 4
-- smallint, smallserial 2
-- booleans 1
-- text (variable size)
-- constraints (any placement, as these are stored in their own table)
--
-- The text representation of enums is stored separately, and the are stored in the table as ints, hence 4 byte size.

-- So in practice our groupings will be:
-- id + fks
-- timestamps like created/updated at
-- enums + int wrappers
-- bools
-- texts

-- TODO: equivalent of global cur_surgeon - this may require using an identity from BetterAuth.rs
-- TODO: RLS on surgeon_cas, see https://www.inferable.ai/blog/posts/understanding-postgres-row-security

create extension if not exists citext;

-- domain constraints
create domain acd as integer check (value >= 0 and value <= 600); -- dm
create domain al as integer check (value >= 1200 and value <= 3800); -- dm 
create domain axis as integer check (value >= 0 and value <= 179); -- °
create domain cct as integer check (value >= 350 and value <= 650); -- µm


create domain email as citext constraint validate_email check (
    value
    ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$'
);

-- D * 100
create domain iol_se as integer check (value >= -2000 and value <= 6000 and value % 25 = 0);
create domain k_power as integer check (value >= 3000 and value <= 6500); -- D * 100
create domain lt as integer check (value >= 200 and value <= 800); -- dm
create domain main as integer check (value >= 100 and value <= 600); -- dm
-- D * 100
create domain ref_cyl_power as integer check (value >= -1000 and value <= 1000 and value % 25 = 0);
-- D * 100
create domain ref_sph as integer check (value >= -2000 and value <= 2000 and value % 25 = 0);
create domain sia_power as integer check (value >= 0 and value <= 200); -- D * 100
create domain target_cyl_power as integer check (value >= 0 and value <= 600); -- D * 100
create domain target_se as integer check (value >= -600 and value <= 200); -- D * 100
-- D * 100
create domain toric_power as integer check (value >= 100 and value <= 2000 and value % 25 = 0);
create domain va_den as integer check (value >= 1); -- * 100
create domain va_num as integer check (value >= 0 and value <= 2000); -- * 100
create domain wtw as integer check (value >= 800 and value <= 1400); -- dm
create domain year as integer check (value >= 2000 and value <= 2100);

-- enums
create type adverse as enum ('Rhexis', 'Pc', 'Zonule', 'Other');
create type focus as enum ('Mono', 'Edof', 'Multi');

create type formula as enum (
    'AscrsKrs',
    'Barrett',
    'BarrettTrueK',
    'Evo',
    'Haigis',
    'HaigisL',
    'HillRbf',
    'HofferQ',
    'Holladay1',
    'Holladay2',
    'Kane',
    'Okulix',
    'Olsen',
    'SrkT',
    'Other'
);

create type side as enum ('Right', 'Left');

-- tables
create table iol (
    id uuid primary key default gen_random_uuid(),

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    focus focus not null default 'Mono',

    toric toric_power,

    model text unique not null,
    name text,
    company text
);

create table cas (
    id uuid primary key default gen_random_uuid(),

    iol uuid,
    foreign key (iol) references iol (id) on delete set null,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    side side not null,
    target_formula formula,
    adverse adverse,

    al al not null,
    flat_k_power k_power not null,
    flat_k_axis axis not null,
    steep_k_power k_power not null,
    steep_k_axis axis not null,
    acd acd not null,
    lt lt not null,
    cct cct,
    wtw wtw,

    iol_se iol_se,
    iol_axis axis,

    target_se target_se not null,
    target_cyl_power target_cyl_power,
    target_cyl_axis axis,

    constraint target_cyl_field_agreement check (
        (target_cyl_power is not null and target_cyl_axis is not null)
        or (target_cyl_power is null and target_cyl_axis is null)
    ),

    year year not null,
    main main,
    sia_power sia_power,
    sia_axis axis,

    constraint sia_cyl_field_agreement check (
        (sia_power is not null and sia_axis is not null)
        or (sia_power is null and sia_axis is null)
    ),

    va_before_best_num va_num not null,
    va_before_best_den va_den not null,
    va_before_raw_num va_num,
    va_before_raw_den va_den,
    va_after_best_num va_num,
    va_after_best_den va_den,
    va_after_raw_num va_num not null,
    va_after_raw_den va_den not null,

    ref_before_sph ref_sph not null,
    ref_before_cyl_power ref_cyl_power,
    ref_before_cyl_axis axis,

    constraint ref_before_cyl_field_agreement check (
        (ref_before_cyl_power is not null and ref_before_cyl_axis is not null)
        or (ref_before_cyl_power is null and ref_before_cyl_axis is null)
    ),

    ref_after_sph ref_sph not null,
    ref_after_cyl_power ref_cyl_power,
    ref_after_cyl_axis axis,

    constraint ref_after_cyl_field_agreement check (
        (ref_after_cyl_power is not null and ref_after_cyl_axis is not null)
        or (ref_after_cyl_power is null and ref_after_cyl_axis is null)
    ),

    target_custom_constant boolean not null default false
);

create table site (
    id uuid primary key default gen_random_uuid(),

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    name text unique not null
);

-- TODO: BetterAuth identity of some sort
create table surgeon (
    id uuid primary key default gen_random_uuid(),

    default_site uuid,
    foreign key (default_site) references site (id) on delete set null,

    default_iol uuid,
    foreign key (default_iol) references iol (id) on delete set null,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    terms timestamptz,

    default_formula formula,

    default_main main,
    default_sia_power sia_power,
    default_sia_axis axis,

    constraint default_sia_field_agreement check (
        (default_sia_power is not null and default_sia_axis is not null)
        or (default_sia_power is null and default_sia_axis is null)
    ),

    default_custom_constant boolean not null default false,

    email email unique not null,
    full_name text,
    preferred_name text

);

create table surgeon_cas (
    id uuid primary key default gen_random_uuid(),

    surgeon uuid not null,
    -- delete the surgeon_cas if its surgeon is deleted
    foreign key (surgeon) references surgeon (id) on delete cascade,

    site uuid,
    foreign key (site) references site (id) on delete set null,

    cas uuid unique not null,
    -- delete the surgeon_cas if its cas is deleted
    foreign key (cas) references cas (id) on delete cascade,

    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),

    number serial unique,
    date date not null

);

create or replace function updated_at() returns trigger language 'plpgsql' as $$
    begin new.updated_at = now(); return new; end;
$$;

create trigger cas_updated_at
before update on cas for each row execute function updated_at();

create trigger iol_updated_at
before update on iol for each row execute function updated_at();

create trigger site_updated_at
before update on site for each row execute function updated_at();

create trigger surgeon_updated_at
before update on surgeon for each row execute function updated_at();

create trigger surgeon_cas_updated_at
before update on surgeon_cas for each row execute function updated_at();

-- inserting bulk data
-- Method 1: Using INSERT with Multiple Rows
--
-- INSERT INTO table_name (column1, column2,...) VALUES
-- (value1a, value1b,...),
-- (value2a, value2b,...),
-- ...
-- (valueNa, valueNb,...) ;
-- This method allows you to insert multiple rows with a single INSERT statement.This is ideal for small to medium amounts of data,
-- where writing individual INSERT statements for each record would be impractical.
--
-- Method 2: Using the COPY Command
--
-- COPY table_name (column1, column2,...) FROM '/path/to/data.csv' DELIMITER ',' CSV ;
-- The COPY command is one of the fastest ways to perform bulk inserts in PostgreSQL.The data needs to be in a format suitable for COPY,
-- typically a CSV file.Proper permissions for the file and directory are required for the PostgreSQL server to read the file.
