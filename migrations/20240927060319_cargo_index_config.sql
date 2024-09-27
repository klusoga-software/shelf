create table configs
(
    id      serial
        constraint configs_pk
            primary key,
    repo_id integer
        constraint configs_repos_id_fk
            references repos,
    dl varchar,
    api varchar
);