-- Your SQL goes here
CREATE TABLE users (
  id            VARCHAR PRIMARY KEY NOT NULL,
  username      NVARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,  -- salted password hash
  salt          VARCHAR NOT NULL,  -- password salt
  full_name     NVARCHAR NOT NULL, -- full name of account
  created_time  DATETIME NOT NULL  -- time created
)
