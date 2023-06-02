use std::collections::HashMap;
use std::sync::Arc;
use askama::Template;
use axum::extract::State;
use axum::{Extension, Form};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use crate::model::models::{AdminTemplate, IndexTemplate, LoginTemplate, RegisterTemplate, RegisterUsers, Users};
use crate::{User};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    memory_store::MemoryStore as AuthMemoryStore,
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};//
use axum_login::secrecy::ExposeSecret;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;


fn get_password_hash_form(pass: String) -> SecretVec<u8> {
    let sec = SecretVec::new(pass.clone().into());
    println!("Hello");
    println!("password final {:?}",sec.expose_secret());
    sec// secret vector
}

type AuthContext = axum_login::extractors::AuthContext<i64,User,AuthMemoryStore<i64,User>>;

pub async fn login_user(Extension(user_state): Extension<User>, mut auth: AuthContext, Form(user):Form<Users>) -> Redirect{

    let mut user_cred:User = User {
        id: 2,
        name: user.user_name,
        password_hash: user.password,
    };

    println!("pass in login {:?}",user_cred.password_hash.clone());
    auth.login(&user_cred).await.unwrap();
    println!("Logged in {:?}",&auth.current_user);
    Redirect::to("/admins")

}

pub async fn login_user_ui() -> impl IntoResponse {
    let template = LoginTemplate{ user_name: "Username", password: "Password" };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),// into response
    }
}

pub async fn register_user_ui() -> impl IntoResponse {
    let template = RegisterTemplate{ user_name: "Username", password: "Password", repeat_password: "Repeat Password" };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}

pub async fn register_user(Form(user):Form<RegisterUsers>){
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");
    //println!("Form {}", update_post.post_title);
    let mut res = sqlx::query("insert into users(user_id,user_name,user_password) values (2,($1),($2))")
        .bind(user.clone().user_names.clone())
        .bind(get_password_hash_form(user.clone().passwords.clone()).expose_secret())
        .execute(&pool)
        .await;

    println!("user inserted {:?}",res);

}

pub async fn admin_gui() -> impl IntoResponse {
    let template = AdminTemplate{ };

    match template.render() {
    Ok(html) => Html(html).into_response(),
    Err(err) => (
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("Failed to render template. Error {}", err),
    ).into_response(),
}

}