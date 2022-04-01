-- Your SQL goes here
DROP TABLE IF EXISTS comments;
CREATE TABLE comments(
  cid INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  aid INT(10) UNSIGNED,
  created TIMESTAMP NOT NULL DEFAULT now(),
  author_id INT(10) UNSIGNED NOT NULL,
  owner_id INT(10) UNSIGNED,
  text VARCHAR(255) NOT NULL,
  PRIMARY KEY (cid)
);