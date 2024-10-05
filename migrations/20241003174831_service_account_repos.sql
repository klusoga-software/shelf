create table service_accounts_repos
(
    repo_id            int
        constraint service_accounts_repos_repos_id_fk
            references repos,
    service_account_id int
        constraint service_accounts_repos_service_accounts_id_fk
            references service_accounts,
    constraint service_accounts_repos_pk
        primary key (repo_id, service_account_id),
    role_id            integer
        constraint service_accounts_role_id references roles not null
);