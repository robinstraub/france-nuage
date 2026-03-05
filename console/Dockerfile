FROM node:22-alpine AS base

RUN apk add --no-cache curl

WORKDIR /node-sdk
COPY ./node-sdk/package.json ./node-sdk/package-lock.json ./
RUN npm ci
COPY ./node-sdk .
RUN npm run build

WORKDIR /app
COPY ./console/package.json ./
RUN npm install
COPY ./console .

FROM base AS development
WORKDIR /app
CMD ["npm", "run", "dev"]

FROM base AS build
WORKDIR /app
RUN npm run build

FROM nginx:alpine AS release
COPY --from=build /app/dist /usr/share/nginx/html
COPY ./console/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
