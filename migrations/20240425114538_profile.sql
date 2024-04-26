-- Add migration script here
create table Profiles
(
    id         INTEGER
        primary key autoincrement,
    firstName  VARCHAR(255) not null,
    lastName   VARCHAR(255) not null,
    profession VARCHAR(255) not null,
    balance    DECIMAL(12, 2) not null,
    type       VARCHAR(20),
    createdAt  DATETIME     not null default (datetime('now', 'local')),
    updatedAt  DATETIME     not null default (datetime('now', 'local'))
);
