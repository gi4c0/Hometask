-- Add migration script here
create table Jobs
(
    id          INTEGER
        primary key autoincrement,
    description TEXT           not null,
    price       DECIMAL(12, 2) not null,
    paid        TINYINT(1),
    paymentDate DATETIME,
    createdAt  DATETIME     not null default (datetime('now', 'local')),
    updatedAt  DATETIME     not null default (datetime('now', 'local')),
    ContractId  INTEGER
                               references Contracts
                                   on update cascade on delete set null
);

