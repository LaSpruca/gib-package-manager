FROM rust

# Update APT
RUN apt update

# Install NodeJS
RUN curl -fsSL https://deb.nodesource.com/setup_15.x | bash -

RUN apt install -y nodejs musl-dev libcurl4-openssl-dev libpq-dev
RUN npm i -g yarn

# Install tools
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /gibpm

COPY . .

# Build the server
RUN cargo build --bin gib-server --release

# Build the website
RUN cd gib-web && yarn install && cargo update && yarn build

EXPOSE 5000/tcp

CMD ["./start.sh"]
