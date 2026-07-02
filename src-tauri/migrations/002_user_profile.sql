CREATE TABLE user_profile (
    id TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL DEFAULT '',
    avatar_type TEXT NOT NULL DEFAULT 'icon' CHECK(avatar_type IN ('image', 'emoji', 'initials', 'icon')),
    avatar_data TEXT NOT NULL DEFAULT '',
    avatar_color TEXT NOT NULL DEFAULT '#6366f1',
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO user_profile (id, display_name) VALUES ('default', 'User');
