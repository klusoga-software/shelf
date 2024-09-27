create table repos
(
    id        serial
        constraint repos_pk primary key,
    name      varchar not null unique,
    repo_type integer not null,
    public    boolean not null
);