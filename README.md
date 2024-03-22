# email-relay-api

## Description
This is a simple api server that listens for incoming requests and sends an email using the provided data.

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
    }
]
```

Response:
```json
[
    {"Ok": {}} // or {"Err": "error message"}
]
```
