use crate::model::models::{AdminTemplate, LoginTemplate, RegisterTemplate, RegisterUsers, Users};
use crate::User;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Extension, Form};
use axum_login::secrecy::ExposeSecret;
use axum_login::{memory_store::MemoryStore as AuthMemoryStore, secrecy::SecretVec};

use sqlx::postgres::PgPoolOptions;

fn get_password_hash_form(pass: String) -> SecretVec<u8> {
    SecretVec::new(pass.into())
}

fn get_password_hash_form2(pass: String) -> SecretVec<u8> { SecretVec::new(pass.into())}

type AuthContext = axum_login::extractors::AuthContext<i64, User, AuthMemoryStore<i64, User>>;

pub async fn login_user(
    Extension(_user_state): Extension<User>,
    mut auth: AuthContext,
    Form(user): Form<Users>,
) -> Redirect {
    let user_cred: User = User {
        id: 2,
        name: user.user_name,
        password_hash: user.password,
    };
    // if auth.login(&user_cred).await.unwrap().eq(None) {
    //     println!("Logged in {:?}", &auth.current_user);
    //     Redirect::to("/posts")
    // }
    // else{
    //     Redirect::to("/login")
    // }
    //if auth.login(&user_cred).await.unwrap().eq() { }
    println!("err ");
    match auth.login(&user_cred).await{
        Ok(inner)=> {println!("inner"); Redirect::to("/admin")}
        Err(_)=> {println!("error "); Redirect::to("/login")
        }

    }

    // auth.login(&user_cred).await.unwrap();
    // println!("Logged in {:?}", &user_cred);
    // Redirect::to("/posts")
}

pub async fn login_user_ui() -> impl IntoResponse {
    let template = LoginTemplate {
        user_name: "Username",
        password: "Password",
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

pub async fn register_user_ui() -> impl IntoResponse {
    let template = RegisterTemplate {
        user_name: "Username",
        password: "Password",
        repeat_password: "Repeat Password",
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

pub async fn register_user(Form(user): Form<RegisterUsers>) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");
    let res =
        sqlx::query("insert into users(user_id,user_name,user_password) values (2,($1),($2))")
            .bind(user.clone().user_names.clone())
            .bind(get_password_hash_form(user.clone().passwords.clone()).expose_secret())
            .execute(&pool)
            .await;

    println!("user inserted {:?}", res);
}

pub async fn admin_gui() -> impl IntoResponse {
    let template = AdminTemplate {};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
