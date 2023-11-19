-- Your SQL goes here
CREATE TABLE book (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL
);

CREATE TABLE author (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE book_author (
  book_id INTEGER REFERENCES book(id),
  author_id INTEGER REFERENCES author(id),
  PRIMARY KEY(book_id, author_id)
);
