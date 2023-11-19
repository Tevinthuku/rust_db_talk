// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    book_author (book_id, author_id) {
        book_id -> Int4,
        author_id -> Int4,
    }
}

diesel::joinable!(book_author -> author (author_id));
diesel::joinable!(book_author -> book (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    book,
    book_author,
);
