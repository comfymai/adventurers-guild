DROP TABLE posts;
CREATE TABLE posts (
    id VARCHAR(36) PRIMARY KEY,
    author_id VARCHAR(36) NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    kind INT NOT NULL,
    FOREIGN KEY(author_id) REFERENCES members(id)
)
