use diesel::prelude::*;

use crate::models;

pub fn get_paged_comments(conn: &PgConnection, page: i64)
    -> Result<Vec<models::Comment>, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let result = comments
        .select((
            id,
            body,
            author,
            created_at
            ))
        .limit(10)
        .offset(10 * (page - 1))
        .order_by(created_at.desc())
        .load::<models::Comment>(conn)?;

    Ok(result)
}

pub fn get_count_comments(conn: &PgConnection)
                          -> Result<i64, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    comments.count().get_result(conn)
}

pub fn insert_comment(conn: &PgConnection, comment: models::NewComment)
    -> Result<i32, diesel::result::Error> {
    use crate::schema::comments;
    diesel::insert_into(comments::table)
        .values(&comment)
        .returning(comments::id)
        .get_result(conn)
}