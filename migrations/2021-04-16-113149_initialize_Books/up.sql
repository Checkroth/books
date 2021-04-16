-- Your SQL goes here
CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  author_name VARCHAR NOT NULL,
  birth_date DATE,
  picture VARCHAR
);

CREATE TABLE publishers (
  id SERIAL PRIMARY KEY,
  publisher_name VARCHAR NOT NULL
);

CREATE TABLE titles (
  id SERIAL PRIMARY KEY,
  publisher_id INT NOT NULL,
  CONSTRAINT fk_publisher
    FOREIGN KEY(publisher_id) REFERENCES publishers(id)
);

CREATE TABLE author_titles (
  id SERIAL PRIMARY KEY,
  author_id INT NOT NULL,
  title_id INT NOT NULL,
  CONSTRAINT fk_author
    FOREIGN KEY(author_id) REFERENCES authors(id),
  CONSTRAINT fk_title
    FOREIGN KEY(title_id) REFERENCES titles(id)
);

CREATE TABLE distributions (
  id SERIAL PRIMARY KEY,
  distribution_name VARCHAR,
  publisher_id INT NOT NULL,
  title_id INT NOT NULL,
  page_count INT,
  max_pos INT,
  CONSTRAINT fk_publisher
    FOREIGN KEY(publisher_id) REFERENCES publishers(id),
  CONSTRAINT fk_title
    FOREIGN KEY(title_id) REFERENCES titles(id)
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  email_verified BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE readers (
  id SERIAL PRIMARY KEY,
  user_id INT,
  CONSTRAINT fk_user
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE reader_distributions (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  distribution_id INT NOT NULL,
  last_pos INT NOT NULL DEFAULT 0,
  last_read DATE,
  finished_on DATE,
  CONSTRAINT fk_user
    FOREIGN KEY(user_id) REFERENCES users(id),
  CONSTRAINT fk_distribution
    FOREIGN KEY (distribution_id) REFERENCES distributions(id)
);
