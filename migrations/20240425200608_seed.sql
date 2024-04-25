-- Add migration script here
INSERT INTO main.Profiles (
    id,
    firstName,
    lastName,
    profession,
    balance,
    type
) VALUES
(1, 'Harry', 'Potter', 'Wizard', 1150, 'client'),

(2, 'Mr', 'Robot', 'Hacker', 231.11, 'client'),

(3, 'John', 'Snow', 'Knows nothing', 451.3, 'client'),

(4, 'Ash', 'Kethcum', 'Pokemon master', 1.3, 'client'),

(5, 'John', 'Lenon', 'Musician', 64, 'contractor'),

(6, 'Linus', 'Torvalds', 'Programmer', 1214, 'contractor'),

(7, 'Alan', 'Turing', 'Programmer', 22, 'contractor'),

(8, 'Aragorn', 'II Elessar Telcontarvalds', 'Fighter', 314, 'contractor');

INSERT INTO main.Contracts (
    id,
    terms,
    status,
    ClientId,
    ContractorId
) VALUES 

(1, 'bla bla bla', 'terminated', 1, 5),

(2, 'bla bla bla', 'in_progress', 1, 6),

(3, 'bla bla bla', 'in_progress', 2, 6),

(4, 'bla bla bla', 'in_progress', 2, 7),

(5, 'bla bla bla', 'new', 3, 8),

(6, 'bla bla bla', 'in_progress', 3, 7),

(7, 'bla bla bla', 'in_progress', 4, 7),

(8, 'bla bla bla', 'in_progress', 4, 6),

(9, 'bla bla bla', 'in_progress', 4, 8);


INSERT INTO Jobs (
    description,
    price,
    paid,
    paymentDate,
    ContractId
) VALUES
('work', 200, 0, NULL, 1),
('work', 201, 0, NULL, 2),
('work', 202, 0, NULL, 3),
('work', 200, 0, NULL, 4),
('work', 200, 0, NULL, 7),
('work', 2020, 1, '2020-08-15T19:11:26.737Z', 7),
('work', 200, 1, '2020-08-15T19:11:26.737Z', 2),
('work', 200, 1, '2020-08-16T19:11:26.737Z', 3),
('work', 200, 1, '2020-08-17T19:11:26.737Z', 1),
('work', 200, 1, '2020-08-17T19:11:26.737Z', 5),
('work', 21, 1, '2020-08-10T19:11:26.737Z', 1),
('work', 21, 1, '2020-08-15T19:11:26.737Z', 2),
('work', 121, 1, '2020-08-15T19:11:26.737Z', 3),
('work', 121, 1, '2020-08-14T23:11:26.737Z', 3),

