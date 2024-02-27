-- Create Location table
CREATE TABLE Location (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    state TEXT
);

-- Insert dummy data into Location
INSERT INTO Location (state) VALUES
('State1'), ('State2'), ('State3'), ('State4'), ('State5');

-- Create Candidate table
CREATE TABLE Candidate (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    Name TEXT,
    Surname TEXT,
    Email TEXT CHECK(length(Email) <= 80)
);

-- Insert dummy data into Candidate
INSERT INTO Candidate (Name, Surname, Email) VALUES
('Name1', 'Surname1', 'email1@example.com'),
('Name2', 'Surname2', 'email2@example.com'),
('Name3', 'Surname3', 'email3@example.com'),
('Name4', 'Surname4', 'email4@example.com'),
('Name5', 'Surname5', 'email5@example.com');

-- Create Job table
CREATE TABLE Job (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    StartDate DATETIME,
    EndDate DATETIME,
    LocationId INTEGER,
    FOREIGN KEY(LocationId) REFERENCES Location(id)
);

-- Insert dummy data into Job
INSERT INTO Job (StartDate, EndDate, LocationId) VALUES
('2024-01-01', '2024-01-31', 1),
('2024-02-01', '2024-02-28', 2),
('2024-03-01', '2024-03-31', 3),
('2024-04-01', '2024-04-30', 4),
('2024-05-01', '2024-05-31', 5);

-- Create Shifts table
CREATE TABLE Shifts (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    CandidateId INTEGER,
    StartDate DATE,
    StartTime TIME,
    EndDate DATE,
    EndTime TIME,
    FOREIGN KEY(CandidateId) REFERENCES Candidate(id)
);

-- Insert dummy data into Shifts
INSERT INTO Shifts (CandidateId, StartDate, StartTime, EndDate, EndTime) VALUES
(1, '2024-01-01', '09:00', '2024-01-01', '17:00'),
(2, '2024-01-02', '09:00', '2024-01-02', '17:00'),
(3, '2024-01-03', '09:00', '2024-01-03', '17:00'),
(4, '2024-01-04', '09:00', '2024-01-04', '17:00'),
(5, '2024-01-05', '09:00', '2024-01-05', '17:00');
