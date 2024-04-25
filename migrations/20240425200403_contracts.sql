-- Add migration script here
create table main.Contracts
(
    id           INTEGER
        primary key autoincrement,
    terms        TEXT     not null,
    status       VARCHAR(20) DEFAULT 'new',
    createdAt    DATETIME not null,
    updatedAt    DATETIME not null,
    ContractorId INTEGER
                          references main.Profiles
                              on update cascade on delete set null,
    ClientId     INTEGER
                          references main.Profiles
                              on update cascade on delete set null
);

