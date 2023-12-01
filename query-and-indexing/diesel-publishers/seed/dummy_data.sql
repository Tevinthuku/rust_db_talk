
-- Generate 500,000 authors
INSERT INTO author (name)
SELECT 'Author' || generate_series(1, 500000);

-- Generate 1,000,000 books
INSERT INTO book (title)
SELECT 'Book' || generate_series(1, 1000000);

-- Generate random book_author relationships
INSERT INTO book_author (book_id, author_id)
SELECT
  floor(random() * 1000000) + 1 as book_id,
  floor(random() * 500000) + 1 as author_id
FROM generate_series(1, 1000000) ON CONFLICT DO NOTHING;
