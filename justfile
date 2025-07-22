METADATA_URL:="https://api.mangabaka.dev/v1/database/series.sqlite.tar.gz"
DB_DIR:="src-tauri/db"

default:
    cargo-tauri dev

pull-meta:
    mkdir -p {{DB_DIR}}
    curl {{METADATA_URL}} --output {{DB_DIR}}/series.sqlite.tar.gz
    tar -xzvf {{DB_DIR}}/series.sqlite.tar.gz -C {{DB_DIR}}
    rm -f {{DB_DIR}}/series.sqlite.tar.gz
    grep -q "DATABASE_URL" src-tauri/.env 2>/dev/null || echo "DATABASE_URL=sqlite://db/series.sqlite" >> src-tauri/.env


