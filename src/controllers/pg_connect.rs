use sqlx::postgres::PgPoolOptions;
use crate::{Blog, Post};

pub async fn connect_to_pg() -> Vec<Post>{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");// could not connect to database

    let mut posts = sqlx::query_as::<_, Post>("select post_title, post_body, post_description from posts")
        .fetch_all(&pool)
        .await
        .unwrap();


    // let mut posts2 = sqlx::query_as::<_, Blog>("select p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_name = ($1)")
    //     .bind(query_title)
    //     .fetch_all(&pool) fetch all pool await
    //     .await
    //     .unwrap();

    return posts;
}
