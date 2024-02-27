use actix_web::{web, App, HttpResponse, HttpServer};
use lettre::Tokio1Executor;
mod v1;
pub mod config;

pub struct Context {
    pub lettre: lettre::AsyncSmtpTransport<Tokio1Executor>,
    pub config: config::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("HOST_ADDR").unwrap_or("0.0.0.0:3000".to_string());
    

    let ctx = Context {
        lettre: lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&std::env::var("SMTP_SERVER").expect("SMTP_SERVER is not set"))
            .unwrap()
            .build(),
        config: config::AppConfig::new(),
    };

    println!("Started on {}.", addr);

    let context = web::Data::new(ctx);
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(v1::scope())
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
