# llm-rust

Experimentation with Rust stack to use main LLM models and vector stores

- crate: a Library/package
- [crates.io](https://crates.io/) create repository
- Cargo: dependency manager


## Docker commands

Commands to run the local files for docker.
````
docker-compose build
docker-compose up -d
````

Connect to a database to verify that the pgvector extension is installed.
````
docker exec -it postgres_pgvector psql -U admin -d mydatabase
````

Once you are logged run this sql commands.
````
CREATE EXTENSION vector;
\dx
````

