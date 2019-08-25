FROM rust

RUN apt-get update && apt-get upgrade -y && rustup component add rustfmt

VOLUME /code
WORKDIR /code

ENTRYPOINT ["cargo"]
