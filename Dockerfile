FROM node:22-alpine AS deps
WORKDIR /website/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

FROM node:22-alpine AS frontend
WORKDIR /website/frontend
COPY --from=deps /website/frontend/node_modules ./node_modules
COPY frontend ./
RUN npm run build

FROM rust:1-bookworm AS backend
WORKDIR /website/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/backend target/release/deps/backend-*
COPY backend/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /website/backend
COPY --from=frontend /website/frontend/out /website/frontend/out
COPY --from=backend /website/backend/target/release/backend ./backend
EXPOSE 3000
CMD ["./backend"]
