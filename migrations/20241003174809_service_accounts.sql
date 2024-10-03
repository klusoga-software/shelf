create table service_accounts
(
    id         serial
        constraint service_accounts_pk primary key,

    name       varchar                                       not null,
    expires_at timestamptz,
    created_at timestamptz default CURRENT_TIMESTAMP,
    updated_at timestamptz default CURRENT_TIMESTAMP,
    deleted_at timestamptz,
    secret     varchar                                       not null
)