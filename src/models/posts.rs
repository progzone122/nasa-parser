use diesel::sql_types::Integer;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::parser::news::Post;
use crate::schema::posts;
#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Posts {
    pub id: i32,
    pub html: String,
    pub text: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPost<'a> {
    pub html: &'a str,
    pub text: &'a str,
}

#[derive(QueryableByName, Debug)]
struct LastInsertedId {
    #[diesel(sql_type = Integer)]
    id: i32,
}

pub fn get_all(conn: &mut MysqlConnection) -> QueryResult<Vec<Posts>> {
    posts::dsl::posts.load::<Posts>(conn)
}
pub fn push(
    conn: &mut MysqlConnection,
    new_post: &NewPost,
) -> QueryResult<i32> {
    diesel::insert_into(posts::dsl::posts)
        .values(new_post)
        .execute(conn)?;

    let post: LastInsertedId = diesel::sql_query("SELECT LAST_INSERT_ID() as id")
        .get_result(conn)?;

    Ok(post.id)
}