-- Add migration script here
create table Contracts
(
    id           INTEGER
        primary key autoincrement,
    terms        TEXT     not null,
    status       VARCHAR(20) DEFAULT 'new',
    createdAt  DATETIME     not null default (datetime('now', 'local')),
    updatedAt  DATETIME     not null default (datetime('now', 'local')),
    ContractorId INTEGER
                          references Profiles
                              on update cascade on delete set null,
    ClientId     INTEGER
                          references Profiles
                              on update cascade on delete set null
);

