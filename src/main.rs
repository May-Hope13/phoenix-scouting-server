use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{State, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use db::PssDb;
use sled::Db;
use tera::{Context, Tera};
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;

#[macro_use]
extern crate log;
#[macro_use]
extern crate tokio;
extern crate axum;
extern crate reqwest;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate utoipa as utopia;

mod db;
mod frcapi;

use utopia::OpenApi;

pub(crate) const CURRENT_SEASON: &'static str = "2024";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Phoenix Scouting Server",
        description = include_str!("api_description.md"),
        contact(
            name = "J. R. \"Bob\" Dobbs",
            email = "bdobbs@waterga.me",
            url = "https://en.wikipedia.org/wiki/J._R._%22Bob%22_Dobbs",
        ),
    ),
    servers(
        (url = "http://localhost:6969/api/v1", description = "API v1"),
    ),
    paths(
        version,
        get_events,
    ),
    components(
        schemas(
            VersionRes,
            Event,
        ),
    ),
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let db = sled::open(".db").unwrap();

    let templates = Tera::new("views/**").unwrap();
    let r = Router::new()
        .route("/", get(home))
        .route("/auth", post(auth))
        .route("/manage", get(manage))
        .nest("/api/v1", Router::new().route("/version", get(version)).route("/:district/events", get(get_events)))
        .with_state((templates, db))
        .merge(SwaggerUi::new("/api").url("/api/openapi.json", ApiDoc::openapi()));

    let l = TcpListener::bind("0.0.0.0:6969".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    axum::serve(l, r).await.unwrap();
}

async fn home(State((mut tmpl, _)): State<(Tera, Db)>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("version", env!("CARGO_PKG_VERSION"));
    tmpl.full_reload().unwrap();
    Html(tmpl.render("home.html", &ctx).unwrap()).into_response()
}

async fn manage(State((mut tmpl, _)): State<(Tera, Db)>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("version", env!("CARGO_PKG_VERSION"));
    tmpl.full_reload().unwrap();
    Html(tmpl.render("manage.html", &ctx).unwrap()).into_response()
}

#[derive(Deserialize)]
struct Auth {
    u: String,
    p: String,
}

async fn auth(Form(Auth { u, p }): Form<Auth>) -> impl IntoResponse {
    println!("wrok: {u} / {p}");

    Response::builder()
        .status(200)
        .body(Body::empty())
        .unwrap()
        .into_response()
}

#[derive(Serialize, ToSchema)]
struct VersionRes {
    version: &'static str,
    current_season: &'static str,
}

/// Get PSS version
#[utopia::path(
    get,
    path = "/version",
    responses(
        (status = 200, body = VersionRes),
    ),
)]
async fn version() -> impl IntoResponse {
    Json(VersionRes {
        version: env!("CARGO_PKG_VERSION"),
        current_season: CURRENT_SEASON,
    })
}

#[derive(Serialize, ToSchema)]
struct Event {
    id: String,
    name: String,
}

/// Get events for a given district
#[utopia::path(
    get,
    path = "/{district}/events",
    responses(
        (status = 200, body = Vec<Event>),
    ),
)]
async fn get_events(State((mut _tmpl, db)): State<(Tera, Db)>, Path(district): Path<String>) -> impl IntoResponse {
    let events = db.district_events(district).map(|id| Event { id, name: String::from("a") }).collect::<Vec<Event>>();
    Json(events)
}
