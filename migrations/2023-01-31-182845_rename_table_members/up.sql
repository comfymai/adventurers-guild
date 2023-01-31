DROP TABLE adventurers;
CREATE TABLE members (
    id VARCHAR(36) PRIMARY KEY,
    username VARCHAR(36) UNIQUE NOT NULL
)
