-- Add migration script here
create table main.Jobs
(
    id          INTEGER
        primary key autoincrement,
    description TEXT           not null,
    price       DECIMAL(12, 2) not null,
    paid        TINYINT(1),
    paymentDate DATETIME,
    createdAt   DATETIME       not null,
    updatedAt   DATETIME       not null,
    ContractId  INTEGER
                               references main.Contracts
                                   on update cascade on delete set null
);

