FROM node:19-bullseye-slim

WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN npm i -g pnpm
RUN pnpm install

COPY . .

RUN pnpm run build


FROM node:19-bullseye-slim
WORKDIR /app
COPY --from=0 /app .
COPY . .

EXPOSE 3000
CMD ["node", "./build"]