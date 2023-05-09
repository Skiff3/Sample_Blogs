use askama::Template;
use sqlx::*;
use sqlx::postgres::PgPoolOptions;

mod filters {
    pub fn rmdashes(title: &str) -> askama::Result<String> {
        Ok(title.replace("-", " ").into())
    }
}

#[derive(FromRow,Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate<'a> {
    pub  post_title: &'a str,
    pub  post_description: &'a str,
    pub  post_body: &'a str,
}


#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
    pub current_url_page: String,
}


#[derive(FromRow,Debug, Clone)]
pub struct Blog {
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
}

pub async fn get_connection() -> Vec<Post> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await// await
        .expect("couldn't connect to the database");

    let mut posts = sqlx::query_as::<_, Post>("select post_title, post_body, post_description from posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    posts
}

pub async fn get_filtered_from_database(final_category: String,page_number: i32) -> Vec<Blog> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await// await
        .expect("couldn't connect to the database");

    let mut posts2 = sqlx::query_as::<_, Blog>("select p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_name = ($1) limit 3 offset ($2)")
        .bind(final_category)
        .bind(page_number)
        .fetch_all(&pool)
        .await
        .unwrap();

    posts2
}

pub async fn get_filtered_from_database_by_category(final_category: String) -> Vec<Blog> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await// await
        .expect("couldn't connect to the database");

    let mut posts2 = sqlx::query_as::<_, Blog>("select p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_name = ($1)")
        .bind(final_category)
        .fetch_all(&pool)
        .await
        .unwrap();

    posts2
}