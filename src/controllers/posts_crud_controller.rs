use crate::model::models::{
    get_all_categories, get_all_categories_with_limit, get_categories_per_page, get_connection,
    get_count_of_categories, get_count_of_posts, get_max_id_of_category, get_max_id_of_post,
    get_posts_per_page, CategoryTemplate, CategoryTemplatePagination, Count, HomeTemplate,
    NewCategoryTemplate, NewPostTemplate, UpdateCategory, UpdateCategoryTemplate,
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
use std::collections::BTreeMap;

pub async fn get_connection_for_crud() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:Sakib123@localhost/blog_temp")
        .await
        .expect("failed to connect")
}

pub async fn create_posts_form_ui() -> impl IntoResponse {
    let categories: Vec<String> = vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "No Category".to_string(),
    ];
    let template = NewPostTemplate {
        index_sec: &categories,
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn create_posts_form(Form(create_post): Form<CreatePost>) -> impl IntoResponse {
    let pool = get_connection_for_crud().await;

    let category_id;
    if create_post.category_name.eq("Category A") {
        category_id = 1;
    } else if create_post.category_name.eq("Category B") {
        category_id = 2;
    } else if create_post.category_name.eq("Category C") {
        category_id = 3;
    } else {
        category_id = 4;
    }
    let m = get_max_id_of_post().await.unwrap();
    let post_id = (m[0].max) + 1; //insert into posts(post_id,post_title,post_body,category_id) values (16,'random','random',4);
    let _res = sqlx::query("insert into posts(post_id,post_title,post_body,category_id,post_description) values (($1),($2),($3),($4),($5))")
        .bind(post_id)// the id of post
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind(category_id)
        .bind("none")
        .execute(&pool)
        .await;
    Redirect::to("/admin/page/1")
}

pub async fn delete_posts_form(Path(post_id): Path<String>) -> Redirect {
    let pool = get_connection_for_crud().await;

    let id_of_post: i32 = post_id.clone().parse().unwrap();
    let _res = sqlx::query("delete from posts where post_id = ($1)")
        .bind(id_of_post)
        .execute(&pool)
        .await;

    Redirect::to("/admin/page/1")
}

pub async fn delete_categories_form(Path(category_id): Path<String>) -> Redirect {
    let pool = get_connection_for_crud().await;
    //let mut post_ids = post_id.clone().parse().u;
    let _res = sqlx::query("delete from category_post where category_name = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;

    Redirect::to("/admin/page/1")
}

pub async fn home_gui() -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pid: Vec<i32> = vec![];
    let mut pnav: Vec<i32> = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let page_number_integer: i32 = 1;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = get_posts_per_page(offset_start).await.unwrap();
    let number_of_pages: i64 = if get_vec_len_of_count(get_count_of_posts().await)
        % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };
    (1..number_of_pages + 1).for_each(|i| pnav.push(i as i32));
    plinks.clear(); // plinks.clear();
                    // let temp = s.as_ref();
                    // let list_iter = temp.clone().map(|posts| {
                    //     let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
                    //     let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
                    //     (v, v2)
                    // });
                    //(plinks, pid) = list_iter.unwrap_or_default();
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    pid = posts.iter().map(|post1| post1.post_id).collect();

    let template = HomeTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &pid,
        index_title: String::from("Posts"),
        page_number: &1,
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
    };

    template.render().map(Html).map_err(|err| {
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
    let category_list = get_all_categories_with_limit().await.unwrap();
    category_list.iter().for_each(|categories| {
        category_id_with_title.insert(categories.category_id, categories.category_name.clone());
        psec.push(categories.clone().category_name);
        category_ids.push(categories.clone().category_id);
    });
    let s = get_connection().await;
    let mut pnav = vec![];
    let number_of_posts_vector = get_count_of_categories().await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1).for_each(|i| pnav.push(i.to_string()));
    let temp = s.as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id).collect();
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

    template.render().map(Html).map_err(|err| {
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
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            psec.push(category.clone().category_name);
            category_ids.push(category.clone().category_id);
        })
    });
    let number_of_posts_vector = get_count_of_categories().await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1).for_each(|i| pnav.push(i.to_string()));
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

    template.render().map(Html).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn create_catgories_form(Form(create_category): Form<CreateCategory>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let m = get_max_id_of_category().await.unwrap();
    let category_id = m[0].max + 1;
    let _res =
        sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
            .bind(category_id) // category id
            .bind(create_category.category_name)
            .execute(&pool)
            .await;

    Redirect::to("/admin/page/1")
}

pub fn get_vec_len_of_count(result_of_count: Result<Vec<Count>, Error>) -> i64 {
    let mut len: i64 = 0;
    let tmp = result_of_count.as_ref();
    tmp.iter()
        .for_each(|posts| posts.iter().for_each(|count| len = count.count));

    len
}

pub async fn update_posts_form(
    Path(post_id): Path<String>,
    Form(update_post): Form<UpdatePost>,
) -> Redirect {
    let pool = get_connection_for_crud().await;

    let _res =
        sqlx::query("update posts set post_title = ($1), post_body = ($2) where post_id = ($3)")
            .bind(update_post.post_title)
            .bind(update_post.post_body)
            .bind(post_id)
            .execute(&pool)
            .await;

    Redirect::to("/admin")
}

pub async fn create_category_form_ui() -> impl IntoResponse {
    let template = NewCategoryTemplate {};
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn update_category_form_ui(Path(category_id): Path<String>) -> impl IntoResponse {
    let template = UpdateCategoryTemplate {
        index_name: "".to_string(),
        index_sec: category_id.parse().unwrap(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn update_category_form(
    Path(category_id): Path<String>,
    Form(update_category): Form<UpdateCategory>,
) -> Redirect {
    let pool = get_connection_for_crud().await;

    let _res =
        sqlx::query("update category_post set category_name = ($1) where category_name = ($2)")
            .bind(update_category.category_name)
            .bind(category_id)
            .execute(&pool)
            .await;

    Redirect::to("/posts")
}
