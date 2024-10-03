create table roles
(
    id          serial
        constraint roles_pk primary key,
    name        varchar    not null,
    permissions varchar(3) not null
);

insert into roles (name, permissions)
values ('read', 'R');

insert into roles (name, permissions)
values ('write', 'RW');

insert into roles (name, permissions)
values ('write and delete', 'RWD');