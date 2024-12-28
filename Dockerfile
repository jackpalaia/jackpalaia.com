# Build NextJS frontend
FROM node:18-alpine AS base

# Install dependencies only when needed
FROM base AS deps
WORKDIR /website/frontend
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk add --no-cache libc6-compat

COPY ./frontend/package.json ./frontend/package-lock.json ./
RUN npm ci

# Rebuild the source code only when needed
FROM base AS builder
WORKDIR /website/frontend
COPY --from=deps /website/frontend/node_modules ./node_modules
COPY ./frontend ./
RUN npm run build

# Run Rust backend server
FROM rust

WORKDIR /website

COPY --from=builder /website/frontend/out ./frontend/out
COPY ./backend ./backend


WORKDIR /website/backend

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/backend"]
