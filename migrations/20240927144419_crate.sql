create table crates(
    id serial
        constraint crates_pk primary key,
    name varchar not null,
    path varchar not null,
    version varchar not null,
    repo_id integer
        constraint configs_repos_id_fk
            references repos not null,
    index jsonb not null
);

alter table crates add constraint unique_name_version UNIQUE (name, version);
