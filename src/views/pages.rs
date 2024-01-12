use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct WikiPageTemplate {
    pub title: String,
    pub data: String,
}