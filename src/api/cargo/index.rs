use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse, Responder};
use std::{fs::File, io::Read};

#[get("/index/config.json")]
pub async fn config() -> impl Responder {
    let mut config = match File::open("assets/config.json") {
        Ok(file) => file,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error while receive config file {}", err))
        }
    };

    let mut config_content = String::new();

    match config.read_to_string(&mut config_content) {
        Ok(_) => (),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error while receive config file {}", err))
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(config_content)
}

#[get("/index/{name:.*}")]
pub async fn index_files(name: web::Path<String>) -> impl Responder {
    let mut file = File::open(format!("assets/{}", name)).unwrap();

    let mut config_content = String::new();

    file.read_to_string(&mut config_content).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(config_content)
}
