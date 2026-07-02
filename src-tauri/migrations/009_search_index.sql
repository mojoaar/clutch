CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
  content,
  session_id,
  session_title
);
