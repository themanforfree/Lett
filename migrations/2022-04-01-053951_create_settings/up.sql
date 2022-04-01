-- Your SQL goes here
DROP TABLE IF EXISTS setting;
CREATE TABLE setting(
  name VARCHAR(255) NOT NULL,
  value VARCHAR(255),
  PRIMARY KEY (name)
);