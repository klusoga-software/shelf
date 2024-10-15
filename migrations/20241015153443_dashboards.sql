create table dashboards
(
    id serial constraint dashboards_fk primary key,
    user_id varchar not null,
    tiles jsonb not null
);