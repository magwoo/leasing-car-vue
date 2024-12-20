FROM node:lts-alpine AS admin-builder
WORKDIR /build
COPY frontend/admin/package.json frontend/admin/package-lock.json ./
RUN npm install
COPY frontend/admin .
RUN npm run build

FROM node:lts-alpine AS client-builder
WORKDIR /build
COPY frontend/client/package.json frontend/client/package-lock.json ./
RUN npm install
COPY frontend/client .
RUN npm run build

FROM caddy:2.8.4-alpine AS runner
COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=admin-builder /build/dist /web/admin
COPY --from=client-builder /build/dist /web/client
