-- Your SQL goes here
DROP TABLE IF EXISTS settings;
CREATE TABLE settings(
  name VARCHAR(255) NOT NULL,
  value VARCHAR(255),
  PRIMARY KEY (name)
);