use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
// This controller contains the CRUD operations of posts
// Create, Read, Update and Delete method for posts.
use crate::model::models::{
    Blog, CategoryTemplate,
    CategoryTemplatePagination, Count, HomeTemplate, Max, NewCategoryTemplate, NewPostTemplate,
    UpdateCategory, UpdateCategoryTemplate,
};
use crate::{
    global_number_of_items_per_page, global_number_of_items_per_page_64, CreateCategory,
    CreatePost, UpdatePost,
};
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use std::string::String;
use std::vec::Vec;
use crate::controllers::base_controller::{get_all_categories, categories_with_limit, categories_per_page, category_by_name, category_by_id, posts_limit_3, pool_for_crud, count_of_categories, total_posts, max_of_category, max_of_post};

pub async fn create_posts_form_ui() -> impl IntoResponse {
    let category_list = get_all_categories().await;
    let mut category_in_template: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_in_template.push(category.clone().category_name);
        })
    });
    let template = NewPostTemplate {
        index_sec: &category_in_template,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn create_posts_form(Form(create_post): Form<CreatePost>) -> Redirect {
    let pool = pool_for_crud().await;
    let category_id = category_by_name(create_post.category_name)
        .await
        .first()
        .clone()
        .unwrap()
        .category_id;
    let max_post_id = max_of_post().await;
    let post_id = (get_max(max_post_id)) + 1;
    let _res= sqlx::query("insert into posts(post_id,post_title,post_body,post_description,category_id) values (($1),($2),($3),($4),($5))")
        .bind(post_id)
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind("none")
        .bind(category_id)
        .execute(&pool)
        .await;

    let _res =
        sqlx::query("insert into blogs(blog_id,post_id,category_id) values (($1),($2),($3))")
            .bind(post_id + 100)
            .bind(post_id.clone())
            .bind(category_id)
            .execute(&pool)
            .await;

    Redirect::to("/admin")
}

pub async fn delete_posts_form(Path(post_id): Path<i32>) -> Redirect {
    let pool = pool_for_crud().await;
    let _res = sqlx::query("delete from posts where post_id = ($1)")
        .bind(post_id)
        .execute(&pool)
        .await;

    Redirect::to("/admin/page/1")
}

pub async fn home_gui() -> impl IntoResponse {
    let mut category_in_template: Vec<String> = vec![];
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    category_in_template.clear();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });
    let posts = posts_limit_3().await.unwrap();
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let number_of_pages = (get_vec_len_of_count(total_posts().await) + 2)
        / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_numbers_in_navigation.push(index as i32));
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    let post_id_in_template = posts.iter().map(|post| post.post_id.clone()).collect();

    let template = HomeTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &1,
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
        current_url_page: ".".to_string(),
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn update_posts_form(
    Path(post_id): Path<i32>,
    Form(update_post): Form<UpdatePost>,
) -> Redirect {
    let pool = pool_for_crud().await;
    let category_id = category_by_name(update_post.category_name)
        .await
        .first()
        .unwrap()
        .category_id;
    let _res =
        sqlx::query("update posts set post_title=($1), post_body = ($2), category_id= ($3) where post_id = ($4) ;")
            .bind(update_post.post_title)
            .bind(update_post.post_body)
            .bind(category_id)
            .bind(post_id)
            .execute(&pool)
            .await;
    sqlx::query("update blogs set category_id=($1) where post_id = ($2) ;")
        .bind(category_id)
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("TODO: panic message");

    Redirect::to("/admin")
}

pub async fn update_posts_form2(
    Path(post_id): Path<i32>,
    Form(update_post): Form<UpdatePost>,
) -> std::result::Result<Redirect, Error> {
    let pool = pool_for_crud().await;
    let category = category_by_name(update_post.category_name).await.first().unwrap().clone().category_id;
    let _res =
        sqlx::query("  update posts set post_title=($1), post_body = ($2), category_id= ($3) from posts p inner join blogs b on p.post_id = b.post_id where p.post_id = ($4) ;")
            .bind(update_post.post_title)
            .bind(update_post.post_body)
            .bind(category)
            .bind(post_id)
            .execute(&pool)
            .await;

    Ok(Redirect::to("/posts"))
}

pub async fn create_category_form_ui() -> impl IntoResponse {
    let template = NewCategoryTemplate {};
    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn categories_with_pagination(
    Path(page_number): Path<String>,
) -> impl IntoResponse {
    let mut category_in_template = vec![];
    let mut category_ids = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut page_numbers_in_navigation = vec![];
    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = categories_per_page(offset_start).await;
    posts.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
            category_ids.push(category.clone().category_id);
        })
    });
    let number_of_posts_vector = count_of_categories().await;
    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_numbers_in_navigation.push(index.to_string()));
    let template = CategoryTemplatePagination {
        category_id_title: category_id_with_title,
        index_id: &vec![],
        category_id: &category_ids,
        index_title: String::from("Posts"),
        index_links: &vec![],
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub fn get_vec_len(result_of_blog: Arc<Result<Vec<Blog>, Error>>) -> i64 {
    let tmp = result_of_blog;
    let mut len: i64 = 0;
    tmp.iter().for_each(|posts| {
        len = posts.len() as i64;
    });

    len
}

pub fn get_vec_len_of_count(result_of_count: Result<Vec<Count>, Error>) -> i64 {
    let mut len: i64 = 0;
    let tmp = result_of_count.as_ref();
    tmp.iter()
        .for_each(|posts| posts.iter().for_each(|count| len = count.count));

    len
}

pub fn get_max(result_of_max: Result<Vec<Max>, Error>) -> i32 {
    let mut len: i32 = 0;
    result_of_max.iter().for_each(|posts| {
        posts.iter().for_each(|count| {
            len = count.max;
        });
    });

    len
}
