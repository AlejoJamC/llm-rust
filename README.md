# llm-rust

Experimentation with Rust stack to use main LLM models and vector stores

- crate: a Library/package
- [crates.io](https://crates.io/) create repository
- Cargo: dependency manager

## How to test the system

1. Run local docker command for ollama
```
docker run -d -p 11434:11434 ollama/ollama
```
2. Execute the rust server
```
cargo run
```

3. Start the test using curl
```
curl -X POST http://localhost:8080/start-test/10
```



### Local Docker Commands for Ollama
```
docker run -d -p 11434:11434 ollama/ollama
```

### Docker files commands

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

