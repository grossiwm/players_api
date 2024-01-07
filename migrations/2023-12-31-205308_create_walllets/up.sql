-- Your SQL goes here
CREATE TABLE wallets (
    wallet_id INT AUTO_INCREMENT PRIMARY KEY,
    player_id INT NOT NULL UNIQUE,
    `address` VARCHAR(255) NOT NULL UNIQUE,
    private_key VARCHAR(255) NOT NULL,
    FOREIGN KEY (player_id) REFERENCES players(player_id)
);