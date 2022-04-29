use super::*;
use actix_web::{get, http::header::ContentType, web::Data, HttpResponse};

#[get("/")]
pub(super) async fn index(data: Data<Cache>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(if data.doc.is_empty() {
            let doc = read_to_string(&data.project)
                .unwrap_or_else(|e| Slides::single("Read project failed", e));
            load(&doc, "/static/", data.reload).unwrap_or_else(error_page)
        } else {
            data.doc.clone()
        })
}

#[get("/help/")]
pub(super) async fn help_page(data: Data<Cache>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(data.help_doc.clone())
}

pub(crate) async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(Slides::single("404 not found", "This page is not exist!"))
}
