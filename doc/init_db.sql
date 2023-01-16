CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW (),
  deleted_at TIMESTAMPTZ DEFAULT NULL,
  token TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS transactions (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  status TEXT DEFAULT '0',
  type TEXT DEFAULT '0',
  created_at TIMESTAMPTZ DEFAULT NOW (),
  deleted_at TIMESTAMPTZ DEFAULT NULL,
  user_id INTEGER DEFAULT NULL,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

INSERT INTO
  users (username, password)
VALUES
  ('mike', 'mpass');

INSERT INTO
  transactions (title, deleted_at, user_id)
VALUES
  (
    'deleted task',
    NOW(),
    (
      select
        id
      from
        users
      where
        username = 'mike'
    )
  );

INSERT INTO
  transactions (title, status, type)
VALUES
  ('a transaction', '2', '12'),
  ('another transaction', '3', '34');