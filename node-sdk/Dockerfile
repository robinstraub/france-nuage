FROM node:22-alpine

RUN apk add --no-cache protoc

WORKDIR /app
COPY ./node-sdk/package.json ./node-sdk/package-lock.json ./
RUN npm ci
COPY ./node-sdk .
