FROM rust:1.72 as builder

ENV LC_CTYPE=ja_JP.utf8 \
  LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y -q \
  ca-certificates \
  locales \
  apt-transport-https\
  libssl-dev \
  libpq-dev \
  pkg-config \
  curl \
  build-essential \
  libdbus-1-dev \
  libsqlite3-dev \
  mariadb-client \
  git \
  wget \
  libc6-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make \
  && cargo install sqlx-cli --no-default-features --features mysql

WORKDIR /app-source

COPY . .
RUN cargo clean && cargo build --release

FROM debian:bullseye
RUN cp /etc/apt/sources.list /etc/apt/sources.list.bak
RUN echo "deb http://deb.debian.org/debian experimental main" >> /etc/apt/sources.list
RUN apt-get update && apt-get upgrade -y && apt-get -t experimental install -y libc6
RUN apt-get install git -y
RUN cp /etc/apt/sources.list.bak /etc/apt/sources.list && rm /etc/apt/sources.list.bak
RUN apt-get update && apt-get clean

RUN mkdir -p /app-product
WORKDIR /app-product
COPY --from=builder /app-source /app-product

# RUN ls /app-source

CMD ["/app-product/start.sh"]
# CMD ["/app-source/start.sh"]
