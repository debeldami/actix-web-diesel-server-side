use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
      "project_name": "Catdex",
      "cats": [
        {
          "name": "Rag Doll",
          "image_path": "/static/img/ragdoll.png"
        },
        {
          "name": "Short Hair",
          "image_path": "/static/img/shorthair.jpg"
        },
        {
          "name": "Tiffany",
          "image_path": "/static/img/tiffany.jpg"
        },
        {
            "name": "Tonkinese",
            "image_path": "/static/img/tonkinese.jpg"
        }
      ]
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
