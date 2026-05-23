CREATE TABLE IF NOT EXISTS products (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  price REAL NOT NULL,
  category TEXT NOT NULL,
  condition TEXT NOT NULL,
  location TEXT NOT NULL,
  contact TEXT NOT NULL,
  created_at TEXT NOT NULL
);
