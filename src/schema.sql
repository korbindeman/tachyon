CREATE TABLE IF NOT EXISTS transfers (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    path TEXT NOT NULL,
    download_count INTEGER DEFAULT 0,
    filesize INTEGER NOT NULL,
    mime_type VARCHAR(255)
);
