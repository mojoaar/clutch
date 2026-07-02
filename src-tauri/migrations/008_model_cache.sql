CREATE TABLE model_cache (
    provider TEXT PRIMARY KEY NOT NULL,
    models TEXT NOT NULL DEFAULT '[]',
    last_updated TEXT NOT NULL DEFAULT (datetime('now')),
    etag TEXT NOT NULL DEFAULT '',
    version TEXT NOT NULL DEFAULT '0'
);
