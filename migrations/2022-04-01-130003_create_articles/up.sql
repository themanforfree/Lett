-- Your SQL goes here
CREATE TABLE articles(
  aid INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  title VARCHAR(150) NOT NULL,
  content TEXT NOT NULL,
  created BIGINT NOT NULL,
  published TINYINT(1) DEFAULT 0 NOT NULL,
  comments_num INT(10) DEFAULT 0 NOT NULL,
  PRIMARY KEY (aid)
);