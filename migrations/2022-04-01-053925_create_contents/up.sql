-- Your SQL goes here
DROP TABLE IF EXISTS contents;
CREATE TABLE contents(
  cid INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  title VARCHAR(150),
  created DATETIME NOT NULL,
  modified DATETIME NOT NULL,
  authorId INT(10) UNSIGNED,
  published VARCHAR(1) DEFAULT 'f',
  commentsNum INT(10) DEFAULT 0,
  PRIMARY KEY (cid)
);