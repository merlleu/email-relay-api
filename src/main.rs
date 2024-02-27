use actix_web::{web, App, HttpResponse, HttpServer};
use lettre::Tokio1Executor;
pub mod config;
mod v1;

pub struct Context {
    pub lettre: lettre::AsyncSmtpTransport<Tokio1Executor>,
    pub config: config::AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("HOST_ADDR").unwrap_or("0.0.0.0:3000".to_string());

    let smtp_server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER is not set");
    let smtp_port = std::env::var("SMTP_PORT")
        .expect("SMTP_PORT is not set")
        .parse::<u16>()
        .expect("SMTP_PORT is not a valid number");
    let ctx = Context {
        lettre: lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
            .unwrap()
            .port(smtp_port)
            .tls(lettre::transport::smtp::client::Tls::None) // no need for tls in same-machine communication
            .build(),
        config: config::AppConfig::new(),
    };

    println!("Started on {}.", addr);

    let context = web::Data::new(ctx);
    HttpServer::new(move || App::new().app_data(context.clone()).service(v1::scope()))
        .bind(addr)?
        .run()
        .await?;

    Ok(())
}
