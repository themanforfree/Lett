-- Your SQL goes here
DROP TABLE IF EXISTS articles;
CREATE TABLE articles(
  aid INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  title VARCHAR(150),
  content TEXT,
  created TIMESTAMP NOT NULL DEFAULT now(),
  modified TIMESTAMP NOT NULL,
  author_id INT(10) UNSIGNED,
  published TINYINT(1) DEFAULT 0 NOT NULL,
  comments_num INT(10) DEFAULT 0 NOT NULL,
  PRIMARY KEY (aid)
);