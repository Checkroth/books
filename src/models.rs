use chrono::{NaiveDate};
use super::schema::*;

#[derive(Queryable, Identifiable)]
pub struct Author {
    pub id: i32,
    pub author_name: String,
    pub birth_date: Option<NaiveDate>,
    pub picture: Option<String>,
}

#[derive(Queryable, Identifiable)]
pub struct Publisher {
    pub id: i32,
    pub publisher_name: String,
}

#[derive(Queryable, Identifiable, Associations)]
pub struct Title {
    pub id: i32,
    pub publisher_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Author)]
#[belongs_to(Title)]
pub struct AuthorTitle {
    pub id: i32,
    pub author_id: i32,
    pub title_id: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Publisher)]
#[belongs_to(Title)]
pub struct Distribution {
    pub id: i32,
    pub distribution_name: Option<String>,
    pub publisher_id: i32,
    pub title_id: i32,
    pub page_count: Option<i32>,
    pub max_pos: Option<i32>,
}

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub email_verified: bool,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Distribution)]
pub struct ReaderDistribution {
    pub id: i32,
    pub user_id: i32,
    pub distribution_id: i32,
    pub last_pos: i32,
    pub last_read: Option<NaiveDate>,
    pub finished_on: Option<NaiveDate>,
}
