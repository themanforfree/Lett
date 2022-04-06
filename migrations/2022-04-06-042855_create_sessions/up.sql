-- Your SQL goes here
DROP TABLE IF EXISTS sessions;
CREATE TABLE sessions(
  sid VARCHAR(255) NOT NULL,
  data TEXT DEFAULT NULL,
  expiration BIGINT NOT NULL,
  PRIMARY KEY (sid)
);