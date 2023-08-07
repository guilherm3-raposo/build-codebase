#[macro_use]
extern crate lazy_static;

use actix_files as fs;
use actix_web::{
    cookie::Cookie, error, get, http::StatusCode, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use serde::Deserialize;

mod config;
mod persistence;
mod security;
mod util;

lazy_static! {
    static ref CONFIG: config::Config = config::get();
}

#[derive(Deserialize)]
struct LoginForm {
    pw: String,
}

#[derive(Deserialize)]
struct BuildForm {
    project: String,
}

fn redirect_forbidden() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", format!("{}/403", CONFIG.root)))
        .finish()
}

fn redirect_internal_server_error() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", format!("{}/500", CONFIG.root)))
        .finish()
}

fn idx(request: HttpRequest) -> impl Responder {
    if !security::is_logged_in(request) {
        return HttpResponse::Found()
            .append_header(("Location", format!("{}/login", CONFIG.root)))
            .finish();
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html").replace("#BASE#", &CONFIG.root))
}

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", format!("{}", CONFIG.root)))
        .finish()
}

#[get("")]
async fn index(request: HttpRequest) -> impl Responder {
    idx(request)
}

#[get("/")]
async fn index_slash(request: HttpRequest) -> impl Responder {
    idx(request)
}

#[get("/login")]
async fn login(request: HttpRequest) -> impl Responder {
    if security::is_logged_in(request) {
        HttpResponse::Found()
            .append_header(("Location", format!("{}", CONFIG.root)))
            .finish()
    } else {
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/login.html").replace("#BASE#", &CONFIG.root))
    }
}

#[post("/do_login")]
async fn do_login(form: web::Form<LoginForm>) -> impl Responder {
    if !security::is_valid_token(form.pw.clone()) {
        return redirect_forbidden();
    }

    let res = security::add_key();

    if res.is_err() {
        return redirect_internal_server_error();
    };

    HttpResponse::Found()
        .cookie(Cookie::build("_session", res.unwrap()).finish())
        .append_header(("Location", CONFIG.root.as_str()))
        .finish()
}

#[get("/data")]
async fn data() -> Result<impl Responder> {
    match persistence::get_data() {
        Ok(u) => Ok(web::Json(u)),
        Err(err) => Err(error::ErrorInternalServerError(web::Json(format!(
            "{{\"reason\":\"{}\"}}",
            err.reason
        )))),
    }
}

#[post("/build")]
async fn build(request: HttpRequest, form: web::Form<BuildForm>) -> impl Responder {
    if !security::is_logged_in(request) {
        return redirect_forbidden();
    }

    if !CONFIG.projects.contains(&form.project) {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("application/json; charset=utf-8")
            .body(r#"{"reason":"INVALID_PROJECT"}"#);
    }

    util::build(form.project.clone());

    HttpResponse::build(StatusCode::OK).finish()
}

#[get("/403")]
async fn forbidden() -> impl Responder {
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/403.html").replace("#BASE#", &CONFIG.root));
}

#[get("/500")]
async fn internal_server_error() -> impl Responder {
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/500.html").replace("#BASE#", &CONFIG.root));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Root at: {}", CONFIG.root);

    log4rs::init_file("log_config.yml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new().service(root).service(
            web::scope(CONFIG.root.as_str())
                .service(index)
                .service(index_slash)
                .service(login)
                .service(data)
                .service(do_login)
                .service(build)
                .service(forbidden)
                .service(internal_server_error)
                .service(fs::Files::new("/", "public").show_files_listing()),
        )
    })
    .bind(("127.0.0.1", 46000))?
    .run()
    .await
}
