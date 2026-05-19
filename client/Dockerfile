# Builder
FROM node:24-alpine AS builder
WORKDIR /app

ARG DOMAIN

COPY package*.json .
RUN npm ci

COPY . .
ENV PUBLIC_SERVER_HTTP_URL=https://$DOMAIN/server
ENV PUBLIC_SERVER_WS_URL=wss://$DOMAIN/server
RUN npx vite build

# Runner
FROM nginx:alpine AS runner
COPY --from=builder /app/build /var/www/aracardi
COPY nginx.conf /etc/nginx/nginx.conf
