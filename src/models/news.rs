use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::news;
use crate::schema::news::short_description;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::news)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct News {
    pub id: i32,
    pub title: String,
    pub short_description: Option<String>,
    pub image: Option<String>,
    pub url: String,
    pub post_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::news)]
pub struct NewNews<'a> {
    pub title: &'a str,
    pub short_description: Option<&'a str>,
    pub image: Option<&'a str>,
    pub url: &'a str,
    pub post_id: i32,
}


pub fn get_all(conn: &mut MysqlConnection) -> QueryResult<Vec<News>> {
    news::dsl::news.load::<News>(conn)
}
pub fn push<'a>(
    conn: &mut MysqlConnection,
    new_news: &'a NewNews<'a>
) -> QueryResult<usize> {
    diesel::insert_into(news::dsl::news)
        .values(new_news)
        .execute(conn)
}