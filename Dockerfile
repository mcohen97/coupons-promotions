FROM rust:1.39

WORKDIR /evaluations
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli

EXPOSE 8080

CMD [ "evaluations" ]








