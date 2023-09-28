use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
// This controller contains the CRUD operations of posts
// Create, Read, Update and Delete method for posts.
use crate::model::models::{
    get_all_categories, get_all_categories_with_limit, get_categories_per_page,
    get_category_id_by_name, get_category_name_by_id, get_connection, get_count_of_categories,
    get_count_of_posts, get_max_id_of_category, get_max_id_of_post, Blog, CategoryTemplate,
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
use std::vec::Vec;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};
use std::string::String;

pub async fn get_connection_for_crud() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("failed to connect")
}

pub async fn create_posts_form_ui() -> impl IntoResponse {
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let template = NewPostTemplate { index_sec: &psec };


    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn create_posts_form(Form(create_post): Form<CreatePost>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let mut category_id_from_vec = 0;
    let category_id = get_category_id_by_name(create_post.category_name).await;
    let iter = category_id.iter();
    for i in iter {
        category_id_from_vec = i.category_id;
    }
    let m = get_max_id_of_post().await;
    let post_id = (get_max(m)) + 1;
    let _res= sqlx::query("insert into posts(post_id,post_title,post_body,post_description,category_id) values (($1),($2),($3),($4),($5))")
        .bind(post_id)
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind("none")
        .bind(category_id_from_vec)
        .execute(&pool)
        .await;

    let _res = sqlx::query("insert into blogs(blog_id,post_id,category_id) values (($1),($2),($3))")
            .bind(post_id + 100)
            .bind(post_id.clone())
            .bind(category_id_from_vec)
            .execute(&pool)
            .await;

    Redirect::to("/admin")
}

pub async fn delete_posts_form(Path(post_id): Path<i32>) -> Redirect {
    let pool = get_connection_for_crud().await;
    println!("Form {}", post_id);
    let _res = sqlx::query("delete from posts where post_id = ($1)")
        .bind(post_id)
        .execute(&pool)
        .await;
    Redirect::to("/admin/page/1")
}

pub async fn delete_categories_form(Path(category_id): Path<i32>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let _res = sqlx::query("update posts set category_id = null where category_id = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;
    let _res = sqlx::query("delete from category_post where category_id = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;
    Redirect::to("/admin/categories")
}

pub async fn home_gui() -> impl IntoResponse {
    let mut psec: Vec<String> = vec![];
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let posts = get_connection().await.unwrap();

    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let mut pnav: Vec<i32> = vec![];
    let number_of_pages = (get_vec_len_of_count(get_count_of_posts().await) + 2)
        / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i as i32));
    posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    let pids = posts.iter().map(|post1| post1.post_id.clone()).collect();

    let template = HomeTemplate {
        post_id_title: post_id_with_title,
        category_id_title:category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &1,
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn create_catgories_form(Form(create_category): Form<CreateCategory>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let m = get_max_id_of_category().await;
    let category_id = get_max(m) + 1;
    let _res =
        sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
            .bind(category_id)
            .bind(create_category.category_name)
            .execute(&pool)
            .await;
    Redirect::to("/admin")
}

pub async fn update_posts_form(
    Path(post_id): Path<i32>,
    Form(update_post): Form<UpdatePost>,
) -> Redirect {
    let pool = get_connection_for_crud().await;
    let mut m1 = 0;
    let category_id = get_category_id_by_name(update_post.category_name).await;
    let m = category_id.iter();
    for i in m {
        m1 = i.category_id;
    }
    let category = m1;
    let _res =
        sqlx::query("update posts set post_title=($1), post_body = ($2), category_id= ($3) where post_id = ($4) ;")
            .bind(update_post.post_title)
            .bind(update_post.post_body)
            .bind(category)
            .bind(post_id)
            .execute(&pool)
            .await;
    sqlx::query("update blogs set category_id=($1) where post_id = ($2) ;")
        .bind(category)
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("TODO: panic message");
    Redirect::to("/admin")
}

pub async fn update_posts_form2(
    Path(post_id): Path<i32>,
    Form(update_post): Form<UpdatePost>,
) -> std::result::Result<Redirect,Error> {
    let pool = get_connection_for_crud().await;
    let mut m1 = 0;
    let category_id = get_category_id_by_name(update_post.category_name).await;
    let m = category_id.iter();
    for i in m {
        m1 = i.category_id; // category id iter await
    }
    let category = m1;
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

pub async fn show_all_categories() -> impl IntoResponse {
    let mut psec = vec![];
    let mut category_ids = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_list = get_all_categories_with_limit().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
            category_ids.push(category.clone().category_id);
        })
    });
    let s = get_connection().await;
    let mut pnav = vec![];
    let number_of_posts_vector = get_count_of_categories().await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let temp = s.as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v, v2)
    });

    let (plinks, pids) = list_iter.unwrap_or_default();
    let template = CategoryTemplate {
        category_id_title: category_id_with_title,
        index_id: &pids,
        category_id: &category_ids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn show_all_categories_with_pagination(
    Path(page_number): Path<String>,
) -> impl IntoResponse {
    let mut psec = vec![];
    let mut category_ids = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let _category_list = get_all_categories_with_limit().await;
    let mut pnav = vec![];
    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let s = get_categories_per_page(offset_start).await;
    s.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
            category_ids.push(category.clone().category_id);
        })
    });
    let number_of_posts_vector = get_count_of_categories().await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let _temp = s.as_ref();
    let template = CategoryTemplatePagination {
        category_id_title: category_id_with_title,
        index_id: &vec![],
        category_id: &category_ids,
        index_title: String::from("Posts"),
        index_links: &vec![],
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn update_category_form_ui(Path(category_id): Path<i32>) -> impl IntoResponse {
    let categoory_ids = category_id;
    let mut temp_category = " ".to_string();
    let temp_string = get_category_name_by_id(category_id).await;
    let iter = temp_string.iter();
    for i in iter {
        temp_category = i.category_name.clone();
    }
    let mut category_names = temp_category;
    let template = UpdateCategoryTemplate {
        index_name: category_names.to_string(),
        index_sec: category_id,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn update_category_form(
    Path(category_id): Path<i32>,
    Form(update_category): Form<UpdateCategory>,
) -> Redirect {
    let pool = get_connection_for_crud().await;
    let _res =
        sqlx::query("update category_post set category_name = ($1) where category_id = ($2)")
            .bind(update_category.category_name)
            .bind(category_id)
            .execute(&pool)
            .await;
    Redirect::to("/admin/categories")
}

pub fn get_vec_len(shared_state2: Arc<Result<Vec<Blog>, Error>>) -> i64 {
    let temp = shared_state2;
    let mut len: i64 = 0;
    temp.iter().for_each(|posts| {
        len = posts.len() as i64;
    });
    len
}
pub fn get_vec_len_of_count(shared_state2: Result<Vec<Count>, Error>) -> i64 {
    let mut len1: i64 = 0;
    let temp = shared_state2.as_ref();
    temp.iter()
        .for_each(|posts| posts.iter().for_each(|count| len1 = count.count));
    len1
}

pub fn get_max(shared_state2: Result<Vec<Max>, Error>) -> i32 {
    let mut len2: i32 = 0;
    shared_state2.iter().for_each(|posts| {
        posts.iter().for_each(|count| {
            len2 = count.max;
        });
    });

    len2
}
