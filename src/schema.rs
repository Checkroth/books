table! {
    author_titles (id) {
        id -> Int4,
        author_id -> Int4,
        title_id -> Int4,
    }
}

table! {
    authors (id) {
        id -> Int4,
        author_name -> Varchar,
        birth_date -> Nullable<Date>,
        picture -> Nullable<Varchar>,
    }
}

table! {
    distributions (id) {
        id -> Int4,
        distribution_name -> Nullable<Varchar>,
        publisher_id -> Int4,
        title_id -> Int4,
        page_count -> Nullable<Int4>,
        max_pos -> Nullable<Int4>,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        publisher_name -> Varchar,
    }
}

table! {
    reader_distributions (id) {
        id -> Int4,
        user_id -> Int4,
        distribution_id -> Int4,
        last_pos -> Int4,
        last_read -> Nullable<Date>,
        finished_on -> Nullable<Date>,
    }
}

table! {
    readers (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
    }
}

table! {
    titles (id) {
        id -> Int4,
        publisher_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        email_verified -> Bool,
    }
}

joinable!(author_titles -> authors (author_id));
joinable!(reader_distributions -> distributions (distribution_id));

allow_tables_to_appear_in_same_query!(
    author_titles,
    authors,
    distributions,
    publishers,
    reader_distributions,
    readers,
    titles,
    users,
);
