FROM node:lts-alpine AS client-builder
WORKDIR /build
COPY frontend/client/package.json frontend/client/package-lock.json ./
RUN npm install
COPY frontend/client .
RUN npm run build

FROM caddy:2.8.4-alpine AS proxy-runtime
COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=client-builder /build/dist /web/client
