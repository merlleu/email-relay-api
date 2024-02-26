pub struct AppConfig {
    pub webhook_secret: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let webhook_secret = std::env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET is not set.");
        AppConfig {
            webhook_secret,
        }
    }
}