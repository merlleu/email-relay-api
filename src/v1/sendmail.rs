use std::collections::BTreeMap;

use actix_web::{
    post,
    web::{Data, Json, JsonBody},
    HttpRequest, HttpResponse,
};
use lettre::{
    message::{MultiPart, SinglePart},
    Address, AsyncSmtpTransport, AsyncTransport,
};
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize)]
pub struct PostRedeployBody {
    pub image: String,
}

#[post("/emails")]
pub async fn post_email(
    req: HttpRequest,
    body: Json<Vec<Email>>,
    ctx: Data<Context>,
) -> HttpResponse {
    if !check_auth(&req, &ctx) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let body = body.into_inner();

    let futs = body
        .into_iter()
        .map(|email| send_email(email, &ctx))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(futs)
        .await
        .into_iter()
        .map(|r| match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(results)
}

fn check_auth(req: &HttpRequest, ctx: &Context) -> bool {
    if req.headers().get("Authorization").is_none() {
        return false;
    }

    let auth = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    ctx.config.webhook_secret == auth
}

async fn send_email(email: Email, ctx: &Context) -> Result<(), anyhow::Error> {
    let email = email.convert()?;
    ctx.lettre.send(email).await?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Email {
    pub from: MailBox,
    pub to: Vec<MailBox>,
    pub reply_to: MailBox,
    pub subject: String,
    pub text_body: String,
    pub html_body: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MailBox {
    pub name: Option<String>,
    pub email: String,
}

impl MailBox {
    pub fn into_lettre_mailbox(&self) -> Result<lettre::message::Mailbox, anyhow::Error> {
        Ok(lettre::message::Mailbox::new(
            self.name.clone(),
            self.email.parse()?,
        ))
    }
}

impl Email {
    pub fn convert(self) -> Result<lettre::Message, anyhow::Error> {
        let mut email = lettre::Message::builder()
            .from(self.from.into_lettre_mailbox()?)
            .reply_to(self.reply_to.into_lettre_mailbox()?)
            .subject(self.subject);

        for i in self.to {
            email = email.to(i.into_lettre_mailbox()?);
        }

        let mut body = MultiPart::alternative().singlepart(SinglePart::plain(self.text_body));
        body = match self.html_body {
            Some(html) => body.singlepart(SinglePart::html(html)),
            None => body,
        };

        let email = email.multipart(body)?;

        Ok(email)
    }
}
