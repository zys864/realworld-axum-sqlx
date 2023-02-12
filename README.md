# Developing !!! currently,only users api work
# RealWorld demo : Rust + Axum + Sqlx + Postgres

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.


# Cargo build
docker run -d --name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:14
> Change postgres database url in .env file 
>
> Run `cargo install sqlx-cli` if not install it
>
> run sqlx command to migration database model then build the project
```shell
    sqlx database create 
    sqlx migrate run
    cargo run --release
```



# Docker

> developing

