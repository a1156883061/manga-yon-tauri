create table if not exists MangaInfo
(
    id           integer
        constraint manga_info_pk
            primary key autoincrement,
    parent_id    integer,
    title        integer,
    cover_path   TEXT,
    read_process integer default 0,
    type         text,
    sort         integer,
    create_time  integer default current_timestamp,
    update_time  integer,
    access_time  integer default current_timestamp
);

create index manga_info_parent_id_index
    on MangaInfo (parent_id);

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

