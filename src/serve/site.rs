use super::*;
use actix_web::{get, web::Data, HttpResponse};

#[get("/")]
pub(super) async fn index(data: Data<Cache>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(if data.doc.is_empty() {
            let doc = read_to_string(&data.project).unwrap_or_else(error_page);
            load(&doc, "/static/", data.reload).unwrap_or_else(error_page)
        } else {
            data.doc.clone()
        })
}

#[get("/help/")]
pub(super) async fn help_page(data: Data<Cache>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(data.help_doc.clone())
}
