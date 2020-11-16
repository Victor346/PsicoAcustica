use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::schema::comments;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub author: String,
    pub created_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="comments"]
pub struct NewComment {
    pub body: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  CommentsListResult {
    pub total: i64,
    pub comments: Vec<Comment>,
}