use actix_files as fs;
use actix_web::{Responder, HttpResponse, get, web};
use crate::utils::files;

#[get("/")]
async fn load_main_page() -> impl Responder {
    let file_data = match files::load_file("./wiki/main_page.md") {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Page not found"),
    };

    return HttpResponse::Ok().body(file_data);
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page)
        .service(fs::Files::new("/wiki", "./wiki").show_files_listing());
}
