# k8s-autodeploy

## Description
This is a simple script to redeploy a kubernetes deployment. It is intended to be used in a CI/CD pipeline to redeploy a deployment after a new image has been built.
It uses Rancher API to do it after a webhook has been triggered.

## Environment variables
- WEBHOOK_SECRET: Secret to be used in the webhook

## Use with docker
- `docker pull merlleu/email-relay-api:latest`
- ```bash
docker run 
    -d
    -e WEBHOOK_SECRET="Bearer xyz" 
    merlleu/k8s-autodeploy:latest`