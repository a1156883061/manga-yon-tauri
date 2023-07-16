create table dir_info
(
    id          integer
        constraint dir_info_pk
            primary key autoincrement,
    name        TEXT,
    cover       TEXT,
    sort        integer,
    create_time integer default current_timestamp,
    update_time integer,
    access_time integer default current_timestamp
);

create table manga_info
(
    id           integer
        constraint manga_info_pk
            primary key autoincrement,
    parent_id    integer,
    title        integer,
    cover_path   TEXT,
    read_process integer default 0,
    type_code    text,
    sort         integer,
    create_time  integer default current_timestamp,
    update_time  integer,
    access_time  integer default current_timestamp
);

create index manga_info_parent_id_index
    on manga_info (parent_id);

create table path_list
(
    id       integer
        constraint path_list_pk
            primary key autoincrement,
    path     TEXT,
    manga_id integer
);

create index path_list_manga_id_index
    on path_list (manga_id);

create table config_dict
(
    code integer,
    val  text
);
create unique index main.config_dict_code_uindex
    on main.config_dict (code);
INSERT INTO config_dict (code, val) VALUES (1, '0.8');


