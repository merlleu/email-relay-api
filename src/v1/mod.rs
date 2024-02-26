mod sendmail;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(sendmail::post_email)
}