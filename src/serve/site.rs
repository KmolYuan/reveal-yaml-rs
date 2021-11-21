use super::*;
use actix_web::{get, web::Data, HttpResponse};

#[get("/")]
pub(super) async fn index(data: Data<Cache>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(if data.doc.is_empty() {
            loader(&read_to_string(&data.project)?, "/static/", data.reload)?
        } else {
            data.doc.clone()
        }))
}

#[get("/help/")]
pub(super) async fn help_page(data: Data<Cache>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(data.help_doc.clone()))
}

#[get("/help/icon.png")]
pub(super) async fn icon() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("image/png").body(ICON))
}

#[get("/help/watermark.png")]
pub(super) async fn watermark() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("image/png").body(WATERMARK))
}
