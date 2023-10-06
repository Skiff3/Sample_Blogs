// should contain functions used by all controllers
// build or render templates
// db functions
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::GlobalPool;
use crate::{global_number_of_items_per_page};
use crate::model::models::{Blog, Category, Category_Id, Category_Name, Count, Max, Post, Post_Name};


pub async fn pool_for_crud() -> Pool<Postgres> {
    let pg_pool: Pool<Postgres> = GlobalPool::global_pool().await;
    pg_pool
}

pub async fn posts_limit_3() -> std::result::Result<Vec<Post>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Post>(
        "select post_id,post_title, post_body, post_description from posts limit 3 offset 0",
    )
        .fetch_all(&pool)
        .await
}

pub async fn posts_per_page(offset_value: i32) -> std::result::Result<Vec<Post>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Post>(
        "select post_id, post_title, post_body, post_description from posts limit ($1) offset ($2)",
    )
        .bind(global_number_of_items_per_page())
        .bind(offset_value)
        .fetch_all(&pool)
        .await
}

pub async fn categories_per_page(
    offset_value: i32,
) -> std::result::Result<Vec<Category>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Category>(
        "select * from category_post limit ($1) offset ($2) ORDER BY category_id ASC",
    )
        .bind(global_number_of_items_per_page())
        .bind(offset_value)
        .fetch_all(&pool)
        .await
}

pub async fn get_all_categories() -> std::result::Result<Vec<Category>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Category>("select * from category_post ORDER BY category_id ASC")
        .fetch_all(&pool)
        .await
}

pub async fn category_by_name(category_name: String) -> Vec<Category_Id> {
    let pool = pool_for_crud().await;
    let res = sqlx::query_as::<_, Category_Id>(
        "select category_id from category_post where category_name = ($1);",
    )
        .bind(category_name)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn post_by_id(post_id: i32) -> Vec<Post_Name> {
    let pool = pool_for_crud().await;
    let res = sqlx::query_as::<_, Post_Name>("select post_title from posts where post_id = ($1);")
        .bind(post_id)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn category_by_id(category_id: i32) -> Vec<Category_Name> {
    let pool = pool_for_crud().await;
    let res = sqlx::query_as::<_, Category_Name>(
        "select category_name from category_post where category_id = ($1);",
    )
        .bind(category_id)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn category_by_post_id(post_name: String) -> Vec<Category_Name> {
    let pool = pool_for_crud().await;
    let res = sqlx::query_as::<_, Category_Name>(
        "select category_name from category_post c,posts p where c.category_id = p.category_id and post_title = ($1);",
    )
        .bind(post_name)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn categories_with_limit() -> std::result::Result<Vec<Category>, Error> {
    let pool = pool_for_crud().await;
                 sqlx::query_as::<_, Category>("select * from category_post limit 3")
        .fetch_all(&pool)
        .await
}

pub async fn details_of_post(post_id: i32) -> std::result::Result<Vec<Post>, Error> {
    let pool = pool_for_crud().await;

    sqlx::query_as::<_, Post>(
        "select post_id, post_title, post_body, post_description from posts where post_id = ($1)",
    )
        .bind(post_id)
        .fetch_all(&pool)
        .await
}

pub async fn max_of_post() -> std::result::Result<Vec<Max>, Error> {
    let pool = pool_for_crud().await;

    sqlx::query_as::<_, Max>("select max(post_id) from posts;")
        .fetch_all(&pool)
        .await
}

pub async fn max_of_category() -> std::result::Result<Vec<Max>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Max>("select max(category_id) from category_post;")
        .fetch_all(&pool)
        .await
}

pub async fn total_posts() -> std::result::Result<Vec<Count>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Count>("select count(*) from posts;")
        .fetch_all(&pool)
        .await
}

pub async fn count_of_categories() -> std::result::Result<Vec<Count>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Count>("select count(*) from category_post;")
        .fetch_all(&pool)
        .await
}

pub async fn filtered_cat_database(
    final_category: i32,
    page_number: i32,
) -> std::result::Result<Vec<Blog>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1) limit 3 offset ($2)")
        .bind(final_category)
        .bind(page_number)
        .fetch_all(&pool)
        .await
}

pub async fn get_filtered_cat(
    final_category: i32,
) -> std::result::Result<Vec<Blog>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1) limit 3")
        .bind(final_category)
        .fetch_all(&pool)
        .await
}
pub async fn filtered_cat() -> std::result::Result<Vec<Blog>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id limit 3")
        .fetch_all(&pool)
        .await
}
pub async fn count_filtered_cat(
    final_category: i32,
) -> std::result::Result<Vec<Count>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Count>("select count(p.post_id) from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1)")
        .bind(final_category)
        .fetch_all(&pool)
        .await
}
pub async fn count_of_postsdb(
) -> std::result::Result<Vec<Count>, Error> {
    let pool = pool_for_crud().await;
    sqlx::query_as::<_, Count>("select count(p.post_id) from posts p; ")
        .fetch_all(&pool)
        .await
}
