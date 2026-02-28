FROM node:22-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /node-sdk
COPY ./node-sdk/package.json ./node-sdk/package-lock.json ./
RUN npm ci
COPY ./node-sdk .
RUN npm run build

WORKDIR /app
COPY ./system-tests/package.json ./system-tests/package-lock.json ./
RUN npm ci
RUN npx playwright install --with-deps chromium
COPY ./system-tests .

CMD ["npm", "run", "test:ui"]
