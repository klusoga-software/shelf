create table dashboards
(
    id serial constraint dashboards_fk primary key,
    user_id varchar not null unique,
    tiles jsonb not null
);