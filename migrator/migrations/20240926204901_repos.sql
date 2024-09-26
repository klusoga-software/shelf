create table repos
(
    id   serial
        constraint repos_pk
            primary key,
    name varchar not null
);