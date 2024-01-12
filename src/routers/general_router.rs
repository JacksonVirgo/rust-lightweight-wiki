use actix_files as fs;
use actix_web::{Responder, HttpResponse, get, web};
use askama::Template;

use crate::utils::{files, formatting};
use crate::views::pages::WikiPageTemplate;

#[get("/")]
async fn load_main_page() -> impl Responder {
    let file_data = match files::load_file("./wiki/main_page.md") {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Page not found"),
    };

    let formatted_file = formatting::parse_markdown(&file_data);
    let page = WikiPageTemplate {
        title: "Test".to_string(),
        data: formatted_file.to_string()
    };

    let rendered_page = match page.render() {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Cannot load page")
    };

    return HttpResponse::Ok().content_type("text/html").body(rendered_page);
}

pub fn config_general_routes(config: &mut web::ServiceConfig) {
    config
        .service(load_main_page)
        .service(fs::Files::new("/public", "./public").show_files_listing())
        .service(fs::Files::new("/wiki", "./wiki").show_files_listing());
}
