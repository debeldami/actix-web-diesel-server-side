use self::models::*;
use actix_files::Files;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use dotenvy::dotenv;
use handlebars::Handlebars;
use std::env;
mod models;
mod schema;
use self::schema::cats::dsl::*;
use serde::Serialize;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    cats: Vec<Cat>,
}

#[get("/")]
async fn index(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let cats_data = web::block(move || {
        let mut conntemp = pool.get();

        let connection = conntemp.as_mut().unwrap();

        cats.limit(100).load::<Cat>(connection)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())
    .unwrap();

    let data: IndexTemplateData;

    match cats_data {
        Ok(data_info) => {
            println!("{:?}", &data_info);
            data = IndexTemplateData {
                project_name: "Catdex".to_string(),
                cats: data_info,
            };
        }

        Err(_) => {
            data = IndexTemplateData {
                project_name: "Catdex".to_string(),
                cats: vec![Cat {
                    id: 0,
                    name: "internal server error".to_string(),
                    image_path: "".to_string(),
                }],
            }
        }
    }

    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[get("/add")]
async fn add(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let body = hb.render("add", &{}).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "static"))
            .service(index)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
