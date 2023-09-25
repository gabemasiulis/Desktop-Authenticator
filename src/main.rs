use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};
use std::sync::Arc;
use std::fs::File;
use std::io::Read;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

mod data;
mod db;
mod error;
mod handler;

const TEMPLATE_DIR: &str = "templates/"; 

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn read_template(template_file: &str) -> String {
    let mut template = String::new();
    match File::open(TEMPLATE_DIR.to_owned() + template_file) {
        Ok(mut file) => {
            file.read_to_string(&mut template).unwrap();
        },
        Err(error) => {
            println!("Error opening file {}: {}", template_file, error);
        },
    }
    template
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    
    let mut hb = Handlebars::new();
    // register the template
    let template_file = "template.html";
    hb.register_template_string(template_file, read_template(&template_file)).unwrap(); //.register_template_string("template.html", template)
        //.unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    let hbs_route = warp::path("hbs")
        .and(warp::get())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({"user" : "Warp"}),
        })
        .map(handlebars);

    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_todos_handler)
        .or(todo
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_todo_handler))
        .or(todo
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_todo_handler))
        .or(todo
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_todo_handler));

    let routes = health_route
        .or(todo_routes)
        .or(hbs_route)
        .or(warp::path("static").and(warp::fs::dir("static")))
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}