# Postfix-Relay-API 📨

## Description
This is a simple api server written in Rust that transfers emails sent via an HTTP request to a postfix instance.

## Why ?
Securing postfix is hard but having it inside a docker network alongside this relay is secure.
This also lets you have a simple REST API to call rather than using SMTP libraries.

## Environment variables
- WEBHOOK_SECRET: Secret to be used in the webhook

## Use with docker
```bash
docker run 
    -d
    -e SMTP_SERVER="postfix" 
    -e WEBHOOK_SECRET="Bearer xyz" 
    merlleu/email-relay-api:latest
```

## Example request
```
POST /v1/emails
Authorization: Bearer {WEBHOOK_SECRET}
Content-Type: application/json
```

```json
[
    {
        "from": {"name": "merlleu", "email": "merlleu@example.org"},
        "to": {"name": "some1", "email": "some1@example.org"},
        "reply_to": {"name": "merlleu", "email": "merlleu@example.org"},
        "subject": "Hello",
        "text_body": "Hello, this is a test email",
        "html_body": "<h1>Hello</h1><p>this is a test email</p>"
    },
    {
        "from": {"name": "merlleu", "email": "merlleu@example.org"},
        "to": {"name": "some2", "email": "some2@example.org"},
        "reply_to": {"name": "merlleu", "email": "merlleu@example.org"},
        "subject": "Hello",
        "text_body": "Hello, this is a test email #2",
        "html_body": "<h1>Hello</h1><p>this is a test email #2</p>"
    }
]
```

Response:
```json
[
    {"Ok": {}},
    {"Err": "error message"}
]
```
