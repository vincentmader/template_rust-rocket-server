#!/bin/sh

DB_DIR="../database"
DB="$DB_DIR/database.sqlite"

mkdir -p "$DB_DIR"
touch "$DB"
cat create_tables.sql | sqlite3 "$DB"
