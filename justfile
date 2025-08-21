METADATA_URL:="https://api.mangabaka.dev/v1/database/series.sqlite.tar.gz"
DB_DIR:="src-tauri/db"

default:
    npm run tauri dev

build:
    npm run tauri build

pull-meta:
    mkdir -p {{DB_DIR}}
    curl {{METADATA_URL}} --output {{DB_DIR}}/series.sqlite.tar.gz
    tar -xzvf {{DB_DIR}}/series.sqlite.tar.gz -C {{DB_DIR}}
    rm -f {{DB_DIR}}/series.sqlite.tar.gz
    grep -q "DATABASE_URL" src-tauri/.env 2>/dev/null || echo "DATABASE_URL=sqlite://db/series.sqlite" >> src-tauri/.env
    sqlite3 {{DB_DIR}}/series.sqlite "CREATE VIRTUAL TABLE IF NOT EXISTS series_fts USING fts5(title, native_title, romanized_title, secondary_titles_en, content='series',tokenize = \"unicode61 separators'0123456789'\");INSERT INTO series_fts(series_fts) VALUES('rebuild');"


