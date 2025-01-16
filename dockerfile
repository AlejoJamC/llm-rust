# Use the official PostgreSQL image as the base
FROM postgres:15

# Install pgvector
RUN apt-get update && apt-get install -y \
    postgresql-server-dev-15 \
    make gcc && \
    rm -rf /var/lib/apt/lists/* && \
    git clone --depth 1 https://github.com/pgvector/pgvector.git && \
    cd pgvector && \
    make && make install && \
    cd .. && rm -rf pgvector

# Expose the default PostgreSQL port
EXPOSE 5432
