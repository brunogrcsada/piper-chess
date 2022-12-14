FROM rust:1.61.0 AS builder

RUN useradd -ms /bin/bash piper
USER piper
ENV USER=piper

RUN mkdir /home/piper/backend
WORKDIR /home/piper/backend

RUN cargo init --vcs none
COPY server/Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
RUN rm target/release/deps/piper*

COPY server/src ./src
RUN cargo build --release

EXPOSE 2020

FROM node:latest AS frontend

RUN mkdir /frontend
WORKDIR /frontend

COPY ./package.json /frontend
COPY ./package-lock.json /frontend
RUN npm install
COPY . .

RUN npm run build
RUN npm install -g serve

COPY --from=0 /home/piper/backend/target/release/piper ./

CMD (serve -l tcp://0.0.0.0:3001 -s build &) && ./piper

EXPOSE 3001