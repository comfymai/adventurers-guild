CREATE TABLE posts (
    id VARCHAR(36) PRIMARY KEY,
    author_id VARCHAR(36),
    title TEXT,
    content TEXT,
    type INT,
    FOREIGN KEY(author_id) REFERENCES members(id)
)
