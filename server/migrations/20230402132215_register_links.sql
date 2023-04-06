CREATE TABLE
  IF NOT EXISTS register_links (
    link VARCHAR(255) NOT NULL UNIQUE PRIMARY KEY,
    used boolean DEFAULT False
  )
