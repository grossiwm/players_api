-- Your SQL goes here
CREATE TABLE players (
    PlayerID INT AUTO_INCREMENT PRIMARY KEY,
    Name VARCHAR(255) NOT NULL,
    Balance DECIMAL(10, 2) NOT NULL,
    Email VARCHAR(255),
    Password VARCHAR(255),
);